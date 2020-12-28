// recover.rs
// Author: Kevin Garner, kevin@kgar.net
//
// Functions used to recover secret file from shares

use sharks::{ Sharks, Share };
use std::{ io::prelude::*,
           fs::{ self, File },
           str,
           convert::TryFrom };

pub fn recover_shares(output_file: String, share_files: Vec<String>) {
    println!("Recovering shares into file {}", output_file);

    // read the min shares required from the header file
    let min_data = fs::read_to_string("header.txt")
        .expect("Unable to read header file.");
    let min_shares: u8 = min_data.parse::<u8>()
        .expect("Error reading header file");
    
    // info from all the share files
    let mut all_shares: Vec<Vec<u8>> = Vec::new();

    // read through share files and get their inputs
    for share_file in share_files {
        let mut inf = File::open(share_file)
            .expect("Error in opening file");
        let mut buffer = Vec::new();
        inf.read_to_end(&mut buffer)
            .expect("Error reading input");
        all_shares.push(buffer);
    }

    let shares: Vec<Share> = all_shares.iter()
        .map(|s| Share::try_from(s.as_slice()).unwrap()).collect();
    // Recover the secret
    let sharks = Sharks(min_shares);
    let secret = sharks.recover(&shares).unwrap();

    fs::write(output_file, str::from_utf8(&secret).unwrap())
        .expect("Error writing output file");
    //println!("{:?}", str::from_utf8(&secret).unwrap());
}
