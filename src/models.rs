use serde::{Deserialize, Serialize};
use serde_this_or_that::{as_bool, as_u64};
// --- LiveDetail 관련 구조체들 ---

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LiveDetail {
    #[serde(rename = "CHANNEL", flatten)]
    pub channel: ChannelInfo,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LiveDetailToCheck {
    #[serde(rename = "CHANNEL", flatten)]
    pub channel: ChannelInfoToCheck,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ChannelInfoToCheck {
    #[serde(rename = "RESULT")]
    pub result: i32, // 1이면 방송 중, 0이면 방송 중 아님
}

#[derive(Debug, Clone, Deserialize, Serialize)]
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

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StationResponse {
    #[serde(rename = "station", flatten)]
    pub station: StationState,
    #[serde(rename = "broad", flatten)]
    pub broad: BroadState,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StationState {
    #[serde(rename = "broad_start")]
    pub broad_start: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BroadState {
    #[serde(rename = "is_password", deserialize_with = "as_bool")]
    pub is_password: bool,
    #[serde(rename = "current_sum_viewer", deserialize_with = "as_u64")]
    pub viewer_count: u64,
    #[serde(rename = "broad_title")]
    pub title: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SignatureEmoticonResponse {
    #[serde(rename = "result")]
    pub result: i32,
    #[serde(rename = "data")]
    pub data: SignatureEmoticonData,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SignatureEmoticonData {
    #[serde(rename = "tier1")]
    pub tier_1: Vec<Emoticon>,
    #[serde(rename = "tier2")]
    pub tier_2: Vec<Emoticon>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Emoticon {
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "pc_img")]
    pub pc_img: String,
    #[serde(rename = "mobile_img")]
    pub mobile_img: String,
}
