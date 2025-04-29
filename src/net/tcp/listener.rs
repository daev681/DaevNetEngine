// src/net/tcp/listener.rs
use tokio::net::TcpListener;
use tokio::sync::Semaphore;
use tokio::time::{timeout, Duration};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use rustls::ServerConfig;
use tokio_rustls::TlsAcceptor;
use std::sync::Arc;
use std::fs::File;
use std::io::BufReader;
use rustls::PrivateKey;
use rustls::Certificate;

pub async fn start_tls_tcp_server(ip: &str, port: u16, cert_file: &str, key_file: &str) -> tokio::io::Result<()> {
    let listener = TcpListener::bind(format!("{}:{}", ip, port)).await?;
    println!("[TLS TCP] Listening on {}:{}", ip, port);

    

    let cert_file = &mut BufReader::new(File::open(cert_file).unwrap());
    let key_file = &mut BufReader::new(File::open(key_file).unwrap());
    let cert_chain = rustls::certs(cert_file).unwrap();
    let private_key = rustls::private_keys(key_file).unwrap().remove(0);

    let mut config = ServerConfig::new(rustls::NoClientAuth::new());
    config.set_single_cert(cert_chain, private_key).unwrap();
    let tls_acceptor = Arc::new(TlsAcceptor::from(config));

    let connection_limiter = Arc::new(Semaphore::new(100));

    loop {
        let permit = match connection_limiter.clone().acquire_owned().await {
            Ok(p) => p,
            Err(_) => {
                eprintln!("[TCP] Failed to acquire connection permit");
                continue;
            }
        };

        let (socket, addr) = listener.accept().await?;
        let tls_stream = tls_acceptor.accept(socket).await.unwrap();
        println!("[TLS TCP] New connection from {:?}", addr);

        tokio::spawn(async move {
            let _permit = permit;
            super::handler::handle_tcp_connection(tls_stream, addr).await;
        });
    }
}


pub async fn start_tcp_server(ip: &str, port: u16) -> tokio::io::Result<()> {
    // TCP 리스너 생성
    let listener = TcpListener::bind(format!("{}:{}", ip, port)).await?;
    println!("[TCP] Listening on {}:{}", ip, port);

    // 연결 제한 세마포어 (최대 100개 연결 제한)
    let connection_limiter = Arc::new(Semaphore::new(100));

    loop {
        // 세마포어를 통해 동시 연결 수 제한
        let permit = match connection_limiter.clone().acquire_owned().await {
            Ok(p) => p,
            Err(_) => {
                eprintln!("[TCP] Failed to acquire connection permit");
                continue;
            }
        };

        // 새로운 연결 수락
        let (socket, addr) = listener.accept().await?;
        println!("[TCP] New connection from {:?}", addr);

        // 새로운 연결 처리
        tokio::spawn(async move {
            let _permit = permit; // permit 사용을 끝낼 때까지 유효하도록
            super::handler::handle_tcp_connection(socket, addr).await;
        });

       
    }
}