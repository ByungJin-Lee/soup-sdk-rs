use crate::chat::{DonationEvent, events::EventMeta, parser::raw::RawMessage, types::DonationType};

pub fn parse_balloon_event(raw: RawMessage) -> DonationEvent {
    let body = raw.body;

    DonationEvent {
        meta: EventMeta {
            received_time: raw.received_time,
        },
        donation_type: DonationType::Balloon,
        from: body[1].clone(),
        from_label: body[2].clone(),
        amount: body[3].parse::<u32>().unwrap_or(0),
        fan_club_ordinal: body[4].parse::<u32>().unwrap_or(0),
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
        from: body[3].clone(),
        from_label: body[4].clone(),
        amount: body[5].parse::<u32>().unwrap_or(0),
        fan_club_ordinal: body[6].parse::<u32>().unwrap_or(0),
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
        from: body[1].clone(),
        from_label: body[2].clone(),
        amount: body[3].parse::<u32>().unwrap_or(0),
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
        from: body[1].clone(),
        from_label: body[2].clone(),
        amount: body[3].parse::<u32>().unwrap_or(0),
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
        from: body[2].clone(),
        from_label: body[3].clone(),
        amount: body[9].parse::<u32>().unwrap_or(0),
        fan_club_ordinal: body[10].parse::<u32>().unwrap_or(0),
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
        from: body[1].clone(),
        from_label: body[2].clone(),
        amount: body[3].parse::<u32>().unwrap_or(0),
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
        from: body[2].clone(),
        from_label: body[3].clone(),
        amount: body[4].parse::<u32>().unwrap_or(0),
        fan_club_ordinal: body[5].parse::<u32>().unwrap_or(0),
        become_top_fan: body[7] == "1",
    }
}
