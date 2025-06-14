use crate::chat::{
    events::{EventMeta, UserEvent},
    parser::{raw::RawMessage, user::parse_user_status},
    types::User,
};

pub fn parse_exit_event(raw: RawMessage) -> Option<UserEvent> {
    let body = raw.body;

    // "1"인 경우는 특정되지 않은 이벤트
    if body[0] == "1" {
        return None;
    }

    Some(UserEvent {
        meta: EventMeta {
            received_time: raw.received_time,
        },
        user: User {
            id: body[1].clone(),
            label: body[2].clone(),
            status: parse_user_status(&body[5]),
            subscribe: None,
        },
    })
}
