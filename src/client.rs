use crate::constants::PLAYER_LIVE_API_URL;
use crate::error::{Error, Result};
use crate::models::{LiveDetail, LiveDetailToCheck};
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

        let live_detail_to_check =
            serde_json::from_slice::<LiveDetailToCheck>(&bytes).map_err(|e| Error::SerdeJson(e))?;

        if !live_detail_to_check.is_streaming() {
            return Ok((false, None));
        }

        let live_detail =
            serde_json::from_slice::<LiveDetail>(&bytes).map_err(|e| Error::SerdeJson(e))?;

        return Ok((true, Some(live_detail)));
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
