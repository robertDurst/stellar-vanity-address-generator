extern crate clap;
extern crate stellar_vanity;
extern crate regex;
#[macro_use]
extern crate fstrings;

use std::sync::{mpsc, Arc};
use std::thread;
use std::time::Instant;

use clap::{App, Arg};
use stellar_vanity::vanity_key::{
    deserialize_private_key, deserialize_public_key, optimized_prefix_deserialize_public_key,
    AddressGenerator,
};

use regex::Regex;

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
        .arg(
            Arg::with_name("THREADS_COUNT")
                .short("c")
                .takes_value(true)
                .default_value("1")
                .help("number of threads to use for searching"),
        )
        .get_matches();

    let threads_count: i64 = matches.value_of("THREADS_COUNT").unwrap().parse().unwrap();
    let postfix_option = Arc::new(matches.value_of("POSTFIX").map(|s| s.to_string()));
    let prefix_option = Arc::new(matches.value_of("PREFIX").map(|s| s.to_string()));

    let (tx, rx) = mpsc::channel();

    if threads_count == 1 {
        println!("\nSEARCHING INITIATED");
    } else {
        println!("\nSEARCHING INITIATED USING {} THREADS", threads_count);
    }

    let start = Instant::now();

    for _i in 0..threads_count {
        let thread_tx = tx.clone();
        let postfix_option = Arc::clone(&postfix_option);
        let prefix_option = Arc::clone(&prefix_option);

        let mut start: std::string::String = "".to_string();
        let mut end: std::string::String = "".to_string();
        let mut startre = Regex::new(r".").unwrap();
        let mut endre = Regex::new(r".").unwrap();

        if let Some(postfix) = &*postfix_option {
            end = postfix.to_uppercase();
            endre = Regex::new(&f!("{end}$")).unwrap();
        }
        if let Some(prefix) = &*prefix_option {
            start = prefix.to_uppercase();
            startre = Regex::new(&f!("^{start}")).unwrap();
        }

        thread::spawn(move || {
            let mut generator: AddressGenerator = Default::default();

            let keypair = generator
                .find(|key| {
                    let mut found = true;

                    if end == "" {
                        let pk = optimized_prefix_deserialize_public_key(key);
                        let key_str = pk.as_str();
                        // found &= key_str[2..].starts_with(&start);
                        found &= &startre.is_match(&key_str[2..]);
                    } else {
                        let pk = deserialize_public_key(key);
                        let key_str = pk.as_str();
                        // found &= key_str[2..].starts_with(&start);
                        // found &= &key_str.ends_with(&end);
                        found &= &startre.is_match(&key_str[2..]);
                        found &= &endre.is_match(&key_str);
                    }

                    found
                })
                .unwrap();

            thread_tx.send(keypair).unwrap();
        });
    }

    let keypair = rx.recv().unwrap();

    let duration = start.elapsed();

    let public_key = deserialize_public_key(&keypair);
    let private_key = deserialize_private_key(&keypair);

    println!(
        "\nSUCCESS!\nPublic Key: {:?}\nSecret Key: {:?}\n\nFound in {:?}",
        public_key, private_key, duration
    );
}
