#![allow(clippy::eq_op)]

#[cfg(test)]
mod see_tests {
    use crate::see_tests::see::SEEContainer;
    use inanis::engine::see;
    use inanis::evaluation::EvaluationParameters;
    use inanis::state::board::Bitboard;
    use inanis::state::movescan::Move;
    use std::mem::MaybeUninit;
    use std::sync::Arc;
    use std::sync::Once;

    static INIT: Once = Once::new();
    static P: i16 = 100;
    static N: i16 = 442;
    static B: i16 = 442;
    static R: i16 = 648;
    static Q: i16 = 1325;
    static K: i16 = 10000;

    macro_rules! see_tests {
        ($($name:ident: $fen:expr, $move:expr, $expected_result:expr, )*) => {
            $(
                #[test]
                fn $name() {
                    INIT.call_once(|| {
                        inanis::init();
                    });

                    let board = Bitboard::new_from_fen($fen).unwrap();
                    let mut moves: [MaybeUninit<Move>; 218] = [MaybeUninit::uninit(); 218];
                    let moves_count = board.get_all_moves(&mut moves, u64::MAX);

                    let see = SEEContainer::default();
                    let mut evaluation_parameters = EvaluationParameters::default();
                    evaluation_parameters.piece_value = [P, N, B, R, Q, K];

                    let evaluation_parameters_arc = Arc::new(evaluation_parameters);
                    for move_index in 0..moves_count {
                        let r#move = unsafe { moves[move_index].assume_init() };
                        if r#move.to_long_notation() == $move {
                            let attacking_piece = board.get_piece(r#move.get_from());
                            let target_piece = board.get_piece(r#move.get_to());
                            let attackers = board.get_attacking_pieces(board.active_color ^ 1, r#move.get_to());
                            let defenders = board.get_attacking_pieces(board.active_color, r#move.get_to());

                            assert_eq!($expected_result, see.get(attacking_piece, target_piece, attackers, defenders, &evaluation_parameters_arc));
                            return;
                        }
                    }

                    assert!(false);
                }
            )*
        }
    }

    see_tests! {
        see_simple_01: "8/8/8/4p3/3P4/8/8/8 w - - 0 1", "d4e5", P,
        see_simple_02: "8/8/5p2/4p3/3P4/8/8/8 w - - 0 1", "d4e5", P - P,
        see_simple_03: "8/8/5p2/4p3/3P4/8/7B/8 w - - 0 1", "d4e5", P - P + P,
        see_simple_04: "8/8/5p2/4p3/3P4/8/7B/8 w - - 0 1", "h2e5", P - B + P,
        see_simple_05: "8/8/8/3k4/3P4/8/8/8 b - - 0 1", "d5d4", P,
        see_simple_06: "8/8/2n2b2/8/3P4/8/4N3/8 b - - 0 1", "c6d4", P - N + N,
        see_complex_01: "8/2bn1n2/8/4p3/6N1/2B2N2/8/8 w - - 0 1", "f3e5", P - N + N - N,
        see_complex_02: "8/2bn1n2/8/4p3/6N1/2B2N2/8/4Q3 w - - 0 1", "f3e5", P - N + N - N + N - B + B,
        see_complex_03: "8/3n2b1/2n5/4R3/5P2/3N1N2/8/8 b - - 0 1", "d7e5", R - N,
        see_complex_04: "8/3n2b1/2nq4/4R3/5P2/3N1N2/8/8 b - - 0 1", "d6e5", R - Q + P - N + N - N + N,
        see_xray_01: "4r3/8/4p3/8/8/8/4R3/4R3 w - - 0 1", "e2e6", P - R + R,
        see_xray_02: "4n3/8/5p2/8/8/2B5/1Q6/8 w - - 0 1", "c3f6", P - B + N,
        see_xray_03: "8/8/5p1q/8/8/5Q2/8/5R2 w - - 0 1", "f3f6", P - Q + Q,
        see_xray_04: "4q3/4r3/4r3/8/8/RQR1P3/8/8 b - - 0 1", "e6e3", P - R + R - R + Q - Q,
        see_xray_05: "7q/8/5b2/8/8/2B5/3P4/8 b - - 0 1", "f6c3", B - B + P,
        see_xray_06: "4r3/8/4q3/8/4P3/5P2/8/8 b - - 0 1", "e6e4", P - Q + P,
    }
}
