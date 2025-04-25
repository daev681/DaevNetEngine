// src/handler/connection_handler.rs
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::time::{timeout, Duration};

pub async fn handle_tcp_connection(mut socket: TcpStream, addr: std::net::SocketAddr) {
    let mut buffer = [0u8; 1024];
    
    loop {
        let n = match timeout(Duration::from_secs(30), socket.read(&mut buffer)).await {
            Ok(Ok(0)) | Ok(Err(_)) | Err(_) => {
                println!("[TCP] Connection with {:?} closed or timed out", addr);
                break;
            }
            Ok(Ok(n)) => n,
            Err(_) => break,
        };

        if socket.write_all(&buffer[0..n]).await.is_err() {
            eprintln!("[TCP] Failed to write to socket for {:?}", addr);
            break;
        }
    }
}
