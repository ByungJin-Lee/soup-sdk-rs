// --- WebSocket 프로토콜 구분자 ---
pub const STARTER: &str = "\x1b\t";
pub const STARTER_VEC: &[u8] = &[27, 9]; // ESC + TAB
pub const SEPARATOR: char = '\x0c';
pub const SEPARATOR_U8: u8 = 12; // Form Feed (FF)
pub const ELEMENT_START: char = '\x11';
pub const ELEMENT_END: char = '\x12';
pub const SPACE: char = '\x06';

// --- 사전 계산된 SEPARATOR 패턴들 ---
pub const SEPARATOR_3_TIMES: &str = "\x0c\x0c\x0c"; // SEPARATOR * 3
pub const SEPARATOR_5_TIMES: &str = "\x0c\x0c\x0c\x0c\x0c"; // SEPARATOR * 5

// --- 채팅 명령 코드 ---
pub mod message_codes {
    pub type MessageCode = u32;
    pub const PING: MessageCode = 0;
    pub const CONNECT: MessageCode = 1;
    pub const JOIN: MessageCode = 2;
    pub const EXIT: MessageCode = 4;
    pub const CHAT: MessageCode = 5;
    pub const BJ_STATE_CHANGE: MessageCode = 7;
    pub const MUTE: MessageCode = 8;
    pub const ENTER_INFO: MessageCode = 12;
    pub const FREEZE: MessageCode = 21;
    pub const SLOW: MessageCode = 23;
    pub const MANAGER_CHAT: MessageCode = 26;
    pub const SUBSCRIBE: MessageCode = 91;
    pub const SUBSCRIBE_RENEW: MessageCode = 93;
    pub const KICK_CANCEL: MessageCode = 76;
    pub const NOTIFICATION: MessageCode = 104;
    pub const EMOTICON: MessageCode = 109;
    pub const USER_JOIN: MessageCode = 127;
    // balloon
    pub const DONATION: MessageCode = 18;
    pub const SUB_DONATION: MessageCode = 33;
    pub const VOD_DONATION: MessageCode = 86;
    // ad balloon
    pub const VOD_AD_DONATION: MessageCode = 103;
    pub const ADBALLOON_DONATION: MessageCode = 87;
    pub const AD_STATION_DONATION: MessageCode = 107;
    // video balloon
    pub const VIDEO_DONATION: MessageCode = 105;
    // battle & mission
    pub const MISSION_DONATION: MessageCode = 121;
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
