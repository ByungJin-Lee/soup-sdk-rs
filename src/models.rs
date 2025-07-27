use serde::{Deserialize, Serialize};
use serde_this_or_that::{as_bool, as_u64};
use crate::error::Result;
// --- LiveDetail 관련 구조체들 ---

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RawLiveDetail {
    #[serde(rename = "CHANNEL")]
    pub channel: ChannelInfo,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveDetail {
    pub is_live: bool,
    pub ch_domain: String,
    pub ch_pt: u64,
    pub ch_no: String,
    pub streamer_nick: String,
    pub title: String,
    pub categories: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LiveDetailToCheck {
    #[serde(rename = "CHANNEL")]
    pub channel: ChannelInfoToCheck,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ChannelInfoToCheck {
    #[serde(rename = "RESULT")]
    pub result: i32, // 1이면 방송 중, 0이면 방송 중 아님
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
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

impl RawVODResponse {
    pub fn into_vods(self) -> Vec<VOD> {
        self.data
            .into_iter()
            .filter(|vod| vod.auth_no == 101)
            .map(|vod| VOD {
                id: vod.title_no,
                title: vod.title_name,
                thumbnail_url: format!("https:{}", vod.ucc.thumb),
                duration: vod.ucc.total_file_duration,
            })
            .collect()
    }
}

impl RawVODDetailResponse {
    pub fn into_vod_detail(self) -> Result<VODDetail> {
        if self.result != 1 {
            return Err(crate::error::Error::ApiError("VOD not found".to_string()));
        }
        
        let data = self.data.ok_or_else(|| crate::error::Error::ApiError("VOD data not available".to_string()))?;
        
        Ok(VODDetail {
            id: data.title_no.to_string(),
            title: data.full_title,
            channel_id: data.bj_id,
            broad_start: data.broad_start,
            files: data.files.into_iter().map(|file| VODFile {
                id: file.idx,
                order: file.file_order,
                file_key: file.file_info_key,
                file_start: file.file_start,
                chat: file.chat,
                duration: file.duration,
            }).collect(),
        })
    }
}

impl LiveDetailToCheck {
    // 방송 중인지 여부를 쉽게 확인할 수 있는 헬퍼 메서드
    pub fn is_streaming(&self) -> bool {
        self.channel.result == 1
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RawStation {
    #[serde(rename = "station")]
    pub station: StationState,
    #[serde(rename = "broad")]
    pub broad: BroadState,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Station {
    pub broad_start: String,
    pub is_password: bool,
    pub viewer_count: u64,
    pub title: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StationState {
    #[serde(rename = "broad_start")]
    pub broad_start: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
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
#[serde(rename_all = "camelCase")]
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
    #[serde(rename(serialize = "pcImg", deserialize = "pc_img"))]
    pub pc_img: String,
    #[serde(rename(serialize = "mobileImg", deserialize = "mobile_img"))]
    pub mobile_img: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct VOD {
    pub id: u64,
    pub title: String,
    pub thumbnail_url: String,
    pub duration: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct VODDetail {
    pub id: String,
    pub title: String,
    pub channel_id: String,
    pub broad_start: String,
    pub files: Vec<VODFile>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct VODFile {
    pub id: u64,
    pub order: u32,
    pub file_key: String,
    pub file_start: String,
    pub chat: String,
    pub duration: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RawVODResponse {
    data: Vec<RawVOD>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RawVOD {
    title_no: u64,
    title_name: String,
    auth_no: u32,
    ucc: RawVODUcc,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RawVODUcc {
    thumb: String,
    total_file_duration: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RawVODDetailResponse {
    result: i32,
    data: Option<RawVODDetailData>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RawVODDetailData {
    title_no: u64,
    full_title: String,
    bj_id: String,
    broad_start: String,
    files: Vec<RawVODDetailFile>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RawVODDetailFile {
    idx: u64,
    file_order: u32,
    file_info_key: String,
    file_start: String,
    chat: String,
    duration: u64,
}
