use clap::{Args, Parser, Subcommand, ArgGroup};
mod account;
use account::*;
use std::path::PathBuf;
mod error;

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
#[clap(group(
    ArgGroup::new("input")
        .required(true)
        .args(&["account", "file"]),
))]
struct GroupArgs {
    #[clap(required = false)]
    /// Account to check
    account: Vec<Account>,
    #[clap(short, long)]
    /// Load passwords from a file
    file: Option<PathBuf>,
}

fn main() -> Result<(), error::Error> {
    let args = AppArgs::parse();
    match args.command {
        Command::Group(args) => {   // args is of type GroupArgs here

            match args.file {

                Some(f) => {
                    let accounts_ff = Account::from_file(&f)?;
                    for account in accounts_ff {
                        println!("{:#?}", account);
                    }
                },

                None             => {
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
    }
    Ok(())
}
  
