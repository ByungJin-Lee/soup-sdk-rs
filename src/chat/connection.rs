use super::commands::Command;
use super::events::Event;
use super::options::SoopChatOptions;
use crate::SoopHttpClient;
use crate::chat::commands::MessageType;
use crate::chat::formatter::ChatFormatter;
use crate::chat::message::MessageHandler;
use crate::chat::verification::NoVerification;
use crate::error::{Error, Result};
use crate::models::LiveDetail;
use futures_util::lock::Mutex;
use futures_util::stream::SplitStream;
use futures_util::{SinkExt, StreamExt, stream::SplitSink};
use reqwest::header::HeaderValue;
use rustls::ClientConfig;
use rustls::crypto::CryptoProvider;
use std::sync::Arc;
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::sync::{broadcast, mpsc};
use tokio_tungstenite::connect_async_tls_with_config;
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream, tungstenite::protocol::Message};
use url::Url;

/// SOOP 채팅 서버와의 실시간 연결을 관리하고,
/// 자동 재연결을 포함한 모든 생명주기를 책임지는 핵심 구조체입니다.
pub struct SoopChatConnection {
    client: Arc<SoopHttpClient>,       // HTTP 클라이언트 (SOOP API 호출용)
    command_tx: mpsc::Sender<Command>, // 명령을 보내는 채널
    command_rx: Mutex<Option<mpsc::Receiver<Command>>>, // 명령을 받는 채널 (Mutex로 감싸서 안전하게 공유)
    event_tx: broadcast::Sender<Event>,                 // 이벤트를 방송하는 채널
    options: SoopChatOptions,                           // 채팅 옵션 (스트리머 ID 등)
}

// --- 내부 상태 관리용 구조체 ---
struct ConnectionLoopState {
    command_rx: mpsc::Receiver<Command>,
    command_tx: mpsc::Sender<Command>,
    event_tx: broadcast::Sender<Event>,
    connection_url: String,
    live_detail: LiveDetail,
}

impl SoopChatConnection {
    /// 새로운 SOOP 채팅 연결을 시작합니다.
    pub fn new(soop_http_client: Arc<SoopHttpClient>, options: SoopChatOptions) -> Result<Self> {
        // * TLS 설정이 없는 경우에만 설정
        if CryptoProvider::get_default().is_none() {
            rustls::crypto::aws_lc_rs::default_provider()
                .install_default()
                .expect("Failed to install default crypto provider");
        }
        // 1. 통신 채널 생성
        // command 채널: 여러 곳에서 명령을 보낼 수 있지만, 받는 곳은 하나(mpsc)
        let (command_tx, command_rx) = mpsc::channel(32);
        // event 채널: 보내는 곳은 하나지만, 여러 곳에서 구독하여 들을 수 있음(broadcast)
        let (event_tx, _) = broadcast::channel(8192);
        // 2. 사용자가 제어할 수 있는 핸들만 반환
        Ok(Self {
            command_tx,
            command_rx: Mutex::new(Some(command_rx)),
            event_tx,
            client: soop_http_client,
            options,
        })
    }

    pub fn command(&self, command: Command) -> Result<()> {
        // 명령을 보내는 채널에 전송합니다.
        self.command_tx
            .try_send(command)
            .map_err(|e| Error::InternalChannel(e.to_string()))
    }

    fn make_connection_url(&self, live_detail: &LiveDetail) -> String {
        format!(
            "wss://{}:{}/Websocket/{}",
            live_detail.ch_domain.to_lowercase(),
            live_detail.ch_pt + 1,
            self.options.streamer_id
        )
    }

    pub async fn start(&self) -> Result<()> {
        // 연결 가능한 상태인지 live detail을 가져옵니다.
        let (is_live, optional_live_detail) = self
            .client
            .get_live_detail_state(&self.options.streamer_id)
            .await?;

        // 오프라인이면 종료합니다.
        if !is_live {
            return Err(Error::StreamOffline);
        } else if optional_live_detail.is_none() {
            return Err(Error::InternalChannel(
                "생방송 정보가 잘못되었습니다.".to_string(),
            ));
        }

        let live_detail = optional_live_detail.unwrap();

        // websocket url 생성
        let connection_url = self.make_connection_url(&live_detail);

        // 소유권 을 안전하게 가져오기 위해, command_rx를 잠급니다.
        let mut rx_guard = self.command_rx.lock().await;
        // 소유권을 이전합니다.
        if let Some(command_rx) = rx_guard.take() {
            let loop_state = ConnectionLoopState {
                command_tx: self.command_tx.clone(),
                command_rx,
                event_tx: self.event_tx.clone(),
                connection_url,
                live_detail,
            };
            // 백그라운드 스레드 실행
            tokio::spawn(run_connection_loop(loop_state));
            Ok(())
        } else {
            // 이 객체는 일회용임, 한번 사용하면 재사용 불가
            Err(Error::AlreadyStarted)
        }
    }

    /// 라이브러리가 방송하는 이벤트를 수신할 "수신기"를 얻습니다.
    pub fn subscribe(&self) -> broadcast::Receiver<Event> {
        self.event_tx.subscribe()
    }
}

// 장애 대응 로직은 SRP에 부합하지 않으므로 제거합니다.

// --- 메인 로직 ---
async fn run_connection_loop(mut state: ConnectionLoopState) {
    // 세션 결과를 바탕으로 다음 행동을 결정합니다.
    match try_connect_and_run_session(&mut state).await {
        // 세션이 정상적으로 종료(Shutdown)되면, 메인 루프를 완전히 빠져나갑니다.
        Ok(_) => {
            state.event_tx.send(Event::Disconnected).ok();
        }
        Err(e) => {
            // 그 외 모든 에러(네트워크, WebSocket 등)는 재연결을 시도합니다.
            print!("[System] Connection error: {:?}. Retrying...\n", e);
        }
    }
}

/// 한 번의 완전한 연결 세션을 시도하고, 성공 또는 실패를 반환합니다.
/// Ok(())는 정상적인 종료(Shutdown)를 의미합니다.
async fn try_connect_and_run_session(state: &mut ConnectionLoopState) -> Result<()> {
    // 1. WebSocket 접속 URL 생성
    let url = Url::parse(&state.connection_url)?;
    print!("[System] Attempting to connect to WebSocket: {}\n", url);

    let mut request = url.into_client_request()?;
    request
        .headers_mut()
        .insert("Sec-WebSocket-Protocol", HeaderValue::from_static("chat"));

    // 2. TLS 설정
    let config = ClientConfig::builder()
        .dangerous()
        .with_custom_certificate_verifier(Arc::new(NoVerification)) // 필드가 없는 검증기 사용
        .with_no_client_auth();

    // - WebSocket 연결 시도
    let (ws_stream, _) = connect_async_tls_with_config(
        request,
        None,
        true,
        Some(tokio_tungstenite::Connector::Rustls(Arc::new(config))),
    )
    .await
    .map_err(|e| Error::ConnectionFailed(e.to_string()))?;

    // 이벤트 전송
    state
        .event_tx
        .send(Event::Connected)
        .map_err(|_| Error::InternalChannel("Event channel closed".into()))?;

    let (mut writer, mut reader) = ws_stream.split();

    // Formatter 인스턴스 생성
    let formatter = ChatFormatter::new(state.live_detail.clone());

    // 4. 초기 패킷 전송 (CONNECT)
    let connect_packet = formatter.format_message(MessageType::Connect);
    writer.send(Message::Binary(connect_packet)).await?;

    // 5. 실제 통신을 위임하고, 그 결과를 그대로 반환
    run_communication_loop(state, &mut reader, &mut writer, &formatter).await
}

/// 연결이 활성화된 동안, 메시지/명령/핑을 처리하는 내부 루프.
/// Ok(())는 정상적인 종료(Shutdown)를 의미하고, Err는 연결 단절을 의미합니다.
async fn run_communication_loop(
    state: &mut ConnectionLoopState,
    reader: &mut SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    writer: &mut SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
    formatter: &ChatFormatter,
) -> Result<()> {
    let mut ping_interval = tokio::time::interval(Duration::from_secs(60));

    let handler = MessageHandler::new(formatter, state.event_tx.clone(), state.command_tx.clone());

    loop {
        tokio::select! {
            // WebSocket 메시지 수신
            Some(msg_result) = reader.next() => {
                let raw = msg_result?.into_data(); // 에러 발생 시 '?'가 Err를 반환하여 루프 종료
                if let Some(resp) = handler.handle(raw)? {
                    // Handle the response if needed, or remove this block if not used
                    writer.send(Message::Binary(resp)).await?;
                }
            },
            // 사용자 커맨드 수신
            Some(command) = state.command_rx.recv() => {
                match command {
                    Command::Shutdown => {
                        // 정상 종료 신호이므로 Ok(())를 반환
                        return Ok(());
                    }
                    _ => {
                        // 다른 명령어는 모두 처리하지 않는다
                    }
                }
            },
            // 주기적인 Ping 전송
            _ = ping_interval.tick() => {
                let msg = formatter.format_message(MessageType::Ping);
                writer.send(Message::Binary(msg)).await?;
            }
        }
    }
}
