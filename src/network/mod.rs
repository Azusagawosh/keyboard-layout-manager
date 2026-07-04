use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

pub async fn send_message(stream: &mut TcpStream, data: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    stream.write_all(data).await?;
    stream.flush().await?;
    Ok(())
}

pub async fn read_message(stream: &mut TcpStream) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut buf = vec![0; 1024];
    let n = stream.read(&mut buf).await?;
    buf.truncate(n);
    Ok(buf)
}

pub async fn start_server(addr: &str) -> Result<TcpListener, Box<dyn std::error::Error>> {
    let listener = TcpListener::bind(addr).await?;
    Ok(listener)
}

pub async fn connect_client(addr: &str) -> Result<TcpStream, Box<dyn std::error::Error>> {
    let stream = TcpStream::connect(addr).await?;
    Ok(stream)
}