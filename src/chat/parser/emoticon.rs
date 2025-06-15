use crate::chat::{
    ChatEvent,
    events::EventMeta,
    parser::{raw::RawMessage, user::parse_user_status},
    types::{ChatType, Emoticon, User, UserSubscribe},
};

pub fn parse_emoticon_event(raw: RawMessage) -> ChatEvent {
    let body = raw.body;
    let sub = parse_subscribe(&body);

    ChatEvent {
        meta: EventMeta {
            received_time: raw.received_time,
        },
        chat_type: ChatType::Emoticon,
        comment: body[1].clone().replace("\r", ""),
        user: User {
            id: body[5].clone(),
            label: body[6].clone(),
            status: parse_user_status(&body[7]),
            subscribe: Some(sub),
        },
        emoticon: Some(Emoticon {
            id: body[2].clone(),
            number: body[3].clone(),
            ext: body[11].clone(),
            version: body[4].clone(),
        }),
        is_admin: false,
    }
}

fn parse_subscribe(body: &Vec<String>) -> UserSubscribe {
    UserSubscribe {
        acc: body[15].parse::<u32>().unwrap_or_default(),
        current: body[12].parse::<u32>().unwrap_or_default(),
    }
}
