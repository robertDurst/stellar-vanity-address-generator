extern crate stellar_vanity;

use std::io;

// Get CLI input from the user
fn get_user_input() -> String {
    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    let trimmed = input_text.trim();
    match trimmed.parse::<String>() {
        Ok(i) => {
            return i;
        },
        Err(..) => {
            println!("Incorrectly formatted guess, try again.");
            return get_user_input();
        },
    };
}

fn main() {
    println!("\nEnter desired account id ending!");
    let user_input = get_user_input();
    println!("\nSEARCHING INITIATED");
    let (public_key, private_key) = stellar_vanity::key::generate_vanity_key(&user_input.to_uppercase());
    println!("\nSUCCESS!\nPublic Key: {:?}\nSecret Key: {:?}", public_key, private_key);
}
