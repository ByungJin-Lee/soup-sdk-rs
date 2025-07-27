use crate::chat::events::{
    ChallengeMissionResultEvent, ChatEvent, DonationEvent, Event, EventMeta, MissionEvent,
    SimplifiedUserEvent, SubscribeEvent,
};
use crate::chat::types::{ChatType, DonationType, MissionType, User, UserStatus};
use chrono::{DateTime, NaiveDateTime, Utc};
use quick_xml::Reader;
use quick_xml::events::Event as XmlEvent;

pub fn parse_vod_chat_xml_with_start_time(
    xml_content: &str,
    broad_start: Option<&str>,
) -> Result<Vec<Event>, Box<dyn std::error::Error>> {
    let mut reader = Reader::from_str(xml_content);
    reader.config_mut().trim_text(true);

    let mut events = Vec::new();
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(XmlEvent::Start(ref e)) => match e.name().as_ref() {
                b"chat" => {
                    if let Ok(event) = parse_chat_element(&mut reader, broad_start) {
                        events.push(event);
                    }
                }
                b"follow" => {
                    if let Ok(event) = parse_follow_element(&mut reader) {
                        events.push(event);
                    }
                }
                b"adballoon" => {
                    if let Ok(event) = parse_adballoon_element(&mut reader) {
                        events.push(event);
                    }
                }
                b"fanclub" => {
                    if let Ok(event) = parse_fanclub_element(&mut reader) {
                        events.push(event);
                    }
                }
                b"balloon" => {
                    if let Ok(event) = parse_balloon_element(&mut reader) {
                        events.push(event);
                    }
                }
                b"challenge_mission" => {
                    if let Ok(event) = parse_challenge_mission_element(&mut reader) {
                        events.push(event);
                    }
                }
                _ => {}
            },
            Ok(XmlEvent::Eof) => break,
            Err(e) => return Err(Box::new(e)),
            _ => {}
        }
        buf.clear();
    }

    Ok(events)
}

fn calculate_event_time(broad_start: Option<&str>, timestamp_seconds: f64) -> DateTime<Utc> {
    if let Some(start_time_str) = broad_start {
        if let Ok(naive_start) = NaiveDateTime::parse_from_str(start_time_str, "%Y-%m-%d %H:%M:%S")
        {
            let start_utc = naive_start.and_utc();
            let duration = chrono::Duration::milliseconds((timestamp_seconds * 1000.0) as i64);
            return start_utc + duration;
        }
    }
    Utc::now()
}

fn parse_chat_element(
    reader: &mut Reader<&[u8]>,
    broad_start: Option<&str>,
) -> Result<Event, Box<dyn std::error::Error>> {
    let mut message = String::new();
    let mut user_id = String::new();
    let mut nickname = String::new();
    let mut timestamp = 0.0;
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(XmlEvent::Start(ref e)) => match e.name().as_ref() {
                b"m" => {
                    if let Ok(XmlEvent::CData(cdata)) = reader.read_event_into(&mut buf) {
                        message = String::from_utf8_lossy(&cdata).to_string();
                    }
                }
                b"u" => {
                    if let Ok(XmlEvent::Text(text)) = reader.read_event_into(&mut buf) {
                        user_id = String::from_utf8_lossy(&text).to_string();
                    }
                }
                b"n" => {
                    if let Ok(XmlEvent::CData(cdata)) = reader.read_event_into(&mut buf) {
                        nickname = String::from_utf8_lossy(&cdata).to_string();
                    }
                }
                b"t" => {
                    if let Ok(XmlEvent::Text(text)) = reader.read_event_into(&mut buf) {
                        timestamp = String::from_utf8_lossy(&text).parse().unwrap_or(0.0);
                    }
                }
                _ => {}
            },
            Ok(XmlEvent::End(ref e)) if e.name().as_ref() == b"chat" => break,
            Ok(XmlEvent::Eof) => break,
            Err(e) => return Err(Box::new(e)),
            _ => {}
        }
        buf.clear();
    }

    Ok(Event::Chat(ChatEvent {
        meta: EventMeta {
            received_time: calculate_event_time(broad_start, timestamp),
        },
        comment: message,
        chat_type: ChatType::Common,
        user: User {
            id: user_id,
            label: nickname,
            status: UserStatus {
                follow: 0,
                is_bj: false,
                is_manager: false,
                is_top_fan: false,
                is_fan: false,
                is_supporter: false,
            },
            subscribe: None,
        },
        is_admin: false,
        emoticon: None,
    }))
}

fn parse_follow_element(reader: &mut Reader<&[u8]>) -> Result<Event, Box<dyn std::error::Error>> {
    let mut user_id = String::new();
    let mut nickname = String::new();
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(XmlEvent::Start(ref e)) => match e.name().as_ref() {
                b"u" => {
                    if let Ok(XmlEvent::Text(text)) = reader.read_event_into(&mut buf) {
                        user_id = String::from_utf8_lossy(&text).to_string();
                    }
                }
                b"n" => {
                    if let Ok(XmlEvent::CData(cdata)) = reader.read_event_into(&mut buf) {
                        nickname = String::from_utf8_lossy(&cdata).to_string();
                    }
                }
                _ => {}
            },
            Ok(XmlEvent::End(ref e)) if e.name().as_ref() == b"follow" => break,
            Ok(XmlEvent::Eof) => break,
            Err(e) => return Err(Box::new(e)),
            _ => {}
        }
        buf.clear();
    }

    Ok(Event::Subscribe(SubscribeEvent {
        meta: EventMeta {
            received_time: Utc::now(),
        },
        user_id,
        label: nickname,
        tier: 1,
        renew: 0,
    }))
}

fn parse_adballoon_element(
    reader: &mut Reader<&[u8]>,
) -> Result<Event, Box<dyn std::error::Error>> {
    let mut user_id = String::new();
    let mut nickname = String::new();
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(XmlEvent::Start(ref e)) => match e.name().as_ref() {
                b"u" => {
                    if let Ok(XmlEvent::Text(text)) = reader.read_event_into(&mut buf) {
                        user_id = String::from_utf8_lossy(&text).to_string();
                    }
                }
                b"n" => {
                    if let Ok(XmlEvent::CData(cdata)) = reader.read_event_into(&mut buf) {
                        nickname = String::from_utf8_lossy(&cdata).to_string();
                    }
                }
                _ => {}
            },
            Ok(XmlEvent::End(ref e)) if e.name().as_ref() == b"adballoon" => break,
            Ok(XmlEvent::Eof) => break,
            Err(e) => return Err(Box::new(e)),
            _ => {}
        }
        buf.clear();
    }

    Ok(Event::Donation(DonationEvent {
        meta: EventMeta {
            received_time: Utc::now(),
        },
        from: user_id,
        from_label: nickname,
        amount: 1,
        fan_club_ordinal: 0,
        become_top_fan: false,
        donation_type: DonationType::ADBalloon,
    }))
}

fn parse_fanclub_element(reader: &mut Reader<&[u8]>) -> Result<Event, Box<dyn std::error::Error>> {
    let mut user_id = String::new();
    let mut nickname = String::new();
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(XmlEvent::Start(ref e)) => match e.name().as_ref() {
                b"u" => {
                    if let Ok(XmlEvent::Text(text)) = reader.read_event_into(&mut buf) {
                        user_id = String::from_utf8_lossy(&text).to_string();
                    }
                }
                b"n" => {
                    if let Ok(XmlEvent::CData(cdata)) = reader.read_event_into(&mut buf) {
                        nickname = String::from_utf8_lossy(&cdata).to_string();
                    }
                }
                _ => {}
            },
            Ok(XmlEvent::End(ref e)) if e.name().as_ref() == b"fanclub" => break,
            Ok(XmlEvent::Eof) => break,
            Err(e) => return Err(Box::new(e)),
            _ => {}
        }
        buf.clear();
    }

    Ok(Event::Join(SimplifiedUserEvent {
        meta: EventMeta {
            received_time: Utc::now(),
        },
        user_id,
    }))
}

fn parse_balloon_element(reader: &mut Reader<&[u8]>) -> Result<Event, Box<dyn std::error::Error>> {
    let mut user_id = String::new();
    let mut nickname = String::new();
    let mut amount = 0u32;
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(XmlEvent::Start(ref e)) => match e.name().as_ref() {
                b"u" => {
                    if let Ok(XmlEvent::Text(text)) = reader.read_event_into(&mut buf) {
                        user_id = String::from_utf8_lossy(&text).to_string();
                    }
                }
                b"n" => {
                    if let Ok(XmlEvent::CData(cdata)) = reader.read_event_into(&mut buf) {
                        nickname = String::from_utf8_lossy(&cdata).to_string();
                    }
                }
                b"fn" => {
                    if let Ok(XmlEvent::Text(text)) = reader.read_event_into(&mut buf) {
                        let amount_str = String::from_utf8_lossy(&text);

                        if let Some(underscore_pos) = amount_str.rfind('_') {
                            if let Ok(parsed_amount) =
                                amount_str[underscore_pos + 1..].parse::<u32>()
                            {
                                amount = parsed_amount;
                            }
                        }
                    }
                }
                b"c" => {
                    if let Ok(XmlEvent::Text(text)) = reader.read_event_into(&mut buf) {
                        if let Ok(parsed_amount) = String::from_utf8_lossy(&text).parse::<u32>() {
                            amount = parsed_amount;
                        }
                    }
                }
                _ => {}
            },
            Ok(XmlEvent::End(ref e)) if e.name().as_ref() == b"balloon" => break,
            Ok(XmlEvent::Eof) => break,
            Err(e) => return Err(Box::new(e)),
            _ => {}
        }
        buf.clear();
    }

    Ok(Event::Donation(DonationEvent {
        meta: EventMeta {
            received_time: Utc::now(),
        },
        from: user_id,
        from_label: nickname,
        amount,
        fan_club_ordinal: 0,
        become_top_fan: false,
        donation_type: DonationType::Balloon,
    }))
}

fn parse_challenge_mission_element(
    reader: &mut Reader<&[u8]>,
) -> Result<Event, Box<dyn std::error::Error>> {
    let mut mission_type_str = String::new();
    let mut user_id = String::new();
    let mut nickname = String::new();
    let mut amount = 0u32;
    let mut title = String::new();
    let mut success_status = String::new();
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(XmlEvent::Start(ref e)) => match e.name().as_ref() {
                b"type" => {
                    if let Ok(XmlEvent::Text(text)) = reader.read_event_into(&mut buf) {
                        mission_type_str = String::from_utf8_lossy(&text).to_string();
                    }
                }
                b"u" => {
                    if let Ok(XmlEvent::Text(text)) = reader.read_event_into(&mut buf) {
                        user_id = String::from_utf8_lossy(&text).to_string();
                    }
                }
                b"n" => {
                    if let Ok(XmlEvent::CData(cdata)) = reader.read_event_into(&mut buf) {
                        nickname = String::from_utf8_lossy(&cdata).to_string();
                    }
                }
                b"title" => {
                    if let Ok(XmlEvent::CData(cdata)) = reader.read_event_into(&mut buf) {
                        title = String::from_utf8_lossy(&cdata).to_string();
                    }
                }
                b"c" => {
                    if let Ok(XmlEvent::Text(text)) = reader.read_event_into(&mut buf) {
                        if let Ok(parsed_amount) = String::from_utf8_lossy(&text).parse::<u32>() {
                            amount = parsed_amount;
                        }
                    }
                }
                b"ms" => {
                    if let Ok(XmlEvent::Text(text)) = reader.read_event_into(&mut buf) {
                        success_status = String::from_utf8_lossy(&text).to_string();
                    }
                }
                _ => {}
            },
            Ok(XmlEvent::End(ref e)) if e.name().as_ref() == b"challenge_mission" => break,
            Ok(XmlEvent::Eof) => break,
            Err(e) => return Err(Box::new(e)),
            _ => {}
        }
        buf.clear();
    }

    match mission_type_str.as_str() {
        "CHALLENGE_GIFT" => Ok(Event::MissionDonation(MissionEvent {
            meta: EventMeta {
                received_time: Utc::now(),
            },
            from: user_id,
            from_label: nickname,
            amount,
            mission_type: MissionType::Challenge,
        })),
        "CHALLENGE_SETTLE" => {
            // ! 정산은 MissionTotal 이벤트로 처리할 수도 있지만, 현재는 MissionDonation으로 처리
            Ok(Event::MissionDonation(MissionEvent {
                meta: EventMeta {
                    received_time: Utc::now(),
                },
                from: if user_id.is_empty() {
                    "system".to_string()
                } else {
                    user_id
                },
                from_label: if nickname.is_empty() {
                    "시스템".to_string()
                } else {
                    nickname
                },
                amount,
                mission_type: MissionType::Challenge,
            }))
        }
        "CHALLENGE_NOTICE" => {
            let is_success = success_status == "SUCCESS";
            Ok(Event::ChallengeMissionResult(ChallengeMissionResultEvent {
                meta: EventMeta {
                    received_time: Utc::now(),
                },
                is_success,
                title,
            }))
        }
        _ => {
            // 알 수 없는 타입은 일반 미션으로 처리
            Ok(Event::MissionDonation(MissionEvent {
                meta: EventMeta {
                    received_time: Utc::now(),
                },
                from: user_id,
                from_label: nickname,
                amount,
                mission_type: MissionType::Challenge,
            }))
        }
    }
}
