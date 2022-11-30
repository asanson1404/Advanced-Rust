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

    //let v: Result<Vec<_>, _> = env::args().skip(1).map(|c| Account::from_str(c.as_str())).collect();
    //let accounts = v?;

    //println!("{accounts:#?}");
    //Ok(())

    // PARTIE 2.4
    let v: Result<Vec<_>, _> = env::args().skip(1).map(|c| Account::from_str(c.as_str())).collect();
    let accounts = v?;

    let mut my_hash_map = Account::group(accounts); 
    my_hash_map.retain(|_, v| v.len() > 1);

    //println!("{my_hash_map:#?}");

    for same_pwd_account in my_hash_map {
        println!("Password {} used by {}", same_pwd_account.0, same_pwd_account.1.join(", "));
    } 

    Ok(())
    
}
  
