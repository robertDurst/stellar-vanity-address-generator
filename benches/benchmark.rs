extern crate criterion;
extern crate stellar_vanity;

use std::sync::{mpsc, Arc};
use std::thread;
use criterion::{black_box, criterion_group, criterion_main, Criterion, Benchmark};
use stellar_vanity::vanity_key::{deserialize_public_key, AddressGenerator};

const NUM_THREADS: i64 = 32;
const NUM_SAMPLES: usize = 25;

fn test_generator_postfix_multicore(pattern: &str, threads_count: i64) {
    let (tx, rx) = mpsc::channel();

    for _i in 0..threads_count {
        let thread_tx = tx.clone();
        let postfix = Arc::clone(&Arc::new(pattern.to_uppercase()));

        thread::spawn(move || {
            let mut generator: AddressGenerator = Default::default();

            let keypair = generator
                .find(|key| {
                    let mut found = true;
                    let pk = deserialize_public_key(key);
                    let key_str = pk.as_str();

                    found &= key_str.ends_with(&postfix.to_uppercase());

                    found
                })
                .unwrap();

            // ignore output - will often panic due to send on closed channel
            // race condition
            thread_tx.send(keypair);
        });
    }

    rx.recv();
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench(
        "prefix",
        Benchmark::new("one", |b| b.iter(|| test_generator_postfix_multicore(black_box("a"), black_box(NUM_THREADS))))
        .with_function("two", |b| b.iter(|| test_generator_postfix_multicore(black_box("ab"), black_box(NUM_THREADS))))
        .with_function("three", |b| b.iter(|| test_generator_postfix_multicore(black_box("abc"), black_box(NUM_THREADS))))
        .with_function("four", |b| b.iter(|| test_generator_postfix_multicore(black_box("abcd"), black_box(NUM_THREADS))))
        .sample_size(NUM_SAMPLES)
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);