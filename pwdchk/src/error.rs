#[derive(Debug)]
pub enum Error {
  IoError(std::io::Error),
  NoColon,
  EmptyLogin, 
  EmptyPassword,
  ReqwestError(reqwest::Error),
  ParseIntError(std::num::ParseIntError),
  Timeout,
}
