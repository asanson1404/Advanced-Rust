use crate::account::*;
use sha1::{Sha1, Digest};
use rayon::prelude::*;
use std::{time::Instant, collections::HashMap};
use crate::error::Error;

/*
    Function which calculate the sha1 of a password in a sequential way 
 */
fn sha1(account: &Account) -> (String, String) {

    // create a Sha1 object
    let mut hasher = Sha1::new();
    // process input message
    hasher.update(&account.password);
    // acquire hash digest in the form of GenericArray
    let hash_result = hasher.finalize();

    // Create a variable which contains the capital hexadecimal of hash_result
    let hex_hash = format!("{hash_result:X}");//.split_at(5);
    let pref_suf = hex_hash.split_at(5);

    let prefix = String::from(pref_suf.0);
    let suffix = String::from(pref_suf.1);

    (prefix, suffix)
}

/*
    Function which calculate the sha1 of a password using all the core of the machine
 */
fn all_sha1(accounts: &[Account]) -> Vec<(String, String, &Account)> {

    // parallele iterator : add prefexix, suffix, and ref_account of each accounts to the vector ret
    let ret: Vec::<(String, String, &Account)> = 
        accounts.par_iter()
        .map(|i| {
            let pref_suf = sha1(i);
            (pref_suf.0, pref_suf.1, i)
        }).collect();

    ret
}

/*
    Function to compare the performances of par_iter and iter functions
    Display the time of sha1 calcuation :
        - with iter        ---> 235374 µs to compute all
        - with par_iter    ---> 58791  µs to compute all
    Using mutli-core calculation is 4 times faster !!!
 */
pub fn all_sha1_timed(accounts: &[Account]) -> Vec<(String, String, &Account)> {
    let t0 = Instant::now();
    let ret = all_sha1(accounts);
    let t1 = Instant::now();
    println!("{} µs to calculate all sha1", (t1 - t0).as_micros());
    ret
}

/*
    Function which returns a HashMap which group the accounts according to their prefix 
 */
pub fn sha1_by_prefix(accounts: &[Account]) -> HashMap<String, Vec<(String, &Account)>> {
    
    let hash_vals = all_sha1(accounts);
    let mut map = HashMap::<String, Vec<(String, &Account)>>::new();

    for hash_val in hash_vals {
        let suf = hash_val.1.clone();
        map.entry(hash_val.0)
        .and_modify(|v| v.push((hash_val.1, hash_val.2)))
        .or_insert_with(|| vec![(suf, hash_val.2)]);
    } 
    map
}

/*
    Function wich retrieve all the suffixes of password with the site Have I been pwned? 
    according to a defined prefix.  
 */
fn get_page(prefix: &str) -> Result<Vec<String>, Error> {

    let url = String::from("https://api.pwnedpasswords.com/range/") + prefix;
    let body = reqwest::blocking::get(url)?.text()?;
    let lines_vec = body.lines().map(String::from).collect();

    Ok(lines_vec)
}

/*
    Function which returns, from a prefix, a Hash table with the number of occurences
    the the password has been hacked
 */
fn get_suffixes(prefix: &str) -> Result<HashMap<String, u64>, Error> {

    let mut hash_map = HashMap::<String, u64>::new();

    let lines = get_page(prefix)?;
    for line in lines {
        let val: Vec<&str> = line.split(':').collect();
        // we are sure all suffixes are unique
        hash_map.insert(String::from(val[0]), val[1].parse::<u64>()?);
    }
    Ok(hash_map)
}
  

impl From<reqwest::Error> for Error {
    fn from(item: reqwest::Error) -> Self {
      Error::ReqwestError(item)
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(item: std::num::ParseIntError) -> Self {
      Error::ParseIntError(item)
    }
}
  