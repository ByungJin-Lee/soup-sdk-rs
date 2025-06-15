use tokio::sync::{broadcast, mpsc};

use crate::{
    Error, Result,
    chat::{
        Event,
        commands::{Command, MessageType},
        constants::message_codes,
        formatter::ChatFormatter,
        parser::{
            chat::{parse_chat_event, parse_manager_chat_event},
            emoticon::parse_emoticon_event,
            exit::parse_exit_event,
            freeze::parse_freeze_event,
            join::parse_join_event,
            kick::parse_kick_cancel_event,
            mute::parse_mute_event,
            notification::parse_notification_event,
            raw::{RawMessage, parse_message},
            slow::parse_slow_event,
        },
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
            message_codes::DISCONNECT => self.handle_disconnect(message),
            message_codes::SLOW => self.handle_slow(message),
            message_codes::KICK_CANCEL => self.handle_kick_cancel(message),
            _ => {
                // 다른 메시지 코드 처리
                let _ = self.broadcast(Event::Unknown(message.code));
                None
            }
        };

        // 메시지에 대한 응답이 필요한 경우, Vec<u8>를 반환합니다.
        res
    }

    fn handle_slow(&self, message: RawMessage) -> Option<Vec<u8>> {
        let _ = self.broadcast(Event::Slow(parse_slow_event(message)));
        None
    }

    fn handle_disconnect(&self, _: RawMessage) -> Option<Vec<u8>> {
        let _ = self.command_tx.try_send(Command::Shutdown);
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
