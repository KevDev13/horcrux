use sharks::{ Sharks, Share };
use std::{ str, env };

mod help;
use help::*;

fn main() {
    // get command line arguments
    let args: Vec<String> = env::args().collect();
    //println!("{:?}", args);

    if args.len() <= 1 {
        println!("Too few arguments, exiting");
        return;
    }

    let first_arg = &args[1];
    
    let help_strings: Vec<String> = vec![String::from("-h"),
                                         String::from("--help"),
                                         String::from("help")];
    if help_strings.contains(first_arg) {
        print_help();
        return;
    }
    
    if args.len() < 5 {
        println!("Too few arguments, exiting.");
        return;
    }
    
    let file_name = &args[2];

    // parse the shares and check to ensure they're good
    let (minimum_shares, num_shares) = match get_shares(&args[3], &args[4]) {
        Some((min, max)) => (min, max),
        None => {
            println!("Exiting...");
            return;
        }
    };
    
    // Set a minimum threshold of shares
    let sharks = Sharks(minimum_shares as u8);
    // get the secret
    let dealer = sharks.dealer((&args[2]).as_bytes());
    // Get number of shares
    let shares: Vec<Share> = dealer.take(num_shares as usize).collect();

    println!("Breaking {} into {} shares, requiring {} to recover",
             file_name,
             num_shares,
             minimum_shares);
    
    // Recover the original secret!
    let secret = sharks.recover(shares.as_slice()).unwrap();
    println!("{:?}", str::from_utf8(&secret).unwrap());
}

fn get_shares(minimum: &String, maximum: &String) -> Option<(u8, usize)> {
    let minimum_as_number = minimum.parse::<i32>().expect("Minimum shares not an integer");
    let maximum_as_number = maximum.parse::<usize>().expect("Maximum shares not an integer");

    if minimum_as_number <= 0 {
        println!("Minimum number of shares must be a positive integer");
        return None;
    }

    if minimum_as_number > 255 {
        println!("Minimum number of shares must be less than 256");
        return None;
    }

    // enforce a maximum
    const MAX_MAX: usize = 2055;
    if maximum_as_number > MAX_MAX {
        println!("Maximum number of shares must be less than {}", MAX_MAX+1);
        return None;
    }
        

    if maximum_as_number <= 0 {
        println!("Maximum number of shares must be a positive number");
        return None;
    }

    // convert minimum to usize. Shouldn't be a problem, because by now
    // we've already verified it's between 0 and 256 exclusive.
    if maximum_as_number <= minimum_as_number as usize {
        println!("Maximum number of shares must be at least 1 greater than minimum number of shares");
        return None;
    }
    
    Some((minimum_as_number as u8, maximum_as_number as usize))
}
