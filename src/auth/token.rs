use std::time::{SystemTime, UNIX_EPOCH};

pub fn generate_token(username: &str) -> String {
    // 실제로는 JWT 등 사용, 여기선 단순 문자열
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    format!("{}_{}", username, timestamp)
}

pub fn validate_token(token: &str) -> bool {
    // 실제 검증 로직은 토큰 파싱 및 만료 체크 등 추가
    token.contains('_') // 매우 단순한 예시
}
