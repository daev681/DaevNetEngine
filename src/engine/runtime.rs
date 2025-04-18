use crate::net::tcp::listener::start_tcp_server;

pub async fn run_server() {
    println!("서버 시작!");
    start_tcp_server("127.0.0.1:8080").await;
}