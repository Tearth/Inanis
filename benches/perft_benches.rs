use criterion::*;
use ina::evaluation::pst;
use ina::perft;
use ina::state::board::Bitboard;
use ina::state::movegen;
use ina::state::patterns;
use ina::state::zobrist;

fn criterion_benchmark(criterion: &mut Criterion) {
    pst::init();
    patterns::init();
    movegen::init();
    zobrist::init();

    criterion.bench_function("perft", |bencher| {
        bencher.iter(|| perft::normal::run(black_box(4), black_box(&mut Bitboard::new_default()), black_box(false)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
