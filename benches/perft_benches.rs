use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ina::{movegen, patterns, perft::*};

fn criterion_benchmark(c: &mut Criterion) {
    patterns::init();
    movegen::init();

    c.bench_function("perft", |b| b.iter(|| perft(black_box(4))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
