#[cfg(test)]
mod perft_tests {
    use inanis::perft;
    use inanis::state::representation::Board;

    macro_rules! perft_tests {
        ($($name:ident: $depth:expr, $fen:expr, $expected_leafs_count:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    assert_eq!($expected_leafs_count, perft::normal::run($depth, &mut Board::new_from_fen($fen, None).unwrap(), false).nodes);
                }
            )*
        }
    }

    perft_tests! {
        perft_position_1: 6, "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", 119060324,
        perft_position_2: 5, "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1", 193690690,
        perft_position_3: 7, "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1", 178633661,
        perft_position_4: 6, "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1", 706045033,
        perft_position_5: 5, "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8", 89941194,
        perft_position_6: 5, "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10", 164075551,
    }
}
