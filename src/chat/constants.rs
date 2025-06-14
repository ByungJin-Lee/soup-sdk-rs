// --- WebSocket 프로토콜 구분자 ---
pub const STARTER: &str = "\x1b\t";
pub const STARTER_VEC: &[u8] = &[27, 9]; // ESC + TAB
pub const SEPARATOR: char = '\x0c';
pub const SEPARATOR_U8: u8 = 12; // Form Feed (FF)
pub const ELEMENT_START: char = '\x11';
pub const ELEMENT_END: char = '\x12';
pub const SPACE: char = '\x06';

// --- 채팅 명령 코드 ---
pub mod message_codes {
    pub type MessageCode = u32;
    pub const PING: MessageCode = 0;
    pub const CONNECT: MessageCode = 1;
    pub const JOIN: MessageCode = 2;
    pub const EXIT: MessageCode = 4;
    pub const CHAT: MessageCode = 5;
    pub const DISCONNECT: MessageCode = 7;
    pub const ENTER_INFO: MessageCode = 12;
    pub const TEXTDONATION: MessageCode = 18;
    pub const ADBALLOONDONATION: MessageCode = 87;
    pub const SUBSCRIBE: MessageCode = 93;
    pub const NOTIFICATION: MessageCode = 104;
    pub const EMOTICON: MessageCode = 109;
    pub const VIDEODONATION: MessageCode = 105;
    pub const VIEWER: MessageCode = 127;
}

pub mod chat_message_fields {
    pub const CONTENT: usize = 0;
    pub const USER_ID: usize = 1;
    pub const USER_NICK: usize = 5;
    pub const FLAGS: usize = 6;
    pub const SUBSCRIBE: usize = 7;
    pub const ACC_SUBSCRIBE: usize = 10;
}

pub mod user_flags {
    pub const BJ: u32 = 1 << 2;
    pub const GUEST: u32 = 1 << 4;
    pub const FAN: u32 = 1 << 5;
    pub const MANAGER: u32 = 1 << 8;
    pub const TOP_FAN: u32 = 1 << 15;
    pub const SUPPORTER: u32 = 1 << 20;
    pub const FOLLOWER_TIER1: u32 = 1 << 18;
    pub const FOLLOWER_TIER2: u32 = 1 << 19;
}
