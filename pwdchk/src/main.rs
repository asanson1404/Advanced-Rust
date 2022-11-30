use clap::{Args, Parser, Subcommand};
mod account;
use account::*;

#[derive(Parser)]
#[clap(version, author, about)]
struct AppArgs {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Check duplicate passwords from command line
    Group(GroupArgs),
}

#[derive(Args)]
struct GroupArgs {
    #[clap(required = true)]
    /// Account to check
    account: Vec<Account>,
}

fn main() {
    let args = AppArgs::parse();
    match args.command {
        Command::Group(args) => {   // args is of type GroupArgs here

           // Grouper les comptes collectés dans args.account, les filtrer, afficher
           // ceux ayant plus d'un login en réutilisant le code écrit précédemment.
           let mut my_hash_map = Account::group(args.account); 
            my_hash_map.retain(|_, v| v.len() > 1);

            for same_pwd_account in my_hash_map {
                println!("Password {} used by {}", same_pwd_account.0, same_pwd_account.1.join(", "));
            } 
        }
    }
}
  
