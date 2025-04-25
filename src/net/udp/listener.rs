// src/net/udp/listener.rs
use tokio::net::UdpSocket;
use tokio::time::{timeout, Duration};
use tokio::sync::Semaphore;
use std::sync::Arc;
use rustls::ServerConfig;
use tokio_rustls::TlsAcceptor;
use std::fs::File;
use std::io::BufReader;

pub async fn start_udp_server(ip: &str, port: u16, cert_file: &str, key_file: &str) -> tokio::io::Result<()> {
    let socket = UdpSocket::bind(format!("{}:{}", ip, port)).await?;
    println!("[UDP] Listening on {}:{}", ip, port);

    let cert_file = &mut BufReader::new(File::open(cert_file).unwrap());
    let key_file = &mut BufReader::new(File::open(key_file).unwrap());
    let cert_chain = rustls::certs(cert_file).unwrap();
    let private_key = rustls::private_keys(key_file).unwrap().remove(0);

    let mut config = ServerConfig::new(rustls::NoClientAuth::new());
    config.set_single_cert(cert_chain, private_key).unwrap();
    let tls_acceptor = Arc::new(TlsAcceptor::from(config));

    let connection_limiter = Arc::new(Semaphore::new(100));

    let mut buf = [0u8; 1024];
    loop {
        let (len, addr) = match timeout(Duration::from_secs(30), socket.recv_from(&mut buf)).await {
            Ok(Ok((len, addr))) => (len, addr),
            _ => continue,
        };

        println!("[UDP] Received from {:?}", addr);

        // 데이터 처리 및 TLS 적용
        let tls_stream = tls_acceptor.accept(socket.clone()).await.unwrap();
        tokio::spawn(async move {
            super::handler::handle_udp_connection(tls_stream, addr).await;
        });

        socket.send_to(&buf[0..len], addr).await?;
    }
}
