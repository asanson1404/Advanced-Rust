use tokio::net::TcpStream;

pub async fn tcp_ping(host: &str, port: u16) -> bool {
    let stream = TcpStream::connect(&format!("{host}:{port}")).await;
    if stream.is_err() {return false;}
    true
}