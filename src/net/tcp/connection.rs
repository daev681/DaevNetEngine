use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use crate::util::buffer_pool::BUFFER_POOL;

pub async fn handle_connection(mut socket: TcpStream) {
    println!("클라이언트 연결됨");

    let mut buf = BUFFER_POOL.get_buf();
    loop {
        let n = match socket.read(&mut buf).await {
            Ok(0) => break, // 연결 종료
            Ok(n) => n,
            Err(_) => break,
        };

        if socket.write_all(&buf[..n]).await.is_err() {
            break;
        }
    }

    BUFFER_POOL.release_buf(buf);
}
