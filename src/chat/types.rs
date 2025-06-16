use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct UserSubscribe {
    pub acc: u32,
    pub current: u32,
}

#[derive(Debug, Serialize, Clone)]
pub struct UserStatus {
    /// 0인 경우 팔로우 아님, 1-2인 경우 티어에 따라 다름
    pub follow: u8,
    pub is_bj: bool,
    pub is_manager: bool,
    pub is_top_fan: bool,
    pub is_fan: bool,
    pub is_supporter: bool,
}

#[derive(Debug, Serialize, Clone)]
pub struct User {
    pub id: String,
    pub label: String,
    // 사용자 상태
    pub status: UserStatus,
    // 구독
    pub subscribe: Option<UserSubscribe>,
}

#[derive(Debug, Serialize, Clone)]
pub enum ChatType {
    Manager,
    Common,
    Emoticon,
}

#[derive(Debug, Serialize, Clone)]
pub struct Emoticon {
    pub id: String,
    pub number: String,
    pub ext: String,
    pub version: String,
}

#[derive(Debug, Serialize, Clone)]
pub enum DonationType {
    Balloon,
    ADBalloon,
    VODBalloon,
}

#[derive(Debug, Serialize, Clone)]
pub enum MissionType {
    Battle,
    Challenge,
}

#[derive(Debug)]
pub enum MissionParser {
    Mission,
    MissionTotal,
    BattleNotice,
    ChallengeNotice,
}
