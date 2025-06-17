use crate::chat::constants::message_codes;

// --- 채팅 명령어 타입 ---
// format_message에서 사용됩니다.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MessageType {
    // 구현
    Ping,
    Connect,
    JOIN,
    Unknown,
}

impl From<u32> for MessageType {
    fn from(code: u32) -> Self {
        match code {
            message_codes::PING => Self::Ping,
            message_codes::CONNECT => Self::Connect,
            message_codes::JOIN => Self::JOIN,
            // 알 수 없는 명령어는 Unknown으로 처리합니다.
            _ => Self::Unknown,
        }
    }
}

impl MessageType {
    pub fn to_code(&self) -> u32 {
        match self {
            Self::Ping => message_codes::PING,
            Self::Connect => message_codes::CONNECT,
            Self::JOIN => message_codes::JOIN,
            Self::Unknown => 0, // 알 수 없는 명령어는 0으로 처리
        }
    }
}

/// 외부에서 백그라운드 연결 루프로 보내는 명령.
#[derive(Debug)]
pub enum Command {
    /// 채팅 메시지를 전송하라는 명령.
    SendChat(String),
    /// 모든 연결을 종료하고 태스크를 중단하라는 명령.
    Shutdown,
}
