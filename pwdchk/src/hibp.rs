use crate::account::*;
use sha1::{Sha1, Digest};
use rayon::prelude::*;
use std::time::Instant;

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
  
  