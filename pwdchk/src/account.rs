use std::str::FromStr;
use std::collections::HashMap;
use std::vec;
use std::fmt::Display;
use std::error::Error;

#[derive(Debug)]
pub struct Account {
    login:    String, 
    password:  String,
}
#[derive(Debug)]
pub struct NoColon;

impl Account {

  pub fn new(login: &str, password: &str) -> Self {
    Account {login: login.to_string(), password: password.to_string()}
  }

  pub fn group(accounts: Vec<Account>) -> HashMap<String, Vec<String>> {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    for account in accounts {
      let login = account.login.clone();
      map.entry(account.password)
        .and_modify(|v| v.push(account.login))
        .or_insert_with(|| vec![login]);
    }
    map
  }
  

}

impl FromStr for Account {
    type Err = NoColon;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
      if s.contains(':') {
        let v: Vec<&str> = s.splitn(2, ':').collect();
        return Ok(Account {login: v[0].to_string(), password: v[1].to_string()});
      }
      Err(NoColon)
    }
}

impl Display for NoColon {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "Error message")
  }
}

impl Error for NoColon {

}
  
