// --- WebSocket 프로토콜 구분자 ---
pub const STARTER: &str = "\x1b\t";
pub const SEPARATOR: &str = "\x0c";
pub const ELEMENT_START: &str = "\x11";
pub const ELEMENT_END: &str = "\x12";
pub const SPACE: &str = "\x06";

// --- 채팅 명령 코드 ---
pub mod message_codes {
    pub const PING: &str = "0000";
    pub const CONNECT: &str = "0001";
    pub const ENTERCHATROOM: &str = "0002";
    pub const EXIT: &str = "0004";
    pub const CHAT: &str = "0005";
    pub const DISCONNECT: &str = "0007";
    pub const ENTER_INFO: &str = "0012";
    pub const TEXTDONATION: &str = "0018";
    pub const ADBALLOONDONATION: &str = "0087";
    pub const SUBSCRIBE: &str = "0093";
    pub const NOTIFICATION: &str = "0104";
    pub const EMOTICON: &str = "0109";
    pub const VIDEODONATION: &str = "0105";
    pub const VIEWER: &str = "0127";
}
