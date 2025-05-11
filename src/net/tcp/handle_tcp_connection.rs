use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::time::{timeout, Duration};
use serde_json;
use std::net::SocketAddr;
use std::sync::Arc;

use crate::auth::auth_handler::AuthService;
use crate::auth::auth_packet::{AuthRequest, AuthResponse};
use crate::auth::session::SessionManager;

const TOKEN_SIZE: usize = 36; // 상수화

pub async fn handle_tcp_connection<S>(
    mut socket: S,
    addr: SocketAddr,
    auth_service: Arc<AuthService>,
    session_manager: Arc<SessionManager>,
)
where
    S: AsyncRead + AsyncWrite + Unpin + Send + 'static,
{
    let mut buffer = [0u8; 4096];

    // 🧩 Step 1: 인증 요청 읽기
    let n = match timeout(Duration::from_secs(10), socket.read(&mut buffer)).await {
        Ok(Ok(0)) | Ok(Err(_)) | Err(_) => {
            println!("[TCP] Authentication timeout or error from {:?}", addr);
            return;
        }
        Ok(Ok(n)) => n,
    };

    // 🧩 Step 2: JSON 파싱
    let auth_req: Result<AuthRequest, _> = serde_json::from_slice(&buffer[..n]);
    let auth_req = match auth_req {
        Ok(req) => req,
        Err(_) => {
            println!("[TCP] Invalid auth request format from {:?}", addr);
            let fail_response = AuthResponse {
                success: false,
                token: None,
                message: Some("Invalid request format".to_string()),
            };
            if let Ok(json) = serde_json::to_vec(&fail_response) {
                let _ = socket.write_all(&json).await;
            }
            return;
        }
    };

    // 🧩 Step 3: 인증 검증
    let auth_res = auth_service.authenticate(auth_req.clone());

    if !auth_res.success {
        println!("[TCP] Authentication failed for {:?}", addr);
        if let Ok(json) = serde_json::to_vec(&auth_res) {
            let _ = socket.write_all(&json).await;
        }
        return;
    }

    println!("[TCP] Authentication success for {:?}", addr);

    // 세션 등록
    if let Some(token) = &auth_res.token {
        session_manager.insert(token.clone(), auth_req.username.clone()).await;
    }

    // 인증 성공 응답
    if let Ok(json) = serde_json::to_vec(&auth_res) {
        if socket.write_all(&json).await.is_err() {
            eprintln!("[TCP] Failed to send auth success to {:?}", addr);
            return;
        }
    }

    // 🧩 Step 4: 인증 이후 통신
    loop {
        let n = match timeout(Duration::from_secs(30), socket.read(&mut buffer)).await {
            Ok(Ok(0)) | Ok(Err(_)) | Err(_) => {
                println!("[TCP] Connection closed or timed out {:?}", addr);
                break;
            }
            Ok(Ok(n)) => n,
        };

        if n < TOKEN_SIZE {
            eprintln!("[TCP] Packet too small from {:?}", addr);
            break;
        }

        let (token_bytes, payload) = buffer[..n].split_at(TOKEN_SIZE);
        let token = String::from_utf8_lossy(token_bytes);

        if !session_manager.validate(&token).await {
            println!("[TCP] Invalid or expired token from {:?}", addr);
            break;
        }

        // 여기선 단순 echo
        if socket.write_all(payload).await.is_err() {
            eprintln!("[TCP] Failed to write to {:?}", addr);
            break;
        }
    }
}
