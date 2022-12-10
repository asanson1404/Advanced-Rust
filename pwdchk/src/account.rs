use std::str::FromStr;
use std::collections::HashMap;
use std::vec;
use std::fmt::Display;
use crate::error;
use std::path::Path;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::BufRead;
use std::error::Error;

#[derive(Debug)]
pub struct Account {
    pub login:    String, 
    pub password:  String,
}

impl Account {

  pub fn new(login: &str, password: &str) -> Self {
    Account {login: login.to_string(), password: password.to_string()}
  }

  pub fn group<'a>(accounts: &'a [Account]) -> HashMap<&'a str, Vec<&'a str>> {
    let mut map: HashMap<&'a str, Vec<&'a str>> = HashMap::new();
    for account in accounts {
      map.entry(account.password.as_str())
        .and_modify(|v| v.push(account.login.as_str()))
        .or_insert_with(|| vec![account.login.as_str()]);
    }
    map
  }

  pub fn from_file(filename: &Path) -> Result<Vec<Account>, error::Error> {

    let f = File::open(filename).map_err(error::Error::from);
    let reader = BufReader::new(f?);

    let mut accounts = Vec::<Account>::new();
  

    for line in reader.lines() {
      accounts.push(Account::from_str(&line.unwrap())?);
    }

    Ok(accounts)
  }

}

impl FromStr for Account {
    type Err = error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
      if s.contains(':') {
        let v: Vec<&str> = s.splitn(2, ':').collect();
        return Ok(Account {login: v[0].to_string(), password: v[1].to_string()});
      }
      Err(error::Error::NoColon)
    }
}

impl Display for error::Error {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "Error message")
  }
}

impl Error for error::Error {}
  
impl From<std::io::Error> for error::Error {
  fn from(item: std::io::Error) -> Self {
    error::Error::IoError(item)
  }
}