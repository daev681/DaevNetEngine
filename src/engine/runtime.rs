use crate::net::tcp::listener::start_tcp_server;
use crate::config::config::Config;

pub async fn run_server() {
    let config = Config::load_config();
    println!("서버 시작! IP: {}, PORT: {}", config.server_ip, config.server_port);
    start_tcp_server(&config.server_ip, config.server_port).await;
}