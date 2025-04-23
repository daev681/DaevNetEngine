// src/net/tcp/listener.rs
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub async fn start_tcp_server(ip: &str, port: u16) {
    let listener = TcpListener::bind(format!("{}:{}", ip, port)).await.unwrap();
    println!("Listening on {}:{}", ip, port);

    loop {
        let (mut socket, _) = listener.accept().await.unwrap();
        println!("New connection established.");

        tokio::spawn(async move {
            let mut buffer = [0u8; 1024];
            loop {
                let n = match socket.read(&mut buffer).await {
                    Ok(0) => break,
                    Ok(n) => n,
                    Err(_) => break,
                };

                if socket.write_all(&buffer[0..n]).await.is_err() {
                    break;
                }
            }
        });
    }
}
