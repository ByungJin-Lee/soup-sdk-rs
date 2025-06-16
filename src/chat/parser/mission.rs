use std::any::Any;

use crate::{
    Error, Result,
    chat::{
        events::{
            BattleMissionResultEvent, ChallengeMissionResultEvent, EventMeta, MissionEvent,
            MissionTotalEvent,
        },
        parser::{
            raw::RawMessage,
            types::{
                AbstractMissionData, BattleMissionResultPayload, ChallengeMissionResultPayload,
                MissionGiftPayload, MissionGiftTotalPayload,
            },
        },
        types::{MissionParser, MissionType},
    },
};

pub fn parse_mission_event(raw: RawMessage) -> Result<(MissionParser, Box<dyn Any>)> {
    let body = &raw.body;
    let raw_json = body[0].clone();
    let abs_json: AbstractMissionData =
        serde_json::from_str(&raw_json).map_err(|err| Error::InternalChannel(err.to_string()))?;

    let message_type = abs_json.message_type.as_str();

    match message_type {
        "GIFT" | "CHALLENGE_GIFT" => {
            let e = parse_gift_event(&raw, &raw_json, message_type)?;
            Ok((MissionParser::Mission, Box::new(e)))
        }
        "SETTLE" | "CHALLENGE_SETTLE" => {
            let e = parse_gift_total_event(raw, &raw_json, message_type)?;
            Ok((MissionParser::MissionTotal, Box::new(e)))
        }
        "NOTICE" => {
            let e = parse_battle_result(raw, &raw_json)?;
            Ok((MissionParser::BattleNotice, Box::new(e)))
        }
        "CHALLENGE_NOTICE" => {
            let e = parse_challenge_result(raw, &raw_json)?;
            Ok((MissionParser::ChallengeNotice, Box::new(e)))
        }
        _ => Err(Error::InternalChannel("미션 이벤트 파싱 실패".to_string())),
    }
}

fn parse_gift_event(raw: &RawMessage, body: &str, message_type: &str) -> Result<MissionEvent> {
    let p: MissionGiftPayload = serde_json::from_str(body)
        .map_err(|_| Error::InternalChannel("미션 페이로드 파싱오류".to_string()))?;

    Ok(MissionEvent {
        meta: EventMeta {
            received_time: raw.received_time,
        },
        from: p.user_id,
        from_label: p.label,
        amount: p.amount as u32,
        mission_type: if message_type == "CHALLENGE_GIFT" {
            MissionType::Challenge
        } else {
            MissionType::Battle
        },
    })
}

fn parse_gift_total_event(
    raw: RawMessage,
    body: &str,
    message_type: &str,
) -> Result<MissionTotalEvent> {
    let p: MissionGiftTotalPayload = serde_json::from_str(body)
        .map_err(|_| Error::InternalChannel("미션 페이로드 파싱오류".to_string()))?;

    Ok(MissionTotalEvent {
        meta: EventMeta {
            received_time: raw.received_time,
        },
        mission_type: if message_type == "CHALLENGE_SETTLE" {
            MissionType::Challenge
        } else {
            MissionType::Battle
        },
        amount: p.amount as u32,
    })
}

fn parse_battle_result(raw: RawMessage, body: &str) -> Result<BattleMissionResultEvent> {
    let p: BattleMissionResultPayload = serde_json::from_str(body)
        .map_err(|_| Error::InternalChannel("미션 페이로드 파싱오류".to_string()))?;

    Ok(BattleMissionResultEvent {
        meta: EventMeta {
            received_time: raw.received_time,
        },
        is_draw: p.draw,
        winner: p.winner,
        title: p.title,
    })
}

fn parse_challenge_result(raw: RawMessage, body: &str) -> Result<ChallengeMissionResultEvent> {
    let p: ChallengeMissionResultPayload = serde_json::from_str(body)
        .map_err(|_| Error::InternalChannel("미션 페이로드 파싱오류".to_string()))?;

    Ok(ChallengeMissionResultEvent {
        meta: EventMeta {
            received_time: raw.received_time,
        },
        is_success: p.status == "SUCCESS",
        title: p.title,
    })
}
