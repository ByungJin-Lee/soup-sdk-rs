use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::chat::{constants::message_codes::MessageCode, types::User};

// --- 채팅 이벤트 ---
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", content = "payload")]
pub enum Event {
    // --- 생명 주기 관련 이벤트 ---
    /// 최초 연결 성공 시 발생
    Connected,
    /// 연결이 끊어져 재연결 성공 시 발생
    Restored(RestoredEvent),
    /// 재연결을 시도하고 있음을 알림
    Reconnecting(ReconnectingEvent),
    /// 연결이 완전히 종료되었을 때 발생
    Disconnected,

    // --- 채팅 관련 이벤트 ---
    /// 일반 채팅 메시지가 수신되었을 때 발생합니다.
    Chat(ChatEvent),
    /// 후원 (텍스트, 영상, 애드벌룬)이 발생했을 때.
    Donation(DonationEvent),
    /// 구독이 발생했을 때.
    Subscribe(SubscribeEvent),
    /// 새로운 시청자가 채팅방에 입장했을 때.
    Enter(UserEvent),
    /// 시청자가 채팅방에서 퇴장했을 때.
    Exit(UserEvent),
    /// 스트리머나 매니저가 보낸 공지사항 메시지.
    Notification(NotificationEvent),
    /// 현재 시청자 수가 업데이트되었을 때.
    Join(SimplifiedUserEvent),
    /// 알 수 없는 이벤트 타입
    Unknown(MessageCode),
    /// 직접 처리
    Raw(Vec<u8>), // 원시 데이터로 처리할 수 있는 이벤트
}

#[derive(Debug, Clone, Serialize)]
pub struct EventMeta {
    /// 이벤트가 라이브러리에서 생성된 시간
    pub received_time: DateTime<Utc>,
}

// --- 생명 주기 관련 이벤트 ---
#[derive(Debug, Clone, Serialize)]
pub struct ConnectedEvent {
    #[serde(flatten)]
    pub meta: EventMeta,
}

#[derive(Debug, Clone, Serialize)]
pub struct ReconnectingEvent {
    #[serde(flatten)]
    pub meta: EventMeta,
    pub attempt: u32,
    pub wait_seconds: u64,
}

#[derive(Debug, Clone, Serialize)]
pub struct RestoredEvent {
    #[serde(flatten)]
    pub meta: EventMeta,
    pub restored_at: DateTime<Utc>,
}

// --- 채팅 관련 이벤트 ---

#[derive(Debug, Clone, Serialize)]
pub struct ChatEvent {
    #[serde(flatten)]
    pub meta: EventMeta,
    pub comment: String,
    pub user: User,
}

#[derive(Debug, Clone, Serialize)]
pub struct DonationEvent {
    #[serde(flatten)]
    pub meta: EventMeta,
    pub from: String,
    pub from_username: String,
    pub to: String,
    pub amount: u64,                   // 후원 금액
    pub fan_club_ordinal: Option<u32>, // 팬클럽 순번
    pub donation_type: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ViewerEvent {
    #[serde(flatten)]
    pub meta: EventMeta,
    pub users: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SubscribeEvent {
    #[serde(flatten)]
    pub meta: EventMeta,
    pub user_id: String,
    pub username: String,
    pub fan_club_ordinal: Option<u32>, // 팬클럽 순번
}

#[derive(Debug, Clone, Serialize)]
pub struct NotificationEvent {
    #[serde(flatten)]
    pub meta: EventMeta,
    pub message: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ViewerCountEvent {
    #[serde(flatten)]
    pub meta: EventMeta,
    pub count: u32, // 현재 시청자 수
}

#[derive(Debug, Clone, Serialize)]
pub struct UserEvent {
    #[serde(flatten)]
    pub meta: EventMeta,
    #[serde(flatten)]
    pub user: User,
}

#[derive(Debug, Clone, Serialize)]
pub struct SimplifiedUserEvent {
    #[serde(flatten)]
    pub meta: EventMeta,
    pub user_id: String,
}
