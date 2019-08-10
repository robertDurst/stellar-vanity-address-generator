# Stellar Vanity
A simple CLI tool to generate Stellar vanity addresses.

**Vanity Address:** similar to a vanity license plate, a vanity cryptocurrency address is an
address where either the beginning (prefix) or end (postfix) is a special or meaningful phrase.
Generating such an address requires work. Below is the expected time and difficulty of finding
different length words in a vanity address (based on a more optimized algorthim/codebase).

![Vanity Key Chrt](https://imgur.com/diotZ02.png)

## How to use library:
```
use stellar_vanity::vanity_key::AddressGenerator;

let mut generator: AddressGenerator = Default::default();

let (public_key, private_key) = generator.find(|public, private| {
  // any conditions go here
  public.as_str().ends_with("RUST") // e.g. find address with the "RUST" suffix
});
```

This will continuously loop until a key with the desired properties is found. Once the vanity address is found,
a tuple (public_key, private_key) will be returned. Note, this is a synchronous function.


## How to use CLI:
```
cargo run [--postfix=<POSTFIX>] [--prefix=<PREFIX>]

Either `--postfix` or `--prefix` option is required.
```
