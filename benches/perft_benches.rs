use criterion::*;
use ina::state::movegen;
use ina::state::patterns;
use ina::state::board::Bitboard;
use ina::perft;

fn criterion_benchmark(criterion: &mut Criterion) {
    patterns::init();
    movegen::init();

    criterion.bench_function("perft", |bencher| {
        bencher.iter(|| perft::normal::run(black_box(4), black_box(&mut Bitboard::new_default()), black_box(false)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
