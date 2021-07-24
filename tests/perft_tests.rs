#[cfg(test)]
mod perft_tests {
    use ina::board::Bitboard;
    use ina::movegen;
    use ina::patterns;
    use ina::perft;
    use ina::zobrist;
    use std::sync::Once;

    static INIT: Once = Once::new();

    macro_rules! perft_tests {
        ($($name:ident: $depth:expr, $expected_leafs_count:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    INIT.call_once(|| {
                        patterns::init();
                        movegen::init();
                        zobrist::init();
                    });

                    assert_eq!($expected_leafs_count, perft::run($depth, &mut Bitboard::new_default(), false).unwrap());
                }
            )*
        }
    }

    perft_tests! {
        perft_depth_1: 1, 20,
        perft_depth_2: 2, 400,
        perft_depth_3: 3, 8902,
        perft_depth_4: 4, 197281,
        perft_depth_5: 5, 4865609,
        perft_depth_6: 6, 119060324,
    }
}
