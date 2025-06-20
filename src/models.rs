use serde::Deserialize;
use serde_this_or_that::{as_bool, as_u64};
// --- LiveDetail 관련 구조체들 ---

#[derive(Debug, Clone, Deserialize)]
pub struct LiveDetail {
    #[serde(rename = "CHANNEL")]
    pub channel: ChannelInfo,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LiveDetailToCheck {
    #[serde(rename = "CHANNEL")]
    pub channel: ChannelInfoToCheck,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ChannelInfoToCheck {
    #[serde(rename = "RESULT")]
    pub result: i32, // 1이면 방송 중, 0이면 방송 중 아님
}

#[derive(Debug, Clone, Deserialize)]
pub struct ChannelInfo {
    #[serde(rename = "RESULT")]
    pub result: i32, // 1이면 방송 중, 0이면 방송 중 아님
    #[serde(rename = "CHDOMAIN")]
    pub ch_domain: String,
    #[serde(rename = "CHPT", deserialize_with = "as_u64")]
    pub ch_pt: u64,
    #[serde(rename = "CHATNO")]
    pub chat_no: String,
    #[serde(rename = "BJNICK")]
    pub bj_nick: String,
    #[serde(rename = "TITLE")]
    pub title: String,
    #[serde(rename = "CATEGORY_TAGS")]
    pub categories: Vec<String>,
}

impl LiveDetailToCheck {
    // 방송 중인지 여부를 쉽게 확인할 수 있는 헬퍼 메서드
    pub fn is_streaming(&self) -> bool {
        self.channel.result == 1
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct StationResponse {
    #[serde(rename = "station")]
    pub station: StationState,
    #[serde(rename = "broad")]
    pub broad: BroadState,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StationState {
    #[serde(rename = "broad_start")]
    pub broad_start: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BroadState {
    #[serde(rename = "is_password", deserialize_with = "as_bool")]
    pub is_password: bool,
    #[serde(rename = "current_sum_viewer", deserialize_with = "as_u64")]
    pub viewer_count: u64,
    #[serde(rename = "broad_title")]
    pub title: String,
}
