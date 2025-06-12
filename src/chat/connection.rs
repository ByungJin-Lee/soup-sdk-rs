use super::commands::Command;
use super::events::{Event, ReconnectingEvent, RestoredEvent};
use super::options::SoopChatOptions;
use crate::SoopHttpClient;
use crate::chat::constants;
use crate::chat::events::EventMeta;
use crate::error::{Error, Result};
use crate::models::LiveDetail;
use chrono::Utc;
use futures_util::{SinkExt, StreamExt, stream::SplitSink};
use reqwest::header::HeaderValue;
use std::sync::Arc;
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::sync::{broadcast, mpsc};
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use tokio_tungstenite::{
    MaybeTlsStream, WebSocketStream, connect_async, tungstenite::protocol::Message,
};
use url::Url;

/// SOOP 채팅 서버와의 실시간 연결을 관리하고,
/// 자동 재연결을 포함한 모든 생명주기를 책임지는 핵심 구조체입니다.
pub struct SoopChatConnection {
    client: Arc<SoopHttpClient>,         // HTTP 클라이언트 (SOOP API 호출용)
    command_tx: mpsc::Sender<Command>,   // 명령을 보내는 채널
    command_rx: mpsc::Receiver<Command>, // 명령을 받는 채널
    event_tx: broadcast::Sender<Event>,  // 이벤트를 방송하는 채널
    options: SoopChatOptions,            // 채팅 옵션 (스트리머 ID 등)
}

// --- 내부 상태 관리용 구조체 ---
struct ConnectionLoopState {
    command_rx: mpsc::Receiver<Command>,
    command_tx: mpsc::Sender<Command>,
    event_tx: broadcast::Sender<Event>,
    options: SoopChatOptions,
    client: Arc<SoopHttpClient>, // HTTP 클라이언트 핸들
}

impl SoopChatConnection {
    /// 새로운 SOOP 채팅 연결을 시작합니다.
    pub async fn new(
        soop_http_client: Arc<SoopHttpClient>,
        options: SoopChatOptions,
    ) -> Result<Self> {
        // 1. 통신 채널 생성
        // command 채널: 여러 곳에서 명령을 보낼 수 있지만, 받는 곳은 하나(mpsc)
        let (command_tx, command_rx) = mpsc::channel(32);
        // event 채널: 보내는 곳은 하나지만, 여러 곳에서 구독하여 들을 수 있음(broadcast)
        let (event_tx, _) = broadcast::channel(128);
        // 2. 사용자가 제어할 수 있는 핸들만 반환
        Ok(Self {
            command_tx,
            command_rx,
            event_tx,
            client: soop_http_client,
            options,
        })
    }

    pub async fn start(&self) {
        // tokio::spawn은 이 코드를 현재 스레드를 막지 않고 별도로 실행시킵니다.
        tokio::spawn(run_connection_loop(self.make_connection_loop_state()));
    }

    pub fn make_connection_loop_state(&self) -> ConnectionLoopState {
        ConnectionLoopState {
            command_rx: self.command_rx,
            command_tx: self.command_tx.clone(),
            event_tx: self.event_tx.clone(),
            options: self.options.clone(),
            client: self.client.clone(),
        }
    }

    /// 라이브러리가 방송하는 이벤트를 수신할 "수신기"를 얻습니다.
    pub fn subscribe(&self) -> broadcast::Receiver<Event> {
        self.event_tx.subscribe()
    }

    /// 채팅 메시지 전송 명령을 보냅니다.
    pub async fn send_chat(&self, message: String) -> Result<()> {
        self.command_tx
            .send(Command::SendChat(message))
            .await
            .map_err(|e| Error::InternalChannel(e.to_string()))
    }

    /// 연결 종료 명령을 보냅니다.
    pub async fn disconnect(&self) -> Result<()> {
        self.command_tx
            .send(Command::Shutdown)
            .await
            .map_err(|e| Error::InternalChannel(e.to_string()))
    }
}

// --- 메인 로직 ---
async fn run_connection_loop(mut state: ConnectionLoopState) {
    let mut attempts = 0u32;

    loop {
        // "작업자"에게 단 한 번의 세션 시도를 맡깁니다.
        let session_result = try_connect_and_run_session(&mut state, attempts).await;

        // 세션 결과를 바탕으로 다음 행동을 결정합니다.
        match session_result {
            // 세션이 정상적으로 종료(Shutdown)되면, 메인 루프를 완전히 빠져나갑니다.
            Ok(_) => {
                state.event_tx.send(Event::Disconnected).ok();
                break;
            }
            // 방송이 꺼져있는 상태라면, 더 긴 주기로 대기합니다.
            Err(Error::StreamOffline) => {
                tokio::time::sleep(Duration::from_secs(30)).await;
                // 이 경우는 '실패'가 아니므로 attempts를 증가시키지 않을 수 있습니다.
            }
            // 그 외 모든 에러(네트워크, WebSocket 등)는 재연결을 시도합니다.
            Err(_) => {
                attempts += 1;
                let wait_time = 5u64; // 고정 대기 시간
                // 재연결 이벤트를 생성합니다.
                let event = Event::Reconnecting(ReconnectingEvent {
                    meta: EventMeta {
                        received_time: Utc::now(),
                    },
                    attempt: attempts,
                    wait_seconds: wait_time,
                });
                // 재연결 이벤트를 방송합니다.
                if state.event_tx.send(event).is_err() {
                    break;
                }
                // 재연결을 시도하기 전에 잠시 대기합니다.
                tokio::time::sleep(Duration::from_secs(wait_time)).await;
            }
        }
    }
}

/// 한 번의 완전한 연결 세션을 시도하고, 성공 또는 실패를 반환합니다.
/// Ok(())는 정상적인 종료(Shutdown)를 의미합니다.
async fn try_connect_and_run_session(state: &mut ConnectionLoopState, attempts: u32) -> Result<()> {
    // 1. HTTP API로 최신 방송 정보 가져오기
    let live_detail = state
        .client
        .get_live_details(&state.options.streamer_id)
        .await?;
    // 방송이 꺼져있다면, 에러를 반환합니다.
    if !live_detail.is_streaming() {
        return Err(Error::StreamOffline);
    }

    // 2. WebSocket 접속 URL 생성
    let url = Url::parse(&format!(
        "wss://{}:{}/Websocket/{}",
        live_detail.channel.ch_domain.to_lowercase(),
        live_detail.channel.ch_pt + 1,
        state.options.streamer_id
    ))?;

    let mut request = url.into_client_request()?;
    request
        .headers_mut()
        .insert("Sec-WebSocket-Protocol", HeaderValue::from_static("chat"));

    // WebSocket 연결 시도
    let (ws_stream, _) = connect_async(request)
        .await
        .map_err(|e| Error::ConnectionFailed(e.to_string()))?;

    // 3. 연결 성공 이벤트 생성
    let event = if attempts == 0 {
        Event::Connected
    } else {
        Event::Restored(RestoredEvent {
            meta: EventMeta {
                received_time: Utc::now(),
            },
            restored_at: Utc::now(),
        })
    };
    // 이벤트 전송
    state
        .event_tx
        .send(event)
        .map_err(|_| Error::InternalChannel("Event channel closed".into()))?;

    let (mut writer, mut reader) = ws_stream.split();

    // 4. 초기 패킷 전송 (CONNECT)
    let connect_packet = format_connect_packet();
    writer.send(Message::Text(connect_packet)).await?;

    // 5. 실제 통신을 위임하고, 그 결과를 그대로 반환
    run_communication_loop(state, live_detail, &mut reader, &mut writer).await
}

/// 연결이 활성화된 동안, 메시지/명령/핑을 처리하는 내부 루프.
/// Ok(())는 정상적인 종료(Shutdown)를 의미하고, Err는 연결 단절을 의미합니다.
async fn run_communication_loop(
    state: &mut ConnectionLoopState,
    live_detail: LiveDetail,
    reader: &mut (impl StreamExt<Item = Result<Message, tokio_tungstenite::tungstenite::Error>> + Unpin),
    writer: &mut (impl SinkExt<Message, Error = tokio_tungstenite::tungstenite::Error> + Unpin),
) -> Result<()> {
    let mut ping_interval = tokio::time::interval(Duration::from_secs(60));

    loop {
        tokio::select! {
            // WebSocket 메시지 수신
            Some(msg_result) = reader.next() => {
                let message = msg_result?; // 에러 발생 시 '?'가 Err를 반환하여 루프 종료

                if let Ok(text) = message.to_text() {
                    // CONNECT 응답을 받으면 JOIN 패킷 전송
                    if &text[2..6] == constants::type_codes::CONNECT {
                        let join_packet = format_join_packet(&live_detail);
                        writer.send(Message::Text(join_packet)).await?;
                    }

                    if let Some(event) = handle_message(text) {
                        state.event_tx.send(event).map_err(|_| Error::InternalChannel("...".into()))?;
                    }
                }
            },
            // 사용자 커맨드 수신
            Some(command) = state.command_rx.recv() => {
                match command {
                    Command::SendChat(text) => {
                        let packet = format_chat_packet(&text);
                        writer.send(Message::Text(packet)).await?;
                    },
                    Command::Shutdown => {
                        // 정상 종료 신호이므로 Ok(())를 반환
                        return Ok(());
                    }
                }
            },
            // 주기적인 Ping 전송
            _ = ping_interval.tick() => {
                let ping_packet = format_packet(constants::type_codes::PING, constants::SEPARATOR);
                writer.send(Message::Text(ping_packet)).await?;
            }
        }
    }
}

// --- 헬퍼 함수들 ---

/// 수신된 명령을 처리하고, 루프를 계속할지 여부를 반환합니다.
async fn handle_command(
    command: Command,
    writer: &mut SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
) -> bool {
    match command {
        Command::SendChat(text) => {
            let packet = format_chat_packet(&text);
            if writer.send(Message::Text(packet)).await.is_err() {
                // 전송 실패는 곧 연결이 끊어졌음을 의미합니다.
                return false; // 통신 루프 중단 신호
            }
        }
        Command::Shutdown => {
            return false; // 메인 루프 중단 신호
        }
    }
    true // 루프 계속 진행
}

/// 원시 메시지를 의미있는 Event로 파싱합니다.
fn handle_message(msg: Message) -> Option<Event> {
    // ... 파싱 로직 ...
    None // 임시
}

/// 전송할 채팅 텍스트를 서버 프로토콜에 맞는 패킷으로 포맷팅합니다.
fn format_chat_packet(text: &str) -> String {
    // ... 포맷팅 로직 ...
    text.to_string() // 임시
}
