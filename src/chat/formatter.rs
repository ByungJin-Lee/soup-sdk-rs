use crate::{
    chat::{
        commands::MessageType,
        constants::{SEPARATOR, STARTER_VEC},
    },
    models::LiveDetail,
};

/// Formatter는 채팅 메시지를 포맷하는 구조체입니다.
#[derive(Debug, Clone)]
pub struct ChatFormatter {
    pub live_detail: LiveDetail,
}

impl ChatFormatter {
    pub fn new(live_detail: LiveDetail) -> Self {
        Self { live_detail }
    }

    pub fn format_message(&self, message_type: MessageType) -> Vec<u8> {
        let payload: String = match message_type {
            MessageType::Connect => self.format_connect_packet(),
            MessageType::JOIN => self.format_join_packet(),
            _ => "".to_string(), // 다른 메시지 코드에 대한 기본값
        };
        return bundle(message_type, payload.as_bytes());
    }

    fn format_connect_packet(&self) -> String {
        format!("{}16{}", SEPARATOR.to_string().repeat(3), SEPARATOR)
    }

    fn format_join_packet(&self) -> String {
        format!(
            "{}{}{}",
            SEPARATOR,
            self.live_detail.channel.chat_no,
            SEPARATOR.to_string().repeat(5)
        )
    }
}

/// 여러 바이트 슬라이스 조각들을 하나의 새로운 Vec<u8>로 병합합니다.
fn flatten_byte_slices(parts: &[&[u8]]) -> Vec<u8> {
    let total_len = parts.iter().map(|s| s.len()).sum();
    let mut result = Vec::with_capacity(total_len);
    for part in parts {
        result.extend_from_slice(part);
    }
    result
}

/// WebSocket으로 전송할 메시지 패킷을 생성합니다.
///
/// # Arguments
/// * `message_code` - 메시지 종류를 나타내는 숫자 코드.
/// * `body` - 전송할 실제 데이터.
///
/// # Returns
/// * 프로토콜에 맞게 헤더와 본문이 결합된 `Vec<u8>`
pub fn bundle(message_type: MessageType, body: &[u8]) -> Vec<u8> {
    // --- 1. 헤더 생성 ---
    // 프로토콜에 따라 각 필드를 문자열로 포맷팅합니다.
    // JS의 padStart(len, '0')는 Rust의 format! 매크로로 쉽게 구현할 수 있습니다.
    let code_str = format!("{:04}", message_type.to_code()); // 4자리, 0으로 채움
    let body_len_str = format!("{:06}", body.len()); // 6자리, 0으로 채움
    let reserved_str = "00"; // 2자리 예약 필드

    // 헤더를 구성하는 각 부분을 바이트 슬라이스로 준비합니다.
    // b"" 문법은 문자열 리터럴을 바이트 슬라이스(&[u8])로 만듭니다.
    let header_parts: Vec<&[u8]> = vec![
        STARTER_VEC,
        code_str.as_bytes(),
        body_len_str.as_bytes(),
        reserved_str.as_bytes(),
    ];

    // 헤더 조각들을 하나의 Vec<u8>로 병합합니다.
    let mut header = flatten_byte_slices(&header_parts);

    // --- 2. 헤더와 본문 결합 ---
    // 생성된 헤더 뒤에 본문 데이터를 이어 붙입니다.
    header.extend_from_slice(body);

    // 최종적으로 완성된 메시지 패킷을 반환합니다.
    header
}
