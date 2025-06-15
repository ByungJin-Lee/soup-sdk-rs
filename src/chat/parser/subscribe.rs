use crate::chat::{
    events::{EventMeta, SubscribeEvent},
    parser::raw::RawMessage,
};

pub fn parse_subscribe_event(raw: RawMessage) -> SubscribeEvent {
    let body = raw.body;

    SubscribeEvent {
        meta: EventMeta {
            received_time: raw.received_time,
        },
        user_id: body[2].clone(),
        label: body[3].clone(),
        tier: body[7].parse::<u32>().unwrap_or(0),
        // 갱신이 아닌 경우는 0으로 할당
        renew: 0,
    }
}

pub fn parse_subscribe_renew_event(raw: RawMessage) -> SubscribeEvent {
    let body = raw.body;

    SubscribeEvent {
        meta: EventMeta {
            received_time: raw.received_time,
        },
        user_id: body[1].clone(),
        label: body[2].clone(),
        tier: body[7].parse::<u32>().unwrap_or(0),
        // 갱신이 아닌 경우는 0으로 할당
        renew: body[3].parse::<u32>().unwrap_or(1),
    }
}
