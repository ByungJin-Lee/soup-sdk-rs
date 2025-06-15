use crate::chat::{
    ChatEvent,
    constants::chat_message_fields,
    events::{EventMeta, ManagerChatEvent},
    parser::{raw::RawMessage, user::parse_user_status},
    types::{User, UserSubscribe},
};

pub fn parse_chat_event(raw: RawMessage) -> ChatEvent {
    let body = raw.body;
    let sub = parse_subscribe(&body);

    ChatEvent {
        meta: EventMeta {
            received_time: raw.received_time,
        },
        comment: body[chat_message_fields::CONTENT].clone().replace("\r", ""),
        user: User {
            id: body[chat_message_fields::USER_ID].clone(),
            label: body[chat_message_fields::USER_NICK].clone(),
            status: parse_user_status(&body[chat_message_fields::FLAGS]),
            subscribe: Some(sub),
        },
    }
}

pub fn parse_manager_chat_event(raw: RawMessage) -> ManagerChatEvent {
    let body = raw.body;

    ManagerChatEvent {
        meta: EventMeta {
            received_time: raw.received_time,
        },
        comment: body[chat_message_fields::CONTENT].clone().replace("\r", ""),
        user: User {
            id: body[chat_message_fields::USER_ID].clone(),
            label: body[4].clone(),
            status: parse_user_status(&body[5]),
            subscribe: None,
        },
        is_admin: body[2] == "1",
    }
}

fn parse_subscribe(body: &Vec<String>) -> UserSubscribe {
    UserSubscribe {
        acc: body[chat_message_fields::ACC_SUBSCRIBE]
            .parse::<u32>()
            .unwrap_or_default(),
        current: body[chat_message_fields::SUBSCRIBE]
            .parse::<u32>()
            .unwrap_or_default(),
    }
}
