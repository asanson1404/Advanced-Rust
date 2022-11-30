mod account;
pub use account::*;
use std::str::FromStr;
use std::env;

fn main() -> Result<(), NoColon> {

    // PARTIE 2.1
    //let account = Account::new("johndoe", "super:complex:password");
    //println!("{:#?}", account);

    // PARTIE 2.2
    //println!("{:#?}", Account::from_string("johndoe:super:complex:password"));

    // PARTIE 2.3
    //match Account::from_str("johndoe:azerty") {
    //    Ok(account) => println!("{account:?}"),
    //    Err(e) => println!("Erreur {e:?}"),
    //  }
    //println!("{:#?}", Account::from_str("johndoe")?);
    //Ok(())

    let v: Vec<Result<_, _>> = env::args().map(|c| Account::from_str(c.as_str())).collect();
    let accounts = &v[1..];
    println!("{accounts:#?}");
    Ok(())
    
}
  
