use md5::{Md5, Digest};
use std::io::{self, Write};

fn main() {
    // Prompt the user for input
    print!("Enter a string to hash: ");
    io::stdout().flush().unwrap(); // Ensure the prompt is printed before reading input

    // Read input from the user
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Remove the newline character from the input
    let input = input.trim();

    // Create a new MD5 hasher
    let mut hasher = Md5::new();

    // Update the hasher with the user input
    hasher.update(input.as_bytes());

    // Finalize the hash computation
    let result = hasher.finalize();

    // Convert the result to a hexadecimal string
    let hex_result = format!("{:x}", result);

    // Print the MD5 hash result
    println!("MD5 hash: {}", hex_result);
}
