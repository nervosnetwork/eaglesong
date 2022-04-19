use blake2b_rs::blake2b;
use criterion::{criterion_group, criterion_main, Criterion};

pub const S: [u8; 4096] = [1u8; 4096];

fn bench(c: &mut Criterion) {
    c.bench_function("bench_eaglesong", |b| {
        b.iter(|| {
            let mut hash = [0u8; 32];
            eaglesong::eaglesong(&S, &mut hash);
        })
    });

    c.bench_function("bench_eaglesong_builder", |b| {
        b.iter(|| {
            let mut builder = eaglesong::EagleSongBuilder::new();
            builder.update(&S);
            builder.finalize();
        })
    });

    c.bench_function("bench_blake2b", |b| {
        b.iter(|| {
            let mut hash = [0u8; 32];
            blake2b(&[], &S, &mut hash);
        })
    });
}

criterion_group!(benches, bench);
criterion_main!(benches);
