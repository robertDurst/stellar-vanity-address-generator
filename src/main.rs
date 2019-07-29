extern crate clap;
extern crate stellar_vanity;

use clap::{App, Arg};

fn main() {
    let matches = App::new("Stellar Vanity Address Generator")
        .version("0.1.0")
        .author("Rob Durst <rsdurst@colby.edu>")
        .about("A simple CLI for generating Stellar Vanity Addresses.")
        .arg(
            Arg::with_name("POSTFIX")
                .required(true)
                .takes_value(true)
                .index(1)
                .help("desired address postfix"),
        )
        .get_matches();
    let postfix = matches.value_of("POSTFIX").unwrap();

    println!("\nSEARCHING INITIATED");

    let (public_key, private_key) =
        stellar_vanity::vanity_key::generate_vanity_key(&postfix.to_uppercase());

    println!(
        "\nSUCCESS!\nPublic Key: {:?}\nSecret Key: {:?}",
        public_key, private_key
    );
}
