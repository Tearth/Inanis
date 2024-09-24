use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;
use inanis::perft;
use inanis::state::representation::Board;

fn perft_benchmark(criterion: &mut Criterion) {
    let fen = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";
    let mut board = Board::new_from_fen(fen, None, None, None).unwrap();

    criterion.bench_function("perft_benchmark", |bencher| {
        bencher.iter(|| perft::normal::run(criterion::black_box(2), criterion::black_box(&mut board), criterion::black_box(false)))
    });
}

criterion_group!(benches, perft_benchmark);
criterion_main!(benches);
