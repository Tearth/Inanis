#[cfg(test)]
mod is_field_attacked_tests {
    use ina::evaluation::pst;
    use ina::state::board::Bitboard;
    use ina::state::movegen;
    use ina::state::patterns;
    use ina::state::zobrist;
    use ina::state::*;
    use std::sync::Once;

    static INIT: Once = Once::new();

    macro_rules! is_field_attacked_tests {
        ($($name:ident: $fen:expr, $white_mask:expr, $black_mask:expr,)*) => {
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

                    for color in 0..2 {
                        let mut result = 0u64;
                        for field_index in 0..64 {
                            if board.is_field_attacked(color, field_index) {
                                result |= 1u64 << field_index;
                            }
                        }

                        if color == WHITE {
                            assert_eq!($white_mask, result);
                        }
                        else if color == BLACK {
                            assert_eq!($black_mask, result);
                        }
                    }
                }
            )*
        }
    }

    is_field_attacked_tests! {
        is_field_attacked_default: "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", 9151313343305220096, 16777086,
        is_field_attacked_mid_game1: "5rk1/2b1qp1p/1r2p1pB/1ppnn3/3pN3/1P1P2P1/2P1QPBP/R4RK1 b - - 7 22", 18410713627276083200, 9548357590732224511,
        is_field_attacked_mid_game2: "2k4r/1p3pp1/p2p2n1/2P1p2q/P1P1P3/3PBPP1/2R3Qr/5RK1 b - - 2 22", 9185565806661272321, 89568307576831,
        is_field_attacked_mid_game3: "r6k/p1B4p/Pp3rp1/3p4/2nP4/2PQ1PPq/7P/1R3RK1 b - - 0 32", 9193953148057572100, 5782712547491610623,
        is_field_attacked_end_game1: "8/8/6Q1/8/6k1/1P2q3/7p/7K b - - 14 75", 614821815842708522, 722824474576036674,
        is_field_attacked_end_game2: "8/8/4nPk1/8/6pK/8/1R3P1P/2B3r1 b - - 1 54", 1452135070281695805, 4632586923901975616,
        is_field_attacked_end_game3: "8/7q/5K2/2q5/6k1/8/8/8 b - - 5 60", 2881868215288276323, 3951704919769088,
    }
}
