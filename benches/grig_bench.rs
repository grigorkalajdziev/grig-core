use criterion::{black_box, criterion_group, criterion_main, Criterion};
use grig_core::api::hash::grig_hash;


fn bench_grig_hash(c: &mut Criterion) {
    let password = b"password123";
    let salt = b"salty";
    
    // black_box prevents the compiler from optimizing away our inputs
    c.bench_function("grig_hash (1024 blocks, 3 rounds)", |b| {
        b.iter(|| {
            grig_hash(
                black_box(password),
                black_box(salt)
            )
        })
    });
}

// Register the benchmark group and main function
criterion_group!(benches, bench_grig_hash);
criterion_main!(benches);