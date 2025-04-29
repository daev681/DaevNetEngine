// src/handler/connection_handler.rs
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::time::{timeout, Duration};

use crate::auth::auth_handler::AuthService;
use crate::auth::auth_packet::{AuthRequest, AuthResponse};
use crate::auth::token::validate_token;
use std::net::SocketAddr;
use serde_json; // JSON 파싱용
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::time::{timeout, Duration};

use crate::auth::auth_handler::AuthService;
use crate::auth::auth_packet::{AuthRequest, AuthResponse};
use crate::auth::token::validate_token;
use std::net::SocketAddr;

use serde_json; // JSON 파싱용
/*
클라이언트에서 아래와 같이보낸다.
{
  "username": "player1",
  "password": "1234"
}

*/
pub async fn handle_tcp_connection(mut socket: TcpStream, addr: SocketAddr) {
    let mut buffer = [0u8; 1024];
    let auth_service = AuthService::new(); // 임시 유저 DB

    // 🧩 Step 1: 인증 요청 읽기
    let n = match timeout(Duration::from_secs(10), socket.read(&mut buffer)).await {
        Ok(Ok(0)) | Ok(Err(_)) | Err(_) => {
            println!("[TCP] Authentication timeout or error from {:?}", addr);
            return;
        }
        Ok(Ok(n)) => n,
    };

    // 🧩 Step 2: JSON 파싱 시도
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
    let auth_res = auth_service.authenticate(auth_req);

    if !auth_res.success {
        println!("[TCP] Authentication failed for {:?}", addr);
        if let Ok(json) = serde_json::to_vec(&auth_res) {
            let _ = socket.write_all(&json).await;
        }
        return;
    }

    // 인증 성공 응답
    println!("[TCP] Authentication success for {:?}", addr);
    if let Ok(json) = serde_json::to_vec(&auth_res) {
        if socket.write_all(&json).await.is_err() {
            eprintln!("[TCP] Failed to send auth success response to {:?}", addr);
            return;
        }
    }

    // 🧩 Step 4: 정상 통신 루프
    loop {
        let n = match timeout(Duration::from_secs(30), socket.read(&mut buffer)).await {
            Ok(Ok(0)) | Ok(Err(_)) | Err(_) => {
                println!("[TCP] Connection with {:?} closed or timed out", addr);
                break;
            }
            Ok(Ok(n)) => n,
        };

        if socket.write_all(&buffer[..n]).await.is_err() {
            eprintln!("[TCP] Failed to write to socket for {:?}", addr);
            break;
        }
    }
}

