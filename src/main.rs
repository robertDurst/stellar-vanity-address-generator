extern crate clap;
extern crate stellar_vanity;

use clap::{App, Arg};
use stellar_vanity::vanity_key::AddressGenerator;

fn main() {
    let matches = App::new("Stellar Vanity Address Generator")
        .version("0.4.0")
        .author("Rob Durst")
        .about("A simple CLI for generating Stellar Vanity Addresses.")
        .arg(
            Arg::with_name("POSTFIX")
                .long("postfix")
                .takes_value(true)
                .help("desired address suffix"),
        )
        .arg(
            Arg::with_name("PREFIX")
                .long("prefix")
                .takes_value(true)
                .help("desired address prefix"),
        )
        .get_matches();

    let postfix_option = matches.value_of("POSTFIX");
    let prefix_option = matches.value_of("PREFIX");

    if postfix_option.is_none() && prefix_option.is_none() {
        eprintln!("\n Please, provide prefix or postfix");
        ::std::process::exit(1);
    }

    let mut generator: AddressGenerator = Default::default();

    println!("\nSEARCHING INITIATED");

    let (public_key, private_key) = generator
        .find(|(pk, _)| {
            let mut found = true;
            let key_str = pk.as_str();

            if let Some(postfix) = postfix_option {
                found &= key_str.ends_with(postfix);
            }

            if let Some(prefix) = prefix_option {
                found &= key_str[2..].starts_with(prefix);
            }

            found
        })
        .unwrap();

    println!(
        "\nSUCCESS!\nPublic Key: {:?}\nSecret Key: {:?}",
        public_key, private_key
    );
}
