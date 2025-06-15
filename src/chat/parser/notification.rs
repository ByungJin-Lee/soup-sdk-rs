use crate::chat::{
    events::{EventMeta, NotificationEvent},
    parser::raw::RawMessage,
};

pub fn parse_notification_event(raw: RawMessage) -> NotificationEvent {
    let body = raw.body;

    NotificationEvent {
        meta: EventMeta {
            received_time: raw.received_time,
        },
        show: body[1] == "1",
        message: body[3].clone(),
    }
}
