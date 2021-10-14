#[cfg(test)]
mod get_attacking_pieces_tests {
    use ina::evaluation::pst;
    use ina::state::board::Bitboard;
    use ina::state::movegen;
    use ina::state::patterns;
    use ina::state::zobrist;
    use ina::state::*;
    use std::sync::Once;

    static INIT: Once = Once::new();

    macro_rules! get_attacking_pieces_tests {
        ($($name:ident: $fen:expr, $color:expr, $field_index:expr, $expected_result:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    INIT.call_once(|| {
                        pst::init();
                        patterns::init();
                        movegen::init();
                        zobrist::init();
                    });

                    let board = Bitboard::new_from_fen($fen).unwrap();
                    assert_eq!($expected_result, board.get_attacking_pieces($color, $field_index));
                }
            )*
        }
    }

    get_attacking_pieces_tests! {
        get_attacking_pieces_default: "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", BLACK, 18, 3,
        get_attacking_pieces_mid_game1: "5rk1/2b1qp1p/1r2p1pB/1ppnn3/3pN3/1P1P2P1/2P1QPBP/R4RK1 b - - 7 22", WHITE, 44, 82,
        get_attacking_pieces_mid_game2: "2b2rk1/4qp1p/1r2pnpB/1pp1n3/3pN3/1P1P2P1/2P1QPBP/R4RK1 b - - 7 22", WHITE, 52, 78,
        get_attacking_pieces_mid_game3: "2k4r/1p3pp1/p2p2n1/2P1p2q/P1P1P3/3PBPP1/2R3Qr/5RK1 b - - 2 22", BLACK, 5, 50,
        get_attacking_pieces_mid_game4: "r6k/p1B4p/Pp3rp1/3p4/2nP4/2PQ1PPq/7P/1R3RK1 b - - 0 32", BLACK, 26, 3,
        get_attacking_pieces_mid_game5: "r1b2rk1/1p2qppp/8/1P1R4/p7/Pn2B1P1/4QPBP/3R2K1 b - - 1 22", BLACK, 12, 114,
        get_attacking_pieces_end_game1: "8/8/6Q1/8/6k1/1P2q3/7p/7K b - - 14 75", WHITE, 17, 192,
        get_attacking_pieces_end_game2: "8/8/4nPk1/8/6pK/8/1R3P1P/2B3r1 b - - 1 54", BLACK, 17, 129,
        get_attacking_pieces_end_game3: "8/7q/5K2/2q5/6k1/8/8/8 b - - 5 60", BLACK, 34, 128,
    }
}
