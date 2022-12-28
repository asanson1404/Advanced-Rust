use futures::Future;

#[derive(Debug)]
pub enum Error {
  IoError(std::io::Error),
  NoColon,
  ReqwestError(reqwest::Error),
  ParseIntError(std::num::ParseIntError),
  Timeout,
}
