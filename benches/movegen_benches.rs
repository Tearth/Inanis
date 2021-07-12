use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ina::{movegen, patterns};

fn criterion_benchmark(c: &mut Criterion) {
    patterns::init();
    movegen::init();

    c.bench_function("get_rook_moves", |b| {
        let mut bitboard = 0u64;
        let mut field_index = 0;

        b.iter(|| {
            bitboard += 1;
            field_index += 1;

            movegen::get_rook_moves(black_box(bitboard), black_box(field_index % 64))
        })
    });

    c.bench_function("get_bishop_moves", |b| {
        let mut bitboard = 0u64;
        let mut field_index = 0;

        b.iter(|| {
            movegen::get_bishop_moves(black_box(bitboard), black_box(field_index % 64));

            bitboard += 1;
            field_index += 1;
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
