use crate::chat::{
    events::{EventMeta, FreezeEvent},
    parser::{constants::freeze_target_flags, raw::RawMessage, util::is},
};

pub fn parse_freeze_event(raw: RawMessage) -> FreezeEvent {
    let body = raw.body;

    FreezeEvent {
        meta: EventMeta {
            received_time: raw.received_time,
        },
        freezed: body[0] != "0",
        limit_balloons: body[3].parse::<u32>().unwrap_or(0),
        limit_subscription_month: body[4].parse::<u32>().unwrap_or(0),
        targets: parse_freeze_targets(&body[2]),
    }
}

fn parse_freeze_targets(flag_str: &str) -> Vec<String> {
    let mut targets: Vec<String> = Vec::new();

    let flag = flag_str.parse::<u32>().unwrap();

    if is(flag, freeze_target_flags::NORMAL) {
        targets.push("BJ".to_string());
    }

    if is(flag, freeze_target_flags::FAN) {
        targets.push("FAN".to_string());
    }

    if is(flag, freeze_target_flags::SUPPORTER) {
        targets.push("SUPPORTER".to_string());
    }

    if is(flag, freeze_target_flags::TOP_FAN) {
        targets.push("TOP_FAN".to_string());
    }

    if is(flag, freeze_target_flags::FOLLOWER) {
        targets.push("FOLLOWER".to_string());
    }

    if is(flag, freeze_target_flags::MANAGER) {
        targets.push("MANGER".to_string());
    }

    targets
}
