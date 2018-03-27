//! <h1> Stellar Vanity </h1>
//!
//! This is a simple CLI tool for generating Stellar vanity addresses.
//!
//! **Vanity Address:** similar to a vanity license plate, a vanity cryptocurrency address is an
//! address where either the beginning (prefix) or end (postfix) is a special or meaningful phrase.
//! Generating such an address requires work. Below is the expected time and difficulty of finding
//! different length words in a vanity address (based on a more optimized algorthim/codebase).
//!
//! ![https://imgur.com/diotZ02.png](https://imgur.com/diotZ02.png)
//!
//!
//!
//! # Examples
//! ```
//! stellar_vanity AAA // Where AAA is the desired postfix
//! ```

extern crate base32;
extern crate byteorder;
extern crate bytes;
extern crate crc16;
extern crate ed25519_dalek;
extern crate rand;
extern crate sha2;

pub mod vanity_key;
