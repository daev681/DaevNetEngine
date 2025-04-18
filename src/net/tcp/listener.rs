use tokio::net::TcpListener;
use crate::net::tcp::connection::handle_connection;

pub async fn start_tcp_server(addr: &str) {
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("Listening on {}", addr);

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            handle_connection(socket).await;
        });
    }
}