use tokio::net::{TcpStream, lookup_host};
use tokio::io::AsyncBufReadExt;
use tokio::io::BufReader;
use tokio::time::timeout;
use std::str::FromStr;
use std::time::Duration;
use crate::error::Error;
use futures::stream;
use futures::stream::{StreamExt};
use ipnet::Ipv4Net;

use super::IdentificationResult;

/*
    Function which test if it's possible to connect to host from a specified port
    Returns Ok(true) if it's possible, Ok(false) if not.
    The fonction returns an Error if the host or the port don't exist  
*/
async fn tcp_ping(host: &str, port: u16) -> Result<IdentificationResult, Error> {

    // Retrieve the first available socket address returned by lookup_host
    // Return an error if the address is not pingable 
    let addr = lookup_host(format!("{host}:{port}")).await?.next().unwrap();

    let stream = TcpStream::connect(&addr);
    // Timeout of 3 seconds to connect to the address
    let time_res = timeout(Duration::from_secs(3), stream).await;

    if time_res.is_err() {
        Err(Error::Timeout)
    }
    else {
        match time_res.unwrap() {
            Ok(a) => Ok(welcome_line(a).await),
            Err(_) => Ok(IdentificationResult::ConnectionRefused),
        }
    }
}

/*
    Function which try to connect to a list of host from a list of port
    Process a parallel call to tcp_ping
*/
async fn tcp_ping_many<'a>(targets: &[(&'a str, u16)]) -> Vec<(&'a str, u16, Result<IdentificationResult, Error>)> {

    let ret: Vec<(&'a str, u16, Result<IdentificationResult, Error>)> = stream::iter(targets)
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
pub async fn tcp_mping<'a>(targets: &[&'a str], ports: &[u16]) -> Vec<(&'a str, u16, Result<IdentificationResult, Error>)> {

    let mut hp_vec: Vec<(&'a str, u16)> = Vec::new();

    for target in targets {
        for port in ports {
            hp_vec.push((*target, *port));
        }
    }

    tcp_ping_many(&hp_vec).await
}

/*
    Fonction which returns a list of host from a specfied string
    Usefull for CIDR notation
*/
pub fn expand_net(host: &str) -> Vec<String> {

    match Ipv4Net::from_str(host) {
        Ok(a) => {
            a.hosts().map(|h| h.to_string()).collect::<Vec<String>>()
        }
        Err(_) => vec![String::from(host)]
    }
}

/*
    Function which returns the welcome line of a connection
    If no welcome line, returns the type IdentificationResult::NoWelcomeLine
*/
async fn welcome_line(server: TcpStream) -> IdentificationResult {
    let first_line = timeout(Duration::from_secs(1), BufReader::new(server).lines().next_line()).await;
    if first_line.is_err() {
        IdentificationResult::NoWelcomeLine 
    }
    else {
        match first_line.unwrap() {
            Ok(opt) => {
                match opt {
                    Some(s) => IdentificationResult::WelcomeLine(s),
                    None =>  IdentificationResult::NoWelcomeLine,
                }
            }
            Err(_) => IdentificationResult::NoWelcomeLine,
        }
    }
}