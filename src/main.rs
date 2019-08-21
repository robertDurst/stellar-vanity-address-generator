extern crate clap;
extern crate stellar_vanity;

use clap::{App, Arg};
use stellar_vanity::vanity_key::{AddressGenerator, deserialize_public_key, deserialize_private_key};

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

    // since we match on uppercase, convert input values to uppercase
    let target_string = if let Some(postfix) = postfix_option {
        postfix
    } else if let Some(prefix) = prefix_option{
        prefix
    } else {
        // impossible case
        ""
    }.to_uppercase();


    let mut generator: AddressGenerator = Default::default();

    println!("\nSEARCHING INITIATED");

    let keypair = generator
        .find(|key| {
            let mut found = true;
            let pk = deserialize_public_key(key);
            let key_str = pk.as_str();

            if postfix_option.is_some() {
                found &= key_str.ends_with(&target_string);
            }

            if prefix_option.is_some() {
                found &= key_str[2..].starts_with(&target_string);
            }

            found
        })
        .unwrap();
    
    let public_key = deserialize_public_key(&keypair);
    let private_key = deserialize_private_key(&keypair);

    println!(
        "\nSUCCESS!\nPublic Key: {:?}\nSecret Key: {:?}",
        public_key, private_key
    );
}
