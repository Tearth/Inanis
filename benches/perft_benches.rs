use criterion::*;
use ina::board::Bitboard;
use ina::movegen;
use ina::patterns;
use ina::perft;

fn criterion_benchmark(c: &mut Criterion) {
    patterns::init();
    movegen::init();

    c.bench_function("perft", |b| {
        b.iter(|| perft::run(black_box(4), black_box(&mut Bitboard::new_default())))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
