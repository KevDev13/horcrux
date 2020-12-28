// split.rs
// Author: Kevin Garner, kevin@kgar.net
//
// Functions used to split file into multiple shares

use sharks::{ Sharks, Share };
use std::{ io::prelude::*, fs::File };

pub fn split_shares(file_name: String, min_shares: u8, total_shares: usize) {
    // Set a minimum threshold of shares
    let sharks = Sharks(min_shares);
    // get iterator over the shares for the secret
    let dealer = sharks.dealer((file_name).as_bytes());
    // Get shares
    let shares: Vec<Share> = dealer.take(total_shares).collect();

    println!("Breaking {} into {} shares, requiring {} to recover",
             file_name,
             total_shares,
             min_shares);

    let mut file_number = 1;
    for share in shares {
        let share_file_name: String = String::from("share".to_string()
                                                  + &file_number.to_string()
                                                  + &".txt".to_string());
        let mut file = File::create(share_file_name).expect("Error in creating output file");
        file.write_all(&Vec::from(&share)).expect("Error in writing file");
        file_number += 1;
    }
}
