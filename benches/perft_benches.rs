use criterion::*;
use inanis::perft;
use inanis::state::board::Bitboard;

fn criterion_benchmark(criterion: &mut Criterion) {
    inanis::init();

    criterion.bench_function("perft", |bencher| {
        bencher.iter(|| perft::normal::run(black_box(4), black_box(&mut Bitboard::new_initial_position()), black_box(false)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
