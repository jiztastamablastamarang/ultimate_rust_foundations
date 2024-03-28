use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

async fn client_run() {
    loop {
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        let _ =  tcp_client().await;
    }
}

async fn tcp_client() -> anyhow::Result<()> {
  let mut stream = TcpStream::connect("127.0.0.1:8123").await?;
    println!("Connected to server.");
    stream.write_all(b"Hello, world!\n").await?;
    let mut buf = vec![0u8; 1024];
    let bytes_read = stream.read(&mut buf).await?;
    println!("Received: {}", String::from_utf8_lossy(&buf[..bytes_read]));

    return Ok(());
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
   tokio::spawn(client_run()); 
    
    let listener = TcpListener::bind("127.0.0.1:8123").await?;

    loop {
        let (mut socket, address) = listener.accept().await?;
        tokio::spawn(async move {
            println!("Connection from {address:?}");
            let mut buf: Vec<u8> = Vec::with_capacity(1024);
            loop {
                let n = socket
                    .read(&mut buf)
                    .await
                    .expect("failed to read data from socket");
                if n == 0 {
                    println!("socket {address:?} closed");
                    return;
                }

                socket
                    .write_all(&buf[0..n])
                    .await
                    .expect("failed to write data to socket");
            }
        });
    }
}
