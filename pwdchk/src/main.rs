use clap::{Args, Parser, Subcommand, ArgGroup};
mod account;
mod scanner;
use account::*;
use scanner::IdentificationResult;
use std::path::PathBuf;
use macros::french;
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
    /// Check if a host adress and a port number is pingable
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
    #[clap(short = 'o', long = "open-only", required = false)]
    /// Display only the open ports
    o_open: bool,
    #[clap(required = true)]
    /// The host you are trying to open a connection with
    hosts: String,
    #[clap(required = true)]
    /// The port with wich you wants to reach the host 
    ports: String,

}

#[tokio::main]
async fn main() -> Result<(), error::Error> {
    /*let args = AppArgs::parse();
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

            let tmp_hosts: Vec<&str>          = args.hosts.split(',').collect();
            let tmp_ports: Vec<&str>          = args.ports.split(',').collect();
            let mut ports: Vec<u16>           = Vec::new();
            let mut hosts_string: Vec<String> = Vec::new();

            // Convert port from string to u16
            for port in tmp_ports {
                ports.push(port.parse::<u16>()?);
            }

            // Create the hosts (String) vector taking into account the CIDR notation
            for host in tmp_hosts {
                hosts_string.append(&mut scanner::net::expand_net(host));
            }
            
            // Convert String vector into &str vector
            let hosts_str = hosts_string.iter().map(String::as_str).collect::<Vec<&str>>();

            let res = scanner::net::tcp_mping(&hosts_str, &ports).await;

            for conn in res {
                match conn.2 {
                    Ok(a) => {
                        match a {
                            IdentificationResult::WelcomeLine(s) => println!("{}:{} is open: {s}", conn.0, conn.1),
                            IdentificationResult::NoWelcomeLine          => println!("{}:{} is open", conn.0, conn.1),
                            IdentificationResult::ConnectionRefused      => if !args.o_open {println!("{}:{} is closed", conn.0, conn.1);},
                        }
                    }
                    Err(error::Error::IoError(_)) => println!("{}: failed to lookup address information: Name or service not known", conn.0),
                    Err(error::Error::Timeout)    => if !args.o_open {println!("{}:{} timed out", conn.0, conn.1)},
                    Err(_) => println!("Another error : {:?}", conn.2),
                }
            }
            
        }
    }*/
    // Test the error message from the french macro
    //println!("1000 + 230 = {}", french!(11111111111111111111111122222228));
    println!("1000 + 230 = {}", french!(1111111111));
    Ok(())
}
