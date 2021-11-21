extern crate criterion;
extern crate num_cpus;
extern crate stellar_vanity;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::sync::{mpsc, Arc};
use std::thread;
use stellar_vanity::vanity_key::{deserialize_public_key, AddressGenerator};

// const NUM_SAMPLES: usize = 10;

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
            let _ = thread_tx.send(keypair);
        });
    }

    let _ = rx.recv();
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("postfix");
    // use as many threads as possible
    let num_threads: i64 = (num_cpus::get() as i64) - 1;
    // hard stop in case fewer than a realistic number of threads availible
    if num_threads < 32 {
        print!("Sorry, you unfortunately do not have enough threads to realistically benchmark. 32 recommended.")
    }

    group
        .bench_function("one", |b| b.iter(|| test_generator_postfix_multicore(black_box("a"), black_box(num_threads))))
        .sample_size(100);

    group
        .bench_function("two", |b| b.iter(|| test_generator_postfix_multicore(black_box("ab"), black_box(num_threads))))
        .sample_size(25);
    
    group
        .bench_function("three", |b| b.iter(|| test_generator_postfix_multicore(black_box("abc"), black_box(num_threads))))
        .sample_size(10);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
