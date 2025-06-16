use std::sync::Arc;

use soop_chat_sdk::{
    SoopHttpClient,
    chat::{Event, SoopChatConnection, options::SoopChatOptions},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("--- SOOP Chat Client Example ---");

    // --- 1. 의존성 생성 ---
    let soop_client = Arc::new(SoopHttpClient::new());
    let options = SoopChatOptions {
        streamer_id: "yangdoki".to_string(),
    };

    // --- 2. 초기화 (생성) ---
    // 이 시점에서는 아무런 네트워크 활동도 일어나지 않습니다.
    println!("[System] Chat connection object created and initialized.");
    let chat_connection = SoopChatConnection::new(Arc::clone(&soop_client), options)?;

    // --- 3. 이벤트 구독 준비 ---
    // start()를 호출하기 전에도 구독은 가능합니다.
    let mut event_receiver = chat_connection.subscribe();
    println!("[System] Subscribed to event channel.");

    // --- 4. 동작 시작 ---
    // start()를 호출하는 순간, 백그라운드에서 연결 시도가 시작됩니다.
    if let Err(e) = chat_connection.start().await {
        eprintln!("[Error] Failed to start a connection loop: {}", e);
        return Ok(());
    }
    println!("[System] Connection loop started. Waiting for events...");

    // --- 5. 메인 이벤트 루프 ---
    // 이제부터 이벤트를 수신하고 처리합니다.
    loop {
        match event_receiver.recv().await {
            Ok(event) => handle_event(event),
            Err(_) => {
                break;
            }
        }
    }

    Ok(())
}

fn handle_event(event: Event) {
    match event {
        Event::Chat(e) => {
            println!("채팅     {:<10} {}", e.user.label, e.comment)
        }
        Event::Join(v) => {
            // println!("{:?}", v)
        }
        Event::Disconnected => {
            println!("정상 종료됨");
        }
        Event::Donation(d) => {
            println!("별풍선    {} {}", d.from_label, d.amount)
        }
        Event::Freeze(e) => {
            println!("얼리기    {:?}", e)
        }
        Event::Mute(e) => {
            println!("채팅 금지 {:?}", e);
        }
        Event::Slow(e) => {
            println!("슬로우    {:?}", e)
        }
        Event::Notification(e) => {
            println!("공지      {:?}", e)
        }
        Event::KickCancel(e) => {
            println!("취소      {:?}", e)
        }
        Event::MissionDonation(e) => {
            println!("미션풍    {:?}", e)
        }
        Event::MissionTotal(e) => {
            println!("미션전체  {:?}", e)
        }
        Event::BattleMissionResult(e) => {
            println!("배틀결과  {:?}", e)
        }
        Event::ChallengeMissionResult(e) => {
            println!("도전결과  {:?}", e)
        }
        // Event::Exit(v) => {
        //     println!("E {}", v.user.id)
        // }
        _ => {
            // println!("[Incoming] {:?}", event);
        }
    }
}
