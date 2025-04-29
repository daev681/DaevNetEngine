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

    let cert_file = &mut BufReader::new(File::open(cert_file)?);
    let key_file = &mut BufReader::new(File::open(key_file)?);
    let cert_chain = rustls_pemfile::certs(cert_file)?.into_iter().map(rustls::Certificate).collect();
    let mut keys = rustls_pemfile::pkcs8_private_keys(key_file)?;
    let private_key = rustls::PrivateKey(keys.remove(0));

    let config = ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth()
        .with_single_cert(cert_chain, private_key)?;
    let tls_acceptor = Arc::new(TlsAcceptor::from(Arc::new(config)));

    let auth_service = Arc::new(AuthService::new());
    let session_manager = Arc::new(SessionManager::new());

    let connection_limiter = Arc::new(Semaphore::new(100));

    loop {
        let permit = match connection_limiter.clone().acquire_owned().await {
            Ok(p) => p,
            Err(_) => {
                eprintln!("[TLS TCP] Failed to acquire connection permit");
                continue;
            }
        };

        let (socket, addr) = listener.accept().await?;
        let tls_acceptor = tls_acceptor.clone();
        let auth_service = auth_service.clone();
        let session_manager = session_manager.clone();

        tokio::spawn(async move {
            let _permit = permit;

            match tls_acceptor.accept(socket).await {
                Ok(tls_stream) => {
                    super::handler::handle_tcp_connection(
                        tls_stream,
                        addr,
                        auth_service,
                        session_manager,
                    ).await;
                }
                Err(e) => {
                    eprintln!("[TLS TCP] TLS accept error: {:?}", e);
                }
            }
        });
    }
}


pub async fn start_tcp_server(ip: &str, port: u16) -> tokio::io::Result<()> {
    let listener = TcpListener::bind(format!("{}:{}", ip, port)).await?;
    println!("[TCP] Listening on {}:{}", ip, port);

    let connection_limiter = Arc::new(Semaphore::new(100));

    // ✅ AuthService, SessionManager 생성
    let auth_service = Arc::new(AuthService::new());
    let session_manager = Arc::new(SessionManager::new());

    loop {
        let permit = match connection_limiter.clone().acquire_owned().await {
            Ok(p) => p,
            Err(_) => {
                eprintln!("[TCP] Failed to acquire connection permit");
                continue;
            }
        };

        let (socket, addr) = listener.accept().await?;
        println!("[TCP] New connection from {:?}", addr);

        let auth_service = auth_service.clone();
        let session_manager = session_manager.clone();

        tokio::spawn(async move {
            let _permit = permit;
            super::handler::handle_tcp_connection(socket, addr, auth_service, session_manager).await;
        });
    }
}