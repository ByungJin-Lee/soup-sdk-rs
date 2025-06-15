use crate::chat::{
    events::{EventMeta, MuteEvent},
    parser::{constants::SUPER_USERS, raw::RawMessage, user::parse_user_status},
    types::User,
};

// bool: 강제퇴장 여부
pub fn parse_mute_event(raw: RawMessage) -> MuteEvent {
    let body = raw.body;

    MuteEvent {
        meta: EventMeta {
            received_time: raw.received_time,
        },
        user: User {
            id: body[0].clone(),
            label: body[7].clone(),
            status: parse_user_status(&body[1]),
            subscribe: None,
        },
        by: SUPER_USERS[body[5].parse::<usize>().unwrap_or(0)].to_string(),
        message: "".to_string(),
        seconds: body[2].parse::<u32>().unwrap_or(0),
        counts: body[3].parse::<u32>().unwrap_or(1),
    }
}
