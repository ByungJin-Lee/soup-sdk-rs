use serde::Deserialize;

// --- LiveDetail 관련 구조체들 ---

#[derive(Debug, Clone, Deserialize)]
pub struct LiveDetail {
    #[serde(rename = "CHANNEL")]
    pub channel: ChannelInfo,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ChannelInfo {
    #[serde(rename = "RESULT")]
    pub result: i32, // 1이면 방송 중, 0이면 방송 중 아님
    #[serde(rename = "CHDOMAIN")]
    pub ch_domain: String,
    #[serde(rename = "CHPT")]
    pub ch_pt: u32,
    #[serde(rename = "CHATNO")]
    pub chat_no: String,
}

impl LiveDetail {
    // 방송 중인지 여부를 쉽게 확인할 수 있는 헬퍼 메서드
    pub fn is_streaming(&self) -> bool {
        self.channel.result == 1
    }
}
