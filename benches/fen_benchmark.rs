use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;
use inanis::engine::see::SEEContainer;
use inanis::evaluation::EvaluationParameters;
use inanis::state::board::Bitboard;
use inanis::state::movegen::MagicContainer;
use inanis::state::patterns::PatternsContainer;
use inanis::state::zobrist::ZobristContainer;
use std::sync::Arc;

fn fen_benchmark(criterion: &mut Criterion) {
    let evaluation_parameters = Arc::new(EvaluationParameters::default());
    let zobrist_container = Arc::new(ZobristContainer::default());
    let patterns_container = Arc::new(PatternsContainer::default());
    let see_container = Arc::new(SEEContainer::new(Some(evaluation_parameters.clone())));
    let magic_container = Arc::new(MagicContainer::default());

    criterion.bench_function("fen_benchmark", |bencher| {
        bencher.iter(|| {
            Bitboard::new_from_fen(
                criterion::black_box("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1"),
                criterion::black_box(Some(evaluation_parameters.clone())),
                criterion::black_box(Some(zobrist_container.clone())),
                criterion::black_box(Some(patterns_container.clone())),
                criterion::black_box(Some(see_container.clone())),
                criterion::black_box(Some(magic_container.clone())),
            )
            .unwrap()
            .to_fen();
        })
    });
}

criterion_group!(benches, fen_benchmark);
criterion_main!(benches);
