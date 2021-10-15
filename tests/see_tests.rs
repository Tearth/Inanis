#[cfg(test)]
mod see_tests {
    use ina::engine::see;
    use ina::evaluation::pst;
    use ina::state::board::Bitboard;
    use ina::state::movegen;
    use ina::state::movescan::Move;
    use ina::state::patterns;
    use ina::state::zobrist;
    use ina::state::*;
    use std::mem::MaybeUninit;
    use std::sync::Once;

    static INIT: Once = Once::new();

    macro_rules! see_tests {
        ($($name:ident: $fen:expr, $color:expr, $move:expr, $expected_result:expr, )*) => {
            $(
                #[test]
                fn $name() {
                    INIT.call_once(|| {
                        pst::init();
                        see::init();
                        patterns::init();
                        movegen::init();
                        zobrist::init();
                    });

                    let board = Bitboard::new_from_fen($fen).unwrap();
                    let mut moves: [Move; 218] = unsafe { MaybeUninit::uninit().assume_init() };
                    let moves_count = board.get_moves::<$color>(&mut moves);

                    for move_index in 1..moves_count {
                        let r#move = moves[move_index];
                        if r#move.to_text() == $move {
                            let attacking_piece = board.get_piece(r#move.get_from());
                            let target_piece = board.get_piece(r#move.get_to());
                            let attackers = board.get_attacking_pieces($color ^ 1, r#move.get_to());
                            let defenders = board.get_attacking_pieces($color, r#move.get_to());

                            assert_eq!($expected_result, see::get(attacking_piece, target_piece, attackers, defenders));
                            return;
                        }
                    }

                    assert!(false);
                }
            )*
        }
    }

    see_tests! {
        see_mid_game1: "5rk1/2b1qp1p/1r2p1pB/1ppnn3/3pN3/1P1P2P1/2P1QPBP/R4RK1 b - - 7 22", BLACK, "e5d3", -2,
        see_mid_game2: "2b2rk1/3nqp1p/1r2pnp1/1pp5/3pN2B/1P1P1QP1/2P2PBP/R4RK1 b - - 7 22", WHITE, "e4f6", 3,
        see_mid_game3: "2k4r/1p3pp1/p2p4/2P1p2q/P1P1P2n/3PBPP1/2R3Qr/5RK1 b - - 2 22", BLACK, "h4g2", 6,
        see_mid_game4: "r6k/p1B4p/Pp3rp1/3p4/2nP4/2PQ1PPq/7P/1R3RK1 b - - 0 32", WHITE, "c7b6", -2,
        see_mid_game5: "r1b2rk1/1p2qppp/8/1P1R4/p7/Pn2B1P1/4QPBP/3R2K1 b - - 1 22", BLACK, "e7a3", 1,
        see_mid_game6: "rn1qkb1r/3b1pnp/2p1p1p1/1pN5/p2P1B2/2PB1N1P/PP2QPP1/R4RK1 w kq - 0 18", WHITE, "c5e6", -2,
        see_mid_game7: "r2q3r/2nb1kn1/2p1pp2/6p1/p1BP2Qp/B4N1P/PP2RPP1/3R2K1 b - - 0 33", WHITE, "f3g5", -1,
    }
}
