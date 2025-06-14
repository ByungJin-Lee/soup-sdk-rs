use serde::Serialize;

pub struct UserFlags {
    pub follow: u32,
    pub combined: u32,
}

#[derive(Debug, Serialize, Clone)]
pub struct UserSubscribe {
    pub acc: u32,
    pub current: u32,
}

#[derive(Debug, Serialize, Clone)]
pub struct User {
    pub id: String,
    pub label: String,
    /// 0인 경우 팔로우 아님, 1-2인 경우 티어에 따라 다름
    pub follow: u8,
    pub is_bj: bool,
    pub is_manager: bool,
    pub is_top_fan: bool,
    pub is_fan: bool,
    pub is_supporter: bool,
    pub is_subscriber: bool,
    // 구독
    pub subscribe: UserSubscribe,
}
