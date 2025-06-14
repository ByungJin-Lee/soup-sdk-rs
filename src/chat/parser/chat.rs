use crate::chat::{
    ChatEvent,
    constants::{chat_message_fields, user_flags},
    events::EventMeta,
    parser::{raw::RawMessage, types::UserFlags, user::parse_user_status, util::is},
    types::{User, UserSubscribe},
};

pub fn parse_chat_event(raw: RawMessage) -> ChatEvent {
    let body = raw.body;
    let sub = parse_subscribe(&body);

    ChatEvent {
        meta: EventMeta {
            received_time: raw.received_time,
        },
        comment: body[chat_message_fields::CONTENT].clone(),
        user: User {
            id: body[chat_message_fields::USER_ID].clone(),
            label: body[chat_message_fields::USER_NICK].clone(),
            status: parse_user_status(&body[chat_message_fields::FLAGS]),
            subscribe: Some(sub),
        },
    }
}

fn parse_user_flags(flag_str: &str) -> UserFlags {
    let flags: Vec<u32> = flag_str
        .split("|")
        .map(|val| val.parse::<u32>().unwrap())
        .collect();

    return UserFlags {
        follow: flags[1],
        combined: flags[0],
    };
}

fn get_follow(flags: u32) -> u8 {
    // 1티어
    if is(flags, user_flags::FOLLOWER_TIER1) {
        return 1;
        // 2티어
    } else if is(flags, user_flags::FOLLOWER_TIER2) {
        return 2;
    }
    return 0;
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
