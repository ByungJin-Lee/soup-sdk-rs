use thiserror::Error;

// 모든 아이템을 pub로 만들어 다른 모듈에서 가져다 쓸 수 있게 합니다.
#[derive(Debug, Error)]
pub enum Error {
    #[error("HTTP 요청 실패: {0}")]
    Request(#[from] reqwest::Error),

    #[error("WebSocket 연결 실패: {0}")]
    ConnectionFailed(String),

    #[error("WebSocket 통신 오류: {0}")]
    WebSocket(#[from] tokio_tungstenite::tungstenite::Error),

    #[error("JSON 파싱 실패: {0}")]
    Json(reqwest::Error),

    #[error("내부 채널 통신 오류: {0}")]
    InternalChannel(String),

    #[error("방송이 꺼져있습니다.")]
    StreamOffline,

    #[error("잘못된 URL 형식: {0}")]
    URLParse(#[from] url::ParseError),

    #[error("아직 구현되지 않은 기능입니다.")]
    NotImplemented,
}

pub type Result<T> = std::result::Result<T, Error>;
