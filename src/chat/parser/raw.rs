use chrono::{DateTime, Utc};

use crate::chat::constants::{SEPARATOR_U8, message_codes::MessageCode};

#[derive(Debug)]
pub struct RawMessage {
    pub code: MessageCode,
    pub red_code: u32,
    pub body: Vec<String>,
    pub received_time: DateTime<Utc>,
}

struct MessageHeader {
    code: MessageCode,
    ret_code: u32,
}

pub fn parse_message(data: &[u8]) -> Result<RawMessage, String> {
    let now = Utc::now();

    let header_bytes = &data[0..14];

    let header = parse_header(header_bytes)?;

    let body = &data[14..];

    Ok(RawMessage {
        code: header.code,
        red_code: header.ret_code,
        body: parse_body(body),
        received_time: now,
    })
}

fn parse_header(header: &[u8]) -> Result<MessageHeader, String> {
    if header.len() != 14 {
        return Err("Invalid header length".to_string());
    }

    Ok(MessageHeader {
        code: parse_bytes_to_u32(&header[2..6]),
        ret_code: parse_bytes_to_u32(&header[12..14]),
    })
}

fn parse_body(body: &[u8]) -> Vec<String> {
    // 입력 데이터가 너무 짧으면(헤더만 있거나 비어있으면) 빈 벡터를 반환합니다.
    if body.len() < 2 {
        return Vec::new();
    }

    // 첫 번째 바이트를 건너뛴 슬라이스를 가져옵니다.
    let data_to_process = &body[1..];

    // 데이터를 구분자로 분할하고, 각 조각을 문자열로 변환한 뒤, 벡터로 수집합니다.
    data_to_process
        .split(|&byte| byte == SEPARATOR_U8)
        .map(|byte_part| {
            // String::from_utf8_lossy는 유효하지 않은 UTF-8 시퀀스를
            // 문자로 대체하여 안전하게 문자열을 생성합니다.
            String::from_utf8_lossy(byte_part).into_owned()
        })
        .collect()
}

fn parse_bytes_to_u32(bytes: &[u8]) -> u32 {
    String::from_utf8_lossy(bytes)
        .to_string()
        .parse::<u32>()
        .unwrap_or(0)
}
