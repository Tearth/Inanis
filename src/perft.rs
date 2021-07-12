use crate::{board::*, common::*, movescan::*};

pub fn perft(depth: i32) -> u32 {
    let mut board = Bitboard::new();
    let count = perft_internal(depth, &mut board);

    count
}

fn perft_internal(depth: i32, board: &mut Bitboard) -> u32 {
    if depth <= 0 {
        return 1;
    }

    let mut moves = [Move::new(0, 0, MoveFlags::Quiet); 218];
    let moves_count = board.get_moves(&mut moves);

    let mut count = 0;
    for i in 0..moves_count {
        let r#move = moves[i];

        board.make_move(&r#move);
        count += perft_internal(depth - 1, board);
        board.undo_move(&r#move);
    }

    count
}
