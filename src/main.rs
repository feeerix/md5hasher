use md5::{Md5, Digest};
use std::env;

fn main() {
    // Collect the command-line arguments
    let args: Vec<String> = env::args().collect();

    // Check if an argument is provided
    if args.len() < 2 {
        eprintln!("Usage: {} <string to hash>", args[0]);
        std::process::exit(1);
    }

    // Get the input string from the first argument
    let input = &args[1];

    // Create a new MD5 hasher
    let mut hasher = Md5::new();

    // Update the hasher with the user input
    hasher.update(input.as_bytes());

    // Finalize the hash computation
    let result = hasher.finalize();

    // Convert the result to a hexadecimal string
    let hex_result = format!("{:x}", result);

    // Print the MD5 hash result
    println!("{}", hex_result);
}
