use criterion::*;
use ina::board::movegen;
use ina::board::patterns;

fn criterion_benchmark(criterion: &mut Criterion) {
    patterns::init();
    movegen::init();

    criterion.bench_function("get_rook_moves", |bencher| {
        let mut bitboard = 0u64;
        let mut field_index = 0;

        bencher.iter(|| {
            bitboard += 1;
            field_index += 1;

            movegen::get_rook_moves(black_box(bitboard), black_box(field_index % 64))
        })
    });

    criterion.bench_function("get_bishop_moves", |bencher| {
        let mut bitboard = 0u64;
        let mut field_index = 0;

        bencher.iter(|| {
            bitboard += 1;
            field_index += 1;

            movegen::get_bishop_moves(black_box(bitboard), black_box(field_index % 64))
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
