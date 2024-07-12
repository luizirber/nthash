use codspeed_criterion_compat::{criterion_group, criterion_main, Criterion};
use rand::distributions::{Distribution, Uniform};

use nthash::{nthash, NtHashIterator};

fn nthash_bench(c: &mut Criterion) {
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

    let mut group = c.benchmark_group("nthash");

    group.bench_function("nthash_iterator", |b| {
        b.iter(|| {
            let iter = NtHashIterator::new(seq.as_bytes(), 5).unwrap();
            //  iter.for_each(drop);
            let _res = iter.collect::<Vec<u64>>();
        })
    });

    group.bench_function("nthash_simple", |b| {
        b.iter(|| {
            nthash(seq.as_bytes(), 5);
        })
    });
}

criterion_group!(benches, nthash_bench);
criterion_main!(benches);
