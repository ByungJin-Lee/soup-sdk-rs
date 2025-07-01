use chrono::{DateTime, Utc};
use serde::{Serialize, Serializer};
use std::sync::Arc;

fn serialize_arc_bytes<S>(data: &Arc<[u8]>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_bytes(data)
}

use crate::chat::{
    constants::message_codes::MessageCode,
    types::{ChatType, DonationType, Emoticon, MissionType, User},
};

// --- 채팅 이벤트 ---
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", content = "payload")]
pub enum Event {
    // --- 생명 주기 관련 이벤트 ---
    /// 최초 연결 성공 시 발생
    Connected,
    /// 연결이 완전히 종료되었을 때 발생
    Disconnected,

    // --- 채팅 관련 이벤트 ---
    BJStateChange,
    /// 채팅 메시지가 수신되었을 때 발생합니다.
    Chat(ChatEvent),
    /// 후원 (텍스트, 영상, 애드벌룬)이 발생했을 때.
    Donation(DonationEvent),
    /// 구독이 발생했을 때.
    Subscribe(SubscribeEvent),
    /// 새로운 시청자가 채팅방에 입장했을 때.
    Enter(UserEvent),
    /// 시청자가 채팅방에서 퇴장했을 때.
    Exit(UserEvent),
    // 시청자가 퇴장당한 경우,
    Kick(UserEvent),
    // 강제 퇴장 취소,
    KickCancel(SimplifiedUserEvent),
    // 시청자가 Mute 당한 경우
    Mute(MuteEvent),
    // 시청자가 블랙 당한경우,
    Black(SimplifiedUserEvent),
    // 채팅방이 얼려진 경우,
    Freeze(FreezeEvent),
    /// 스트리머나 매니저가 보낸 공지사항 메시지.
    Notification(NotificationEvent),
    /// 시청자가 채팅방에 입장했을때
    Join(SimplifiedUserEvent),
    /// 미션
    MissionDonation(MissionEvent),
    /// 미션 정산
    MissionTotal(MissionTotalEvent),
    /// 미션 결과
    BattleMissionResult(BattleMissionResultEvent),
    ChallengeMissionResult(ChallengeMissionResultEvent),
    /// 알 수 없는 이벤트 타입
    Unknown(MessageCode),
    // 슬로우 이벤트
    Slow(SlowEvent),
    /// 직접 처리
    #[serde(serialize_with = "serialize_arc_bytes")]
    Raw(Arc<[u8]>), // 원시 데이터로 처리할 수 있는 이벤트
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

// --- 채팅 관련 이벤트 ---

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatEvent {
    /// 공통 속성 영역
    #[serde(flatten)]
    pub meta: EventMeta,
    pub comment: String,
    pub chat_type: ChatType,
    pub user: User,
    // manager chat에서만 true/false로 할당됩니다.
    pub is_admin: bool,
    // emoticon 채팅에서만 할당됩니다.
    pub emoticon: Option<Emoticon>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DonationEvent {
    #[serde(flatten)]
    pub meta: EventMeta,
    pub from: String,
    pub from_label: String,
    pub amount: u32,           // 후원 금액
    pub fan_club_ordinal: u32, // 팬클럽 순번
    pub become_top_fan: bool,
    pub donation_type: DonationType,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MissionEvent {
    #[serde(flatten)]
    pub meta: EventMeta,
    pub from: String,
    pub from_label: String,
    pub amount: u32, // 후원 금액
    pub mission_type: MissionType,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MissionTotalEvent {
    #[serde(flatten)]
    pub meta: EventMeta,
    pub mission_type: MissionType,
    pub amount: u32,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChallengeMissionResultEvent {
    #[serde(flatten)]
    pub meta: EventMeta,
    pub is_success: bool,
    pub title: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BattleMissionResultEvent {
    #[serde(flatten)]
    pub meta: EventMeta,
    pub is_draw: bool,
    pub winner: String,
    pub title: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SubscribeEvent {
    #[serde(flatten)]
    pub meta: EventMeta,
    pub user_id: String,
    pub label: String,
    pub tier: u32,
    // 구독 갱신인 경우 할당됩니다.
    pub renew: u32,
}

#[derive(Debug, Clone, Serialize)]
pub struct NotificationEvent {
    #[serde(flatten)]
    pub meta: EventMeta,
    pub message: String,
    pub show: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct UserEvent {
    #[serde(flatten)]
    pub meta: EventMeta,
    #[serde(flatten)]
    pub user: User,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SimplifiedUserEvent {
    #[serde(flatten)]
    pub meta: EventMeta,
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FreezeEvent {
    #[serde(flatten)]
    pub meta: EventMeta,
    pub freezed: bool,
    pub limit_subscription_month: u32,
    pub limit_balloons: u32,
    pub targets: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MuteEvent {
    #[serde(flatten)]
    pub meta: EventMeta,
    pub user: User,
    pub seconds: u32,
    pub message: String,
    pub by: String,
    pub counts: u32,
    pub superuser_type: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct SlowEvent {
    #[serde(flatten)]
    pub meta: EventMeta,
    pub duration: u32,
}
