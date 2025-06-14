use crate::constants::PLAYER_LIVE_API_URL;
use crate::error::{Error, Result};
use crate::models::LiveDetail;
use reqwest::Client;

#[derive(Debug)]
pub struct SoopHttpClient {
    client: Client,
}

impl SoopHttpClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    /// 스트리머 ID로 방송 상세 정보를 가져옵니다.
    pub async fn get_live_details(&self, streamer_id: &str) -> Result<LiveDetail> {
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

        // JSON 응답을 LiveDetail 구조체로 파싱합니다.
        let live_detail = response.json::<LiveDetail>().await.map_err(Error::Json)?;

        Ok(live_detail)
    }
}
