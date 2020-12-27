// main.rs
// Author: Kevin Garner, kevin@kgar.net
//
// Horcrux is an application that will split a file into
// multiple shares using Shamir's Secret Sharing. This will
// allow the separating of files across different locations
// (i.e. cloud services, USB drives, etc) while still allowing
// the loss of 1 or more shares with the ability to recover
// the primary file at the end.
//
// License information can be found at the repo:
// https://github.com/KevDev13/horcrux

//use sharks::{ Sharks, Share };
use std::{ str, env };

mod help;
mod split;
mod recover;
use help::*;
use split::*;
use recover::*;

const TOO_FEW_ARGS_STRING: &str = "Too few arguments, exiting...";

fn main() {
    // get command line arguments
    let args: Vec<String> = env::args().collect();
    //println!("{:?}", args);

    if args.len() <= 1 {
        println!("{}", TOO_FEW_ARGS_STRING);
        return;
    }

    let first_arg = &args[1];
    
    let help_strings: Vec<String> = vec![String::from("-h"),
                                         String::from("--help"),
                                         String::from("help")];
    let split_strings: Vec<String> = vec![String::from("-s"),
                                          String::from("--split"),
                                          String::from("split")];
    let recover_strings: Vec<String> = vec![String::from("-r"),
                                            String::from("--recover"),
                                            String::from("recover")];
    
    if help_strings.contains(first_arg) {
        print_help();
        return;
    }
    else if split_strings.contains(first_arg) {
        if args.len() < 5 {
            println!("{}", TOO_FEW_ARGS_STRING);
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

        split_shares(file_name.to_string(), minimum_shares, num_shares);
    }
    else if recover_strings.contains(first_arg) {
        recover_shares();
    }
    else {
        println!("Unknown qualifier. Use \"horcrux -h\" or \"horcrux --help \" for help.");
    }
}

fn get_shares(minimum: &String, total: &String) -> Option<(u8, usize)> {
    let minimum_as_number = minimum.parse::<i32>().expect("Minimum shares not an integer");
    let total_as_number = total.parse::<usize>().expect("Total shares not an integer");

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
    if total_as_number > MAX_MAX {
        println!("Total number of shares must be less than {}", MAX_MAX+1);
        return None;
    }
        

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
    
    Some((minimum_as_number as u8, total_as_number as usize))
}
