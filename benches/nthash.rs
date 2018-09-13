#[macro_use]
extern crate criterion;
extern crate nthash;
extern crate rand;

use criterion::Criterion;
use nthash::{nthash, NtHashIterator};
use rand::distributions::{Distribution, Uniform};
use rand::thread_rng;

fn nthash_iterator(c: &mut Criterion) {
    c.bench_function("nthash_iterator", |b| {
        let range = Uniform::from(0..4);
        let mut rng = rand::thread_rng();
        let seq = (0..10000)
            .map(|_| match range.sample(&mut rng) {
                0 => 'A',
                1 => 'C',
                2 => 'G',
                3 => 'T',
                _ => 'N',
            })
            .collect::<String>();

        b.iter(|| {
            let iter = NtHashIterator::new(seq.as_bytes(), 5);
            iter.for_each(drop);
        })
    });
}

fn nthash_simple(c: &mut Criterion) {
    c.bench_function("nthash_simple", |b| {
        let range = Uniform::from(0..4);
        let mut rng = rand::thread_rng();
        let seq = (0..10000)
            .map(|_| match range.sample(&mut rng) {
                0 => 'A',
                1 => 'C',
                2 => 'G',
                3 => 'T',
                _ => 'N',
            })
            .collect::<String>();

        b.iter(|| {
            nthash(seq.as_bytes(), 5);
        })
    });
}

criterion_group!(benches, nthash_iterator, nthash_simple);
criterion_main!(benches);
