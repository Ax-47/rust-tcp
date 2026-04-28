use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let socket = TcpListener::bind("127.0.0.1:8080").await?;
    loop {
        let (mut soc, from) = socket.accept().await?;
        println!("{}", from);
        tokio::spawn(async move {
            let mut buf = [0; 1024];
            loop {
                let n = match soc.read(&mut buf).await {
                    Ok(0) => return,
                    Ok(n) => n,
                    Err(_) => return,
                };
                println!("Received: {}", String::from_utf8_lossy(&buf[..n]));

                if soc.write(&buf[..n]).await.is_err() {
                    return;
                }
            }
        });
    }
}
