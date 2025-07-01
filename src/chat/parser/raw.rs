use chrono::{DateTime, Utc};

use crate::chat::constants::{SEPARATOR_U8, message_codes::MessageCode};

#[derive(Debug)]
pub struct RawMessage {
    pub code: MessageCode,
    pub _red_code: u32,
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
        _red_code: header.ret_code,
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

    // 구분자 개수를 미리 세어서 Vec 용량을 할당합니다.
    let separator_count = data_to_process.iter().filter(|&&b| b == SEPARATOR_U8).count();
    let mut result = Vec::with_capacity(separator_count + 1);

    // 데이터를 구분자로 분할하고, 각 조각을 문자열로 변환한 뒤, 벡터로 수집합니다.
    for byte_part in data_to_process.split(|&byte| byte == SEPARATOR_U8) {
        // UTF-8 유효성을 먼저 확인하여 불필요한 할당을 피합니다.
        match std::str::from_utf8(byte_part) {
            Ok(s) => result.push(s.to_string()),
            Err(_) => {
                // 유효하지 않은 UTF-8인 경우에만 lossy conversion 사용
                let cow_str = String::from_utf8_lossy(byte_part);
                result.push(cow_str.into_owned());
            }
        }
    }
    
    result
}

fn parse_bytes_to_u32(bytes: &[u8]) -> u32 {
    // UTF-8 바이트를 직접 파싱하여 불필요한 String 할당을 피합니다.
    match std::str::from_utf8(bytes) {
        Ok(s) => s.parse::<u32>().unwrap_or(0),
        Err(_) => {
            // 유효하지 않은 UTF-8인 경우에만 lossy conversion 사용
            String::from_utf8_lossy(bytes).parse::<u32>().unwrap_or(0)
        }
    }
}
