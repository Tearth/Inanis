use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;
use inanis::state::representation::Board;

fn fen_benchmark(criterion: &mut Criterion) {
    criterion.bench_function("fen_benchmark", |bencher| {
        bencher.iter(|| {
            Board::new_from_fen(criterion::black_box("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1")).unwrap().to_fen();
        })
    });
}

criterion_group!(benches, fen_benchmark);
criterion_main!(benches);
