use criterion::{black_box, criterion_group, criterion_main, Criterion};
use grig_core::api::hash::grig_hash;
use std::time::Duration; // <-- Added to configure the time

fn bench_grig_hash(c: &mut Criterion) {
    let password = b"password123";
    let salt = b"salty";    
    
    let mut group = c.benchmark_group("grig_hash_group");    
    
    group.sample_size(50); 
    group.measurement_time(Duration::from_secs(10));
    
    group.bench_function("grig_hash (1024 blocks, 3 rounds)", |b| {
        b.iter(|| {
            grig_hash(
                black_box(password),
                black_box(salt)
            )
        })
    });    
    
    group.finish();
}

criterion_group!(benches, bench_grig_hash);
criterion_main!(benches);