use crate::chat::{
    events::{EventMeta, StickerEvent},
    parser::{
        raw::RawMessage,
        util::{normalize_user_id, parse_u32_or_default},
    },
};

pub fn parse_sticker_event(raw: RawMessage) -> StickerEvent {
    let body = raw.body;

    StickerEvent {
        meta: EventMeta {
            received_time: raw.received_time,
        },
        from: normalize_user_id(&body[2]),
        from_label: body[3].clone(),
        amount: parse_u32_or_default(&body[7]),
        supporter_ordinal: parse_u32_or_default(&body[8]),
    }
}

pub fn parse_sticker_sub_event(raw: RawMessage) -> StickerEvent {
    let body = raw.body;

    StickerEvent {
        meta: EventMeta {
            received_time: raw.received_time,
        },
        from: normalize_user_id(&body[3]),
        from_label: body[4].clone(),
        amount: parse_u32_or_default(&body[8]),
        supporter_ordinal: parse_u32_or_default(&body[9]),
    }
}
