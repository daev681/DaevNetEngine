use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:8080";
    let listener = TcpListener::bind(addr).await?;
    println!("서버가 {}에서 대기 중입니다...", addr);

    loop {
        let (mut socket, _) = listener.accept().await?;
        println!("새로운 클라이언트가 연결되었습니다!");

        tokio::spawn(async move {
            let mut buf = vec![0; 1024];

            loop {
                let n = match socket.read(&mut buf).await {
                    Ok(0) => return, // 클라이언트가 연결을 종료하면
                    Ok(n) => n,
                    Err(_) => return, // 오류 발생 시 종료
                };

                if let Err(_) = socket.write_all(&buf[..n]).await {
                    return; // 오류 발생 시 종료
                }
            }
        });
    }
}
