# Stellar Vanity

![](https://img.shields.io/crates/v/stellar_vanity)
![](https://img.shields.io/librariesio/release/cargo/stellar_vanity)

A simple CLI tool to generate Stellar vanity addresses.

**Vanity Address:** similar to a vanity license plate, a vanity cryptocurrency address is an
address where either the beginning (prefix) or end (postfix) is a special or meaningful phrase.
Generating such an address requires work. 

## Benchmarking

Benchmarking is performed by using [criterion.rs](https://github.com/bheisler/criterion.rs) via `cargo bench`, which executes the `benches/benchmark.rs` file.

To see how things actually perform, [not just in theory based on a random chart from the internet](https://github.com/robertDurst/stellar-vanity-address-generator/issues/13), @grempe (thanks a lot!!!) benchmarked on:

```
AWS c5.metal (358 ECUs, 96 vCPUs, 3.6 GHz, 2nd Gen Intel Xeon Platinum 8275CL, 192 GiB memory, EBS only)

25 Gigabit network

Ubuntu Server 18.04 LTS (HVM), SSD Volume Type - ami-085925f297f89fce1
```

using all 96 vCPUs. Below are his results with a `96` thread and `10` sample configuration executed against rust `1.43.0` on May 13, 2020. Using only `10` samples is a weakness in this benchmarking example - will need to increase for a more accurate testing in the future.

| prefix size | measured time |
|-------------|---------------|
| 1           | ~6.7 ms       |
| 2           | ~38.2 ms      |
| 3           | ~889.9 ms     |
| 4           | ~9.7 s        |
| 5           | ~448.2 s      |

## How can I Benchmark?

Ah, thanks so much! I have limited computing power (if you do too... do not attempt, will likely be long and costly)

1. `git clone https://github.com/robertDurst/stellar-vanity-address-generator.git`
2. `cd stellar-vanity-address-generator`
3. `cargo bench`

**Benchmark Configurations:**
* as many threads as possible (see note below)
* 25 samples per method
* 1 - 6 prefixes

**Note:** this uses `num_cpus::get()` from [num_cpus](https://docs.rs/num_cpus/1.13.0/num_cpus/) to determine the maximum number of cores availible. If that is not desired, you'll have to dig in and set this number manually... or open a pr if you know how to pass CLI args to `cargo bench` :) 

## How to use library:
```
use stellar_vanity::vanity_key::AddressGenerator, deserialize_public_key};;

let mut generator: AddressGenerator = Default::default();
let keypair = generator.find(|key| {
    let public = deserialize_public_key(key);
    // any conditions go here
    public.as_str().ends_with("RUST") // e.g. find address with the "RUST" suffix
});
```

This will continuously loop until a key with the desired properties is found. Once the vanity address is found, a keypair will be returned, which may be deserialized with `deserialize_public_key` and `deserialize_private_key` respectively. Note, this is a synchronous function.


## How to use CLI:
```
cargo run -- [--postfix=<POSTFIX>] [--prefix=<PREFIX>] [-c=<NUMBER_OF_THREADS>]

Either `--postfix` or `--prefix` option is required, while thread count is optional.
```

As an example, the following looks for an address ending in pizza with 8 threads:
```
cargo run -- -c=8 --postfix=pizza
```

The `--prefix` and `--postfix` options will search using RegEx expressions. You may need to enclose the expression in quotes when running from the command-line.

The following looks for an address ending in joe with a number before it, using 8 threads:
```
cargo run -- -c=8 --postfix='[0-9]joe'
```

