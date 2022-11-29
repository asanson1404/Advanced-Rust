mod account;
pub use account::Account;
use std::str::FromStr;

fn main() {

    // PARTIE 2.1
    //let account = Account::new("johndoe", "super:complex:password");
    //println!("{:#?}", account);

    // PARTIE 2.2
    //println!("{:#?}", Account::from_string("johndoe:super:complex:password"));

    // PARTIE 2.3
    match Account::from_str("johndoe") {
        Ok(account) => println!("{account:?}"),
        Err(e) => println!("Erreur {e:?}"),
      }
    
  }
  
