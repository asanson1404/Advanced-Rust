use crate::account::*;
use sha1::{Sha1, Digest};

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
  