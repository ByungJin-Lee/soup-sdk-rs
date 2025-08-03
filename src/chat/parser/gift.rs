use crate::chat::{
    events::{EventMeta, GiftEvent},
    parser::{raw::RawMessage, util::normalize_user_id},
    types::GiftType,
};

pub fn parse_subscribe_gift_event(raw: RawMessage) -> GiftEvent {
    let body = raw.body;

    GiftEvent {
        meta: EventMeta {
            received_time: raw.received_time,
        },
        gift_type: GiftType::Subscription,
        sender_id: normalize_user_id(&body[1]),
        sender_label: body[2].clone(),
        receiver_id: normalize_user_id(&body[3]),
        receiver_label: body[4].clone(),
        gift_code: body[7].clone(),
    }
}

pub fn parse_quickview_gift_event(raw: RawMessage) -> GiftEvent {
    let body = raw.body;

    GiftEvent {
        meta: EventMeta {
            received_time: raw.received_time,
        },
        gift_type: GiftType::QuickView,
        sender_id: normalize_user_id(&body[1]),
        sender_label: body[2].clone(),
        receiver_id: normalize_user_id(&body[3]),
        receiver_label: body[4].clone(),
        gift_code: body[5].clone(),
    }
}

pub fn parse_ogq_gift_event(raw: RawMessage) -> GiftEvent {
    let body = raw.body;

    GiftEvent {
        meta: EventMeta {
            received_time: raw.received_time,
        },
        gift_type: GiftType::OGQ,
        sender_id: normalize_user_id(&body[1]),
        sender_label: body[2].clone(),
        receiver_id: normalize_user_id(&body[3]),
        receiver_label: body[4].clone(),
        gift_code: body[5].clone(),
    }
}
