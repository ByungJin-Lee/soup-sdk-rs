// --- 설정 옵션 구조체 ---
#[derive(Clone, Debug)]
pub struct SoopChatOptions {
    pub streamer_id: String,
    pub password: String,
    // 로그인 정보는 선택 사항이므로 Option으로 감쌉니다.
    // pub login: Option<SoopLoginOptions>,
    // pub base_urls: Option<SoopAPIBaseUrls>,
}
