use divan::{black_box, AllocProfiler};
use wildcard_querier::utils::{generate_find_key, generate_key};
use wildcard_querier::Storage;

#[global_allocator]
static GLOBAL: AllocProfiler = AllocProfiler::system();

#[divan::bench(consts = [5, 6, 7, 8, 9, 10])]
fn key_gen<const N: usize>(bencher: divan::Bencher) {
    bencher.bench_local(|| {
        let key = generate_key::<N>();
        black_box(key);
    });
}

#[divan::bench(consts = [5, 6, 7, 8, 9, 10])]
fn findkey_gen<const N: usize>(bencher: divan::Bencher) {
    bencher.bench_local(|| {
        let key = generate_find_key::<N>();
        black_box(key);
    });
}

// --- graph storage ---

#[divan::bench(consts = [5, 6, 7, 8, 9, 10])]
fn graph_increment<const N: usize>(bencher: divan::Bencher) {
    let mut storage = wildcard_querier::graph::KeyStore::<N>::new();

    bencher
        .with_inputs(generate_key::<N>)
        .bench_local_values(|key| {
            storage.increment(key).unwrap();
        });
}

#[divan::bench(consts = [5, 6, 7, 8, 9, 10])]
fn graph_fetch<const N: usize>(bencher: divan::Bencher) {
    let mut storage = wildcard_querier::graph::KeyStore::<N>::new();

    const ITER: usize = 10000;

    for _ in 0..ITER {
        let key = generate_key::<N>();
        storage.increment(key).unwrap();
    }

    bencher
        .with_inputs(generate_find_key::<N>)
        .bench_local_values(|key| {
            let output = storage.get(key);
            let _ = black_box(output);
        });
}

// --- end ---

fn main() {
    // divan::main();
    divan::Divan::from_args().main();
}
