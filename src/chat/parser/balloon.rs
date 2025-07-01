use crate::chat::{DonationEvent, events::EventMeta, parser::{raw::RawMessage, util::{normalize_user_id, parse_u32_or_default}}, types::DonationType};

pub fn parse_balloon_event(raw: RawMessage) -> DonationEvent {
    let body = raw.body;

    DonationEvent {
        meta: EventMeta {
            received_time: raw.received_time,
        },
        donation_type: DonationType::Balloon,
        from: normalize_user_id(&body[1]),
        from_label: body[2].clone(),
        amount: parse_u32_or_default(&body[3]),
        fan_club_ordinal: parse_u32_or_default(&body[4]),
        become_top_fan: "1" == body[8],
    }
}

pub fn parse_balloon_sub_event(raw: RawMessage) -> DonationEvent {
    let body = raw.body;

    DonationEvent {
        meta: EventMeta {
            received_time: raw.received_time,
        },
        donation_type: DonationType::Balloon,
        from: normalize_user_id(&body[3]),
        from_label: body[4].clone(),
        amount: parse_u32_or_default(&body[5]),
        fan_club_ordinal: parse_u32_or_default(&body[6]),
        become_top_fan: "1" == body[9],
    }
}

pub fn parse_vod_balloon_event(raw: RawMessage) -> DonationEvent {
    let body = raw.body;

    DonationEvent {
        meta: EventMeta {
            received_time: raw.received_time,
        },
        donation_type: DonationType::Balloon,
        from: normalize_user_id(&body[1]),
        from_label: body[2].clone(),
        amount: parse_u32_or_default(&body[3]),
        fan_club_ordinal: 0,
        become_top_fan: false,
    }
}

/// --- ad balloon

pub fn parse_vod_ad_balloon_event(raw: RawMessage) -> DonationEvent {
    let body = raw.body;

    DonationEvent {
        meta: EventMeta {
            received_time: raw.received_time,
        },
        donation_type: DonationType::ADBalloon,
        from: normalize_user_id(&body[1]),
        from_label: body[2].clone(),
        amount: parse_u32_or_default(&body[3]),
        fan_club_ordinal: 0,
        become_top_fan: false,
    }
}

pub fn parse_ad_balloon_event(raw: RawMessage) -> DonationEvent {
    let body = raw.body;

    DonationEvent {
        meta: EventMeta {
            received_time: raw.received_time,
        },
        donation_type: DonationType::ADBalloon,
        from: normalize_user_id(&body[2]),
        from_label: body[3].clone(),
        amount: parse_u32_or_default(&body[9]),
        fan_club_ordinal: parse_u32_or_default(&body[10]),
        become_top_fan: body[12] == "1",
    }
}

pub fn parse_station_ad_balloon_event(raw: RawMessage) -> DonationEvent {
    let body = raw.body;

    DonationEvent {
        meta: EventMeta {
            received_time: raw.received_time,
        },
        donation_type: DonationType::ADBalloon,
        from: normalize_user_id(&body[1]),
        from_label: body[2].clone(),
        amount: parse_u32_or_default(&body[3]),
        fan_club_ordinal: 0,
        become_top_fan: false,
    }
}

pub fn parse_video_balloon_event(raw: RawMessage) -> DonationEvent {
    let body = raw.body;

    DonationEvent {
        meta: EventMeta {
            received_time: raw.received_time,
        },
        donation_type: DonationType::VODBalloon,
        from: normalize_user_id(&body[2]),
        from_label: body[3].clone(),
        amount: parse_u32_or_default(&body[4]),
        fan_club_ordinal: parse_u32_or_default(&body[5]),
        become_top_fan: body[7] == "1",
    }
}
