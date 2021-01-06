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

use std::env;

// horcrux-specific crates
mod support;
mod split;
mod recover;
use support::*;
use split::*;
use recover::*;

fn main() {
    // get command line arguments
    let args: Vec<String> = env::args().collect();
    //println!("{:?}", args);

    if args.len() <= 1 {
        println!("{}", TOO_FEW_ARGS_STRING);
        return;
    }

    // get the first argument, which should be what the user wants to do
    let first_arg = &args[1];

    // strings to check for what the user wants to do
    let help_strings: Vec<String> = vec![String::from("-h"),
                                         String::from("--help"),
                                         String::from("help")];
    let split_strings: Vec<String> = vec![String::from("-s"),
                                          String::from("--split"),
                                          String::from("split")];
    let recover_strings: Vec<String> = vec![String::from("-r"),
                                            String::from("--recover"),
                                            String::from("recover")];

    // if user wants to list help
    if help_strings.contains(first_arg) {
        print_help();
        return;
    }
    // else if user wants to split strings
    else if split_strings.contains(first_arg) {
        if args.len() < 5 {
            println!("{}", TOO_FEW_ARGS_STRING);
        }

        // input file name
        let file_name = &args[2];
    
        // parse the shares and error check them before proceeding
        let (minimum_shares, num_shares) = match get_shares(&args[3], &args[4]) {
            Some((min, max)) => (min, max),
            None => {
                println!("Exiting..."); // TODO: make this error message better
                return;
            }
        };

        // split the shares into the appropriate files
        split_shares(file_name.to_string(), minimum_shares, num_shares);
    }
    // else if the user wants to recover a secret
    else if recover_strings.contains(first_arg) {
        if args.len() < 4 {
            println!("{}", TOO_FEW_ARGS_STRING);
            return;
        }

        // file name where user wants to output the secret
        let file_name = &args[2];

        // add all recovery shares to a vector to use in a recovery attempt
        let mut r_shares: Vec<String> = Vec::new();
        for share in 3..args.len() {
            r_shares.push(args[share].to_string());
        }

        // attempt to recover the shares
        recover_shares(file_name.to_string(), r_shares);
    }
    // if get here, the user typed in an unknown command and we don't know what to do.
    else {
        println!("Unknown qualifier. Use \"horcrux -h\" or \"horcrux --help \" for help.");
    }
}
