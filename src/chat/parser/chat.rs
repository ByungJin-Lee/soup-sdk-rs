use crate::chat::{
    ChatEvent,
    constants::chat_message_fields,
    events::EventMeta,
    parser::{raw::RawMessage, user::parse_user_status},
    types::{ChatType, User, UserSubscribe},
};

pub fn parse_chat_event(raw: RawMessage) -> ChatEvent {
    let body = raw.body;
    let sub = parse_subscribe(&body);

    ChatEvent {
        meta: EventMeta {
            received_time: raw.received_time,
        },
        chat_type: ChatType::Common,
        comment: body[chat_message_fields::CONTENT].clone().replace("\r", ""),
        user: User {
            id: body[chat_message_fields::USER_ID].clone(),
            label: body[chat_message_fields::USER_NICK].clone(),
            status: parse_user_status(&body[chat_message_fields::FLAGS]),
            subscribe: Some(sub),
        },
        is_admin: false,
        emoticon: None,
    }
}

pub fn parse_manager_chat_event(raw: RawMessage) -> ChatEvent {
    let body = raw.body;

    ChatEvent {
        meta: EventMeta {
            received_time: raw.received_time,
        },
        chat_type: ChatType::Manager,
        comment: body[chat_message_fields::CONTENT].clone().replace("\r", ""),
        user: User {
            id: body[chat_message_fields::USER_ID].clone(),
            label: body[4].clone(),
            status: parse_user_status(&body[5]),
            subscribe: None,
        },
        emoticon: None,
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
