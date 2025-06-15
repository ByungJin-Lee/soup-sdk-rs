use std::cmp::max;

use crate::chat::{
    events::{EventMeta, SlowEvent},
    parser::raw::RawMessage,
};

// bool: 강제퇴장 여부
pub fn parse_slow_event(raw: RawMessage) -> SlowEvent {
    let body = raw.body;

    SlowEvent {
        meta: EventMeta {
            received_time: raw.received_time,
        },
        duration: max(
            body[0].parse::<u32>().unwrap_or(0),
            body[1].parse::<u32>().unwrap_or(0),
        ),
    }
}
