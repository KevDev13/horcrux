// recover.rs
// Author: Kevin Garner, kevin@kgar.net
//
// Functions used to recover secret file from shares

use sharks::{ Sharks, Share };

pub fn recover_shares(file_name: String, shares: Vec<String>) {
    println!("Recovering shares into file {}", file_name);
    // Recover the original secret!
    //let secret = sharks.recover(shares.as_slice()).unwrap();
    //println!("{:?}", str::from_utf8(&secret).unwrap());
}
