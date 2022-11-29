mod account;
pub use account::Account;

fn main() {

    // PARTIE 2.1
    //let account = Account::new("johndoe", "super:complex:password");
    //println!("{:#?}", account);

    // PARTIE 2.2
    println!("{:#?}", Account::from_string("johndoe:super:complex:password"));
    
  }
  
