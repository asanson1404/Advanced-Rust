use tokio::net::{TcpStream, lookup_host};
use crate::error::Error;

pub async fn tcp_ping(host: &str, port: u16) -> Result<bool, Error> {
    for addr in lookup_host(format!("{host}:{port}")).await? {
        let stream = TcpStream::connect(&addr).await;
        if stream.is_err() {return Ok(false);}
    }
    Ok(true)
}