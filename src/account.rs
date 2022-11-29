use std::str::FromStr;

#[derive(Debug)]
pub struct Account {
    pub login:    String, 
    pub password:  String,
}
#[derive(Debug)]
pub struct NoColon;

impl Account {

  pub fn new(login: &str, password: &str) -> Self {
    Account {login: login.to_string(), password: password.to_string()}
  }

  pub fn from_string(s: &str) -> Self {
    let v: Vec<&str> = s.splitn(2, ':').collect();
    Account {login: v[0].to_string(), password: v[1].to_string()}
  }

}

impl FromStr for Account {
    type Err = NoColon;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v: Vec<&str> = s.splitn(2, ':').collect();
        Ok(Account {login: v[0].to_string(), password: v[1].to_string()})
      }
}
  
