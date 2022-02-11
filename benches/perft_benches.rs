use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;
use inanis::perft;
use inanis::state::board::Bitboard;

fn criterion_benchmark(criterion: &mut Criterion) {
    inanis::init();

    criterion.bench_function("perft", |bencher| {
        bencher.iter(|| {
            perft::normal::run(
                criterion::black_box(4),
                criterion::black_box(&mut Bitboard::new_initial_position()),
                criterion::black_box(false),
            )
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
