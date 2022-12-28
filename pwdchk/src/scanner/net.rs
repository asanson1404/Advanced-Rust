use tokio::net::{TcpStream, lookup_host};
use tokio::time::timeout;
use std::time::Duration;
use crate::error::Error;

pub async fn tcp_ping(host: &str, port: u16) -> Result<bool, Error> {
    
    for addr in lookup_host(format!("{host}:{port}")).await? {
        let stream = TcpStream::connect(&addr);
        let time_res = timeout(Duration::from_secs(3), stream).await;
        if time_res.is_err() {
            return Err(Error::Timeout);
        }
        else if time_res.unwrap().is_err() {
            return Ok(false);
        }
    }
    Ok(true)
}