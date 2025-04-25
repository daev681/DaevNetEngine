// src/handler/udp_connection_handler.rs
use tokio::net::UdpSocket;
use std::net::SocketAddr;

pub async fn handle_udp_connection(socket: UdpSocket, addr: SocketAddr) {
    let mut buf = [0u8; 1024];
    
    // 수신 대기
    let len = socket.recv_from(&mut buf).await.unwrap();
    println!("[UDP Handler] Received from {:?}", addr);
    
    // 응답 처리
    socket.send_to(&buf[0..len], addr).await.unwrap();
}
