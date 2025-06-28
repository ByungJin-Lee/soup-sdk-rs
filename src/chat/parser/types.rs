use serde::Deserialize;
use serde_this_or_that::{as_bool, as_u64};

pub struct UserFlags {
    pub follow: u32,
    pub combined: u32,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AbstractMissionData {
    #[serde(rename = "type")]
    pub message_type: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MissionGiftPayload {
    #[serde(rename = "user_id")]
    pub user_id: String,
    #[serde(rename = "user_nick")]
    pub label: String,
    #[serde(rename = "gift_count", deserialize_with = "as_u64")]
    pub amount: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MissionGiftTotalPayload {
    #[serde(rename = "settle_count", deserialize_with = "as_u64")]
    pub amount: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BattleMissionResultPayload {
    #[serde(rename = "draw", deserialize_with = "as_bool")]
    pub draw: bool,
    #[serde(rename = "winner")]
    pub winner: String,
    #[serde(rename = "title")]
    pub title: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ChallengeMissionResultPayload {
    #[serde(rename = "missionStatus")]
    pub status: String,
    #[serde(rename = "title")]
    pub title: String,
}
