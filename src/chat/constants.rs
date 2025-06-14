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
