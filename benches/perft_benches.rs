use criterion::*;
use ina::board::Bitboard;
use ina::movegen;
use ina::patterns;
use ina::perft;

fn criterion_benchmark(criterion: &mut Criterion) {
    patterns::init();
    movegen::init();

    criterion.bench_function("perft", |bencher| {
        bencher.iter(|| perft::run(black_box(4), black_box(&mut Bitboard::new_default()), black_box(false)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
