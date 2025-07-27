use crate::chat::events::Event;
use crate::constants::{EMOTICON_API_URL, PLAYER_LIVE_API_URL};
use crate::error::{Error, Result};
use crate::models::{
    LiveDetail, LiveDetailToCheck, RawLiveDetail, RawStation, RawVODDetailResponse, RawVODResponse,
    SignatureEmoticonData, SignatureEmoticonResponse, Station, VOD, VODDetail, VODFile,
};
use crate::vod_chat_parser::parse_vod_chat_xml_with_start_time;
use reqwest::{Client, Response};

#[derive(Debug)]
pub struct SoopHttpClient {
    client: Client,
}

/// (is_live_detail, live_detail)
type LiveDetailState = (bool, Option<LiveDetail>);

impl SoopHttpClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    /// 스트리머 ID로 방송 상세 정보를 가져옵니다.
    pub async fn get_live_detail_state(&self, streamer_id: &str) -> Result<LiveDetailState> {
        let resp = self.fetch_live_detail_response(streamer_id).await?;

        let bytes = resp.bytes().await.map_err(|e| Error::ResponseJson(e))?;

        // bytes를 공유해서 두 번의 파싱을 수행하되, bytes 복사는 피합니다
        let live_detail_to_check =
            serde_json::from_slice::<LiveDetailToCheck>(&bytes).map_err(|e| Error::SerdeJson(e))?;

        if !live_detail_to_check.is_streaming() {
            return Ok((false, None));
        }

        // 방송 중인 경우에만 전체 JSON을 파싱합니다
        let live_detail =
            serde_json::from_slice::<RawLiveDetail>(&bytes).map_err(|e| Error::SerdeJson(e))?;

        return Ok((
            true,
            Some(LiveDetail {
                is_live: live_detail_to_check.is_streaming(),
                ch_domain: live_detail.channel.ch_domain,
                ch_pt: live_detail.channel.ch_pt,
                ch_no: live_detail.channel.chat_no,
                streamer_nick: live_detail.channel.bj_nick,
                title: live_detail.channel.title,
                categories: live_detail.channel.categories,
            }),
        ));
    }

    pub async fn get_station(&self, streamer_id: &str) -> Result<Station> {
        let request = self
            .client
            .get(format!(
                "https://chapi.sooplive.co.kr/api/{}/station",
                streamer_id
            )) // URL 쿼리 파라미터 추가
            .header("User-Agent", "Mozilla/5.0 (compatible; SoopClient/1.0)"); // User-Agent 헤더 설정

        let response = request.send().await?;

        if !response.status().is_success() {
            return Err(Error::Request(response.error_for_status().unwrap_err()));
        }

        let station_response = response.json::<RawStation>().await?;

        return Ok(Station {
            broad_start: station_response.station.broad_start,
            is_password: station_response.broad.is_password,
            viewer_count: station_response.broad.viewer_count,
            title: station_response.broad.title,
        });
    }

    pub async fn get_signature_emoticon(&self, streamer_id: &str) -> Result<SignatureEmoticonData> {
        // x-www-form-urlencoded 형식의 본문을 만듭니다.
        let params = [("szBjId", streamer_id), ("work", "list"), ("v", "tier")];

        let request = self
            .client
            .post(EMOTICON_API_URL)
            .header("Content-Type", "application/x-www-form-urlencoded") // 헤더 설정
            .header("User-Agent", "Mozilla/5.0 (compatible; SoopClient/1.0)") // User-Agent 헤더 설정
            .form(&params); // form-urlencoded 본문 추가

        let response = request.send().await?;

        if !response.status().is_success() {
            return Err(Error::Request(response.error_for_status().unwrap_err()));
        }

        let emoticon_response = response.json::<SignatureEmoticonResponse>().await?;

        return Ok(emoticon_response.data);
    }

    /// 스트리머 ID로 방송 상세 정보 response를 가져옵니다.
    async fn fetch_live_detail_response(&self, streamer_id: &str) -> Result<Response> {
        // x-www-form-urlencoded 형식의 본문을 만듭니다.
        let params = [("bid", streamer_id)];

        let request = self
            .client
            .post(PLAYER_LIVE_API_URL)
            .query(&[("bjid", streamer_id)]) // URL 쿼리 파라미터 추가
            .header("Content-Type", "application/x-www-form-urlencoded") // 헤더 설정
            .header("User-Agent", "Mozilla/5.0 (compatible; SoopClient/1.0)") // User-Agent 헤더 설정
            .form(&params); // form-urlencoded 본문 추가

        let response = request.send().await?;

        if !response.status().is_success() {
            return Err(Error::Request(response.error_for_status().unwrap_err()));
        }

        Ok(response)
    }

    pub async fn get_vod_list(&self, streamer_id: &str, page: u32) -> Result<Vec<VOD>> {
        let url = format!(
            "https://chapi.sooplive.co.kr/api/{}/vods/all?page={}&per_page=60&orderby=reg_date&field=title%2Ccontents&created=false",
            streamer_id, page
        );

        let request = self
            .client
            .get(&url)
            .header("User-Agent", "Mozilla/5.0 (compatible; SoopClient/1.0)");

        let response = request.send().await?;

        if !response.status().is_success() {
            return Err(Error::Request(response.error_for_status().unwrap_err()));
        }

        let vod_response = response.json::<RawVODResponse>().await?;
        Ok(vod_response.into_vods())
    }

    pub async fn get_vod_detail(&self, vod_id: u64) -> Result<VODDetail> {
        let params = [("nTitleNo", vod_id.to_string())];

        let request = self
            .client
            .post("https://api.m.sooplive.co.kr/station/video/a/view")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .header("User-Agent", "Mozilla/5.0 (compatible; SoopClient/1.0)")
            .form(&params);

        let response = request.send().await?;

        if !response.status().is_success() {
            return Err(Error::Request(response.error_for_status().unwrap_err()));
        }

        let vod_detail_response = response.json::<RawVODDetailResponse>().await?;
        vod_detail_response.into_vod_detail()
    }

    pub async fn get_vod_chat(&self, chat_url: &str, start_time: u64) -> Result<String> {
        let url = format!("{}&startTime={}", chat_url, start_time);

        let request = self
            .client
            .get(&url)
            .header("User-Agent", "Mozilla/5.0 (compatible; SoopClient/1.0)");

        let response = request.send().await?;

        if !response.status().is_success() {
            return Err(Error::Request(response.error_for_status().unwrap_err()));
        }

        let xml_content = response.text().await?;
        Ok(xml_content)
    }

    async fn get_file_chat_events(
        &self,
        file: &VODFile,
        broad_start: &str,
        chunk_size_seconds: u64,
    ) -> Result<Vec<Event>> {
        let duration_seconds = file.duration / 1_000_000; // 마이크로초를 초로 변환
        let mut all_events = Vec::new();
        let mut current_time = 0;

        while current_time < duration_seconds {
            match self.get_vod_chat(&file.chat, current_time).await {
                Ok(xml_content) => {
                    if !xml_content.trim().is_empty() {
                        match parse_vod_chat_xml_with_start_time(&xml_content, Some(broad_start)) {
                            Ok(mut events) => {
                                all_events.append(&mut events);
                            }
                            Err(e) => {
                                eprintln!("XML 파싱 오류 (시간 {}초): {}", current_time, e);
                            }
                        }
                    }
                }
                Err(e) => {
                    eprintln!("채팅 조회 오류 (시간 {}초): {}", current_time, e);
                }
            }
            current_time += chunk_size_seconds;
        }

        Ok(all_events)
    }

    pub async fn get_full_vod_chat(&self, vod_id: u64) -> Result<Vec<Event>> {
        let vod_detail = self.get_vod_detail(vod_id).await?;
        let mut all_events = Vec::new();

        println!(
            "VOD '{}' 전체 채팅 수집 시작... ({} 파일)",
            vod_detail.title,
            vod_detail.files.len()
        );

        for (i, file) in vod_detail.files.iter().enumerate() {
            println!(
                "파일 {}/{} 처리 중... (재생시간: {}분)",
                i + 1,
                vod_detail.files.len(),
                file.duration / 1_000_000 / 60
            );

            let mut file_events = self
                .get_file_chat_events(file, &vod_detail.broad_start, 300)
                .await?; // 5분 간격
            all_events.append(&mut file_events);
        }

        println!("총 {}개 이벤트 수집 완료", all_events.len());
        Ok(all_events)
    }
}
