use tokio::sync::{broadcast, mpsc};

use crate::{
    Error, Result,
    chat::{
        Event,
        commands::{Command, MessageType},
        constants::message_codes,
        events::{
            BattleMissionResultEvent, ChallengeMissionResultEvent, MissionEvent, MissionTotalEvent,
        },
        formatter::ChatFormatter,
        parser::{
            balloon::{
                parse_ad_balloon_event, parse_balloon_event, parse_balloon_sub_event,
                parse_station_ad_balloon_event, parse_video_balloon_event,
                parse_vod_ad_balloon_event, parse_vod_balloon_event,
            },
            chat::{parse_chat_event, parse_manager_chat_event},
            emoticon::parse_emoticon_event,
            exit::parse_exit_event,
            freeze::parse_freeze_event,
            join::parse_join_event,
            kick::parse_kick_cancel_event,
            mission::parse_mission_event,
            mute::parse_mute_event,
            notification::parse_notification_event,
            raw::{RawMessage, parse_message},
            slow::parse_slow_event,
            subscribe::{parse_subscribe_event, parse_subscribe_renew_event},
        },
        types::MissionParser,
    },
};

pub struct MessageHandler {
    pub formatter: ChatFormatter,
    pub event_tx: broadcast::Sender<Event>,
    pub command_tx: mpsc::Sender<Command>,
}

impl MessageHandler {
    pub fn new(
        formatter: &ChatFormatter,
        event_tx: broadcast::Sender<Event>,
        command_tx: mpsc::Sender<Command>,
    ) -> Self {
        Self {
            formatter: formatter.clone(),
            event_tx,
            command_tx,
        }
    }
    /// 메시지를 처리하고 이벤트를 전송합니다.
    pub fn handle(&self, raw: Vec<u8>) -> Result<Option<Vec<u8>>> {
        // Raw 메시지 처리
        self.broadcast(Event::Raw(raw.clone()))?;
        // 메시지 파싱
        let ret = match parse_message(&raw) {
            Ok(message) => self.handle_message(message),
            Err(_) => {
                // 파싱 오류 처리
                // self.broadcast(Event::Error(e))?;
                None
            }
        };

        Ok(ret)
    }

    fn broadcast(&self, event: Event) -> Result<()> {
        self.event_tx
            .send(event)
            .map_err(|_| Error::InternalChannel("Failed to send event".into()))?;
        Ok(())
    }

    fn handle_message(&self, message: RawMessage) -> Option<Vec<u8>> {
        // 메시지 처리 로직을 여기에 구현합니다.
        // 예를 들어, raw 메시지를 파싱하고 필요한 이벤트를 생성할 수 있습니다.
        let res = match message.code {
            message_codes::CONNECT => self.handle_connect(message),
            message_codes::CHAT => self.handle_chat(message),
            message_codes::EXIT => self.handle_exit(message),
            message_codes::USER_JOIN => self.handle_join(message),
            message_codes::FREEZE => self.handle_freeze(message),
            message_codes::MUTE => self.handle_mute(message),
            message_codes::MANAGER_CHAT => self.handle_manager_message(message),
            message_codes::EMOTICON => self.handle_emoticon_message(message),
            message_codes::NOTIFICATION => self.handle_notification(message),
            message_codes::BJ_STATE_CHANGE => self.handle_bj_state_change(message),
            message_codes::SLOW => self.handle_slow(message),
            message_codes::KICK_CANCEL => self.handle_kick_cancel(message),
            message_codes::SUBSCRIBE => self.handle_subscribe(message),
            message_codes::SUBSCRIBE_RENEW => self.handle_subscribe_renew(message),
            // 미션
            message_codes::MISSION_DONATION => self.handle_mission(message),
            // 도네이션
            message_codes::DONATION
            | message_codes::ADBALLOON_DONATION
            | message_codes::SUB_DONATION
            | message_codes::VOD_AD_DONATION
            | message_codes::VOD_DONATION
            | message_codes::AD_STATION_DONATION
            | message_codes::VIDEO_DONATION => self.handle_donation(message),

            _ => {
                // 다른 메시지 코드 처리
                let _ = self.broadcast(Event::Unknown(message.code));
                None
            }
        };

        // 메시지에 대한 응답이 필요한 경우, Vec<u8>를 반환합니다.
        res
    }

    fn handle_donation(&self, message: RawMessage) -> Option<Vec<u8>> {
        let e = match message.code {
            message_codes::DONATION => parse_balloon_event(message),
            message_codes::SUB_DONATION => parse_balloon_sub_event(message),
            message_codes::VOD_DONATION => parse_vod_balloon_event(message),
            message_codes::VOD_AD_DONATION => parse_vod_ad_balloon_event(message),
            message_codes::ADBALLOON_DONATION => parse_ad_balloon_event(message),
            message_codes::AD_STATION_DONATION => parse_station_ad_balloon_event(message),
            message_codes::VIDEO_DONATION => parse_video_balloon_event(message),
            _ => return None,
        };
        let _ = self.broadcast(Event::Donation(e));
        None
    }

    fn handle_mission(&self, message: RawMessage) -> Option<Vec<u8>> {
        if let Some((name, val)) = parse_mission_event(message).ok() {
            match name {
                MissionParser::Mission => {
                    if let Ok(be) = val.downcast::<MissionEvent>() {
                        let e: MissionEvent = *be;
                        let _ = self.broadcast(Event::MissionDonation(e));
                    }
                }
                MissionParser::MissionTotal => {
                    if let Ok(be) = val.downcast::<MissionTotalEvent>() {
                        let e: MissionTotalEvent = *be;
                        let _ = self.broadcast(Event::MissionTotal(e));
                    }
                }
                MissionParser::BattleNotice => {
                    if let Ok(be) = val.downcast::<BattleMissionResultEvent>() {
                        let e: BattleMissionResultEvent = *be;
                        let _ = self.broadcast(Event::BattleMissionResult(e));
                    }
                }
                MissionParser::ChallengeNotice => {
                    if let Ok(be) = val.downcast::<ChallengeMissionResultEvent>() {
                        let e: ChallengeMissionResultEvent = *be;
                        let _ = self.broadcast(Event::ChallengeMissionResult(e));
                    }
                }
            }
        }

        None
    }

    fn handle_subscribe_renew(&self, message: RawMessage) -> Option<Vec<u8>> {
        let _ = self.broadcast(Event::Subscribe(parse_subscribe_renew_event(message)));
        None
    }

    fn handle_subscribe(&self, message: RawMessage) -> Option<Vec<u8>> {
        let _ = self.broadcast(Event::Subscribe(parse_subscribe_event(message)));
        None
    }

    fn handle_slow(&self, message: RawMessage) -> Option<Vec<u8>> {
        let _ = self.broadcast(Event::Slow(parse_slow_event(message)));
        None
    }

    fn handle_bj_state_change(&self, _: RawMessage) -> Option<Vec<u8>> {
        let _ = self.broadcast(Event::BJStateChange);
        None
    }

    fn handle_emoticon_message(&self, message: RawMessage) -> Option<Vec<u8>> {
        let _ = self.broadcast(Event::Chat(parse_emoticon_event(message)));
        None
    }

    fn handle_notification(&self, message: RawMessage) -> Option<Vec<u8>> {
        let _ = self.broadcast(Event::Notification(parse_notification_event(message)));
        None
    }

    fn handle_manager_message(&self, message: RawMessage) -> Option<Vec<u8>> {
        let _ = self.broadcast(Event::Chat(parse_manager_chat_event(message)));
        None
    }

    fn handle_kick_cancel(&self, message: RawMessage) -> Option<Vec<u8>> {
        if let Some(e) = parse_kick_cancel_event(message) {
            let _ = self.broadcast(Event::KickCancel(e));
        }
        None
    }

    fn handle_join(&self, message: RawMessage) -> Option<Vec<u8>> {
        if let Some(e) = parse_join_event(message) {
            let _ = self.broadcast(Event::Join(e));
        }
        None
    }

    fn handle_mute(&self, message: RawMessage) -> Option<Vec<u8>> {
        let _ = self.broadcast(Event::Mute(parse_mute_event(message)));
        None
    }

    fn handle_freeze(&self, message: RawMessage) -> Option<Vec<u8>> {
        let _ = self.broadcast(Event::Freeze(parse_freeze_event(message)));
        None
    }

    fn handle_chat(&self, message: RawMessage) -> Option<Vec<u8>> {
        let _ = self.broadcast(Event::Chat(parse_chat_event(message)));
        None
    }

    fn handle_exit(&self, message: RawMessage) -> Option<Vec<u8>> {
        if let Some((is_kick, e)) = parse_exit_event(message) {
            if is_kick {
                let _ = self.broadcast(Event::Exit(e));
            } else {
                let _ = self.broadcast(Event::Kick(e));
            };
        }
        None
    }

    // CONNECT 메시지 처리 -> JOIN 메시지 전송
    fn handle_connect(&self, _: RawMessage) -> Option<Vec<u8>> {
        let ret = self.formatter.format_message(MessageType::JOIN);
        Some(ret)
    }
}
