use tokio::net::{TcpStream, lookup_host};
use tokio::time::timeout;
use std::time::Duration;
use crate::error::Error;
use futures::stream;
use futures::stream::{StreamExt};

/*
    Function which test if it's possible to connect to host from a specified port
    Returns Ok(true) if it's possible, Ok(false) if not.
    The fonction returns an Error if the host or the port don't exist  
*/
async fn tcp_ping(host: &str, port: u16) -> Result<bool, Error> {
    
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

/*
    Function which try to connect to a list of host from a list of port
    Process a parallel call to tcp_ping
*/
async fn tcp_ping_many<'a>(targets: &[(&'a str, u16)]) -> Vec<(&'a str, u16, Result<bool, Error>)> {

    let ret: Vec<(&'a str, u16, Result<bool, Error>)> = stream::iter(targets)
                                                        .map(|arg| async { 
                                                           (arg.0, arg.1, tcp_ping(arg.0, arg.1).await) 
                                                        })
                                                        .buffer_unordered(30)
                                                        .collect().await;

    ret
}

/*
    Function which call tcp_ping_many
    It just transforms of list of hosts and a list of ports to a list of (host, port)
*/
pub async fn tcp_mping<'a>(targets: &[&'a str], ports: &[u16]) -> Vec<(&'a str, u16, Result<bool, Error>)> {

    let mut hp_vec: Vec<(&'a str, u16)> = Vec::new();

    for target in targets {
        for port in ports {
            hp_vec.push((*target, *port));
        }
    }

    tcp_ping_many(&hp_vec).await
}