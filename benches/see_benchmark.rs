use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;
use inanis::engine::see::SEEContainer;
use inanis::state::representation::Board;

fn see_benchmark(criterion: &mut Criterion) {
    let fen = "1b2r2k/2qnrn2/5p2/4R3/5P2/3N1N2/1B2Q3/K3R3 w - - 0 1";
    let board = Board::new_from_fen(fen, None, None, None, None, None).unwrap();
    let see_container = SEEContainer::default();

    criterion.bench_function("see_benchmark", |bencher| {
        bencher.iter(|| {
            let attacking_piece = board.get_piece(criterion::black_box(51));
            let captured_piece = board.get_piece(criterion::black_box(35));
            let attackers = board.get_attacking_pieces(criterion::black_box(board.active_color ^ 1), criterion::black_box(35));
            let defenders = board.get_attacking_pieces(criterion::black_box(board.active_color), criterion::black_box(35));

            see_container.get(
                criterion::black_box(attacking_piece),
                criterion::black_box(captured_piece),
                criterion::black_box(attackers),
                criterion::black_box(defenders),
            );
        })
    });
}

criterion_group!(benches, see_benchmark);
criterion_main!(benches);
