// support.rs
// Author: Kevin Garner, kevin@kgar.net
//
// Support functions/info for Horcrux
//
// License information can be found at the repo:
// https://github.com/KevDev13/horcrux

pub const TOO_FEW_ARGS_STRING: &str = "Too few arguments, exiting...";

const HORCRUX_VERSION: &str = "0.2.0";
const REPO: &str = "https://github.com/KevDev13/horcrux";

pub fn print_help() {
    println!("Horcrux v{}", HORCRUX_VERSION);
    println!("License information at: {}/blob/main/LICENSE", REPO);
    println!();
    println!("Warning: Horcrux is still a work in progress. Until Horcrux v1.0 is");
    println!("released, use at your own risk.");
    println!();
    println!("Commands:");
    println!();
    println!(" -h, --help, help             show this menu");
    println!(" -s, --split, split           split file");
    println!(" -r, --recover, recover       recover file");
    println!();
    println!("Split example:");
    println!(" ./horcrux -s test.txt 3 5");
    println!(" This will split test.txt into 5 separate shares, requiring 3 to recover");
    println!();
    println!("Recover example:");
    println!(" ./horcrux --recover out.txt share1.txt share3.txt share4.txt");
    println!(" This will recover the secret file into out.txt using the share files");
    println!();
    println!("Please report bugs here: {}/issues", REPO);
}

pub fn get_shares(minimum: &String, total: &String) -> Option<(u8, usize)> {
    // parse the share numbers from strings to numbers.
    let minimum_as_number = minimum.parse::<i32>().expect("Minimum shares not an integer");
    let total_as_number = total.parse::<usize>().expect("Total shares not an integer");

    // need at least 1 minimum share
    // TODO: might want to make this 2, since only having one share as minimum
    // seems counter-productive
    if minimum_as_number <= 0 {
        println!("Minimum number of shares must be a positive integer");
        return None;
    }

    // GF256 can't handle more than 256 fields
    if minimum_as_number > 255 {
        println!("Minimum number of shares must be less than 256");
        return None;
    }

    // enforce a maximum
    const MAX_MAX: usize = 256;
    if total_as_number > MAX_MAX {
        println!("Total number of shares must be less than {}", MAX_MAX+1);
        return None;
    }
        
    // need at least one share
    // TODO: make this 2?
    if total_as_number <= 0 {
        println!("Total number of shares must be a positive number");
        return None;
    }

    // convert minimum to usize. Shouldn't be a problem, because by now
    // we've already verified it's between 0 and 256 exclusive.
    if total_as_number <= minimum_as_number as usize {
        println!("Total number of shares must be at least 1 greater than minimum number of shares");
        return None;
    }

    // if we get here, then all error checks passed, so return the values
    Some((minimum_as_number as u8, total_as_number as usize))
}
