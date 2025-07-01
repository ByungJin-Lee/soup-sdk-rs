use crate::constants::{EMOTICON_API_URL, PLAYER_LIVE_API_URL};
use crate::error::{Error, Result};
use crate::models::{
    LiveDetail, LiveDetailToCheck, RawLiveDetail, RawStation, SignatureEmoticonData,
    SignatureEmoticonResponse, Station,
};
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
}
