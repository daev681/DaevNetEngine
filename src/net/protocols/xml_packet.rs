use serde::{Serialize, Deserialize};

/// 예시: 로그인 요청
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

/// 예시: 게임 채팅
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub sender_id: u32,
    pub message: String,
}