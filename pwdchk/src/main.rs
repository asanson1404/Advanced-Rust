use clap::{Args, Parser, Subcommand, ArgGroup};
mod account;
mod scanner;
use account::*;
use std::path::PathBuf;
mod error;
mod hibp;


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
    /// Evaluate the performances of par_iter() / iter() functions
    Hipb(HipbArgs),
    /// Check is a host adress and a port number is pingable
    Ping(PingArgs),
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

#[derive(Args)]
struct HipbArgs {
    #[clap(required = true)]
    /// Load accounts and estimate the time to calculate their SHA-1
    file: Option<PathBuf>,
}

#[derive(Args)]
struct PingArgs {
    #[clap(required = true)]
    /// The host you are trying to open a connection with
    host: String,
    #[clap(required = true)]
    /// The port with wich you wants to reach the host 
    port: u16,

}

#[tokio::main]
async fn main() -> Result<(), error::Error> {
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
                    let mut my_hash_map = Account::group(&args.account); 
                    my_hash_map.retain(|_, v| v.len() > 1);

                    for same_pwd_account in my_hash_map {
                        println!("Password {} used by {}", same_pwd_account.0, same_pwd_account.1.join(", "));
                    }
                }
            }
        }

        Command::Hipb(args) => {
            // Should display the time to calculate the the SHA-1
            let accounts = Account::from_file(&args.file.unwrap())?;
            // Check if an account is vulnerable
            let pwd_hack = hibp::check_accounts(&accounts)?;
            println!("{:#?}", pwd_hack);
        }

        Command::Ping(args) => {
            //let t = scanner::net::tcp_ping(args.host.as_str(), args.port).await;
            if scanner::net::tcp_ping(args.host.as_str(), args.port).await {
                println!("{}:{} is open", args.host, args.port);
            }
            else {
                println!("{}:{} is closed", args.host, args.port);
            }
        }
    }
    Ok(())
}
