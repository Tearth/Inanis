#[cfg(test)]
mod board_tests {
    use inanis::state::representation::Board;
    use inanis::state::*;

    macro_rules! is_square_attacked_tests {
        ($($name:ident: $fen:expr, $white_mask:expr, $black_mask:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let board = Board::new_from_fen($fen, None, None, None, None, None).unwrap();

                    for color in WHITE..=BLACK {
                        let mut result = 0u64;
                        for square_index in A1..=H8 {
                            if board.is_square_attacked(color, square_index) {
                                result |= 1u64 << square_index;
                            }
                        }

                        match color {
                            WHITE => assert_eq!($white_mask, result),
                            BLACK => assert_eq!($black_mask, result),
                            _ => panic!("Invalid value: color={}", color)
                        };
                    }
                }
            )*
        }
    }

    is_square_attacked_tests! {
        is_square_attacked_default: "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", 9151313343305220096, 16777086,
        is_square_attacked_mid_game1: "5rk1/2b1qp1p/1r2p1pB/1ppnn3/3pN3/1P1P2P1/2P1QPBP/R4RK1 b - - 7 22", 18410713627276083200, 9548357590732224511,
        is_square_attacked_mid_game2: "2k4r/1p3pp1/p2p2n1/2P1p2q/P1P1P3/3PBPP1/2R3Qr/5RK1 b - - 2 22", 9185565806661272321, 89568307576831,
        is_square_attacked_mid_game3: "r6k/p1B4p/Pp3rp1/3p4/2nP4/2PQ1PPq/7P/1R3RK1 b - - 0 32", 9193953148057572100, 5782712547491610623,
        is_square_attacked_end_game1: "8/8/6Q1/8/6k1/1P2q3/7p/7K b - - 14 75", 614821815842708522, 722824474576036674,
        is_square_attacked_end_game2: "8/8/4nPk1/8/6pK/8/1R3P1P/2B3r1 b - - 1 54", 1452135070281695805, 4632586923901975616,
        is_square_attacked_end_game3: "8/7q/5K2/2q5/6k1/8/8/8 b - - 5 60", 2881868215288276323, 3951704919769088,
    }

    macro_rules! get_attacking_pieces_tests {
        ($($name:ident: $fen:expr, $color:expr, $square_index:expr, $expected_result:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let board = Board::new_from_fen($fen, None, None, None, None, None).unwrap();
                    assert_eq!($expected_result, board.get_attacking_pieces($color, $square_index));
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
