// split.rs
// Author: Kevin Garner, kevin@kgar.net
//
// Functions used to split file into multiple shares

use sharks::{ Sharks, Share };
use std::str;

pub fn split_shares(file_name: String, min_shares: u8, total_shares: usize) {
    // Set a minimum threshold of shares
    let sharks = Sharks(min_shares as u8);
    // get the secret
    let dealer = sharks.dealer((file_name).as_bytes());
    // Get number of shares
    let shares: Vec<Share> = dealer.take(total_shares as usize).collect();

    println!("Breaking {} into {} shares, requiring {} to recover",
             file_name,
             total_shares,
             min_shares);
    
    // Recover the original secret!
    let secret = sharks.recover(shares.as_slice()).unwrap();
    println!("{:?}", str::from_utf8(&secret).unwrap());
}
