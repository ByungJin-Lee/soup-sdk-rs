use crate::chat::{
    events::{EventMeta, SimplifiedUserEvent},
    parser::{raw::RawMessage, util::normalize_user_id},
};

pub fn parse_join_event(raw: RawMessage) -> Option<SimplifiedUserEvent> {
    let body = raw.body;

    if body.len() != 3 {
        return None;
    }

    Some(SimplifiedUserEvent {
        meta: EventMeta {
            received_time: raw.received_time,
        },
        user_id: normalize_user_id(&body[0]),
    })
}
