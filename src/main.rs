use sharks::{ Sharks, Share };
use std::{ str, env };

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    if args.len() < 4 {
        println!("Too few arguments, exiting.");
        return;
    }
    // Set a minimum threshold of shares
    let sharks = Sharks(3);
    // get the secret
    let dealer = sharks.dealer((&args[1]).as_bytes());
    // Get number of shares
    let shares: Vec<Share> = dealer.take(5).collect();
    
    // Recover the original secret!
    let secret = sharks.recover(shares.as_slice()).unwrap();
    println!("{:?}", str::from_utf8(&secret).unwrap());
}
