use crate::{board::*, common::*, movescan::*};

pub fn perft(depth: i32) -> u32 {
    let mut board = Bitboard::new();
    let count = perft_internal::<WHITE>(depth, &mut board);

    count
}

fn perft_internal<const color: u8>(depth: i32, board: &mut Bitboard) -> u32 {
    if depth <= 0 {
        return 1;
    }

    let mut moves = [Move::new(0, 0, MoveFlags::Quiet); 218];
    let moves_count = board.get_moves::<color>(&mut moves);

    let mut count = 0;
    for i in 0..moves_count {
        let r#move = moves[i];

        board.make_move(&r#move);
        count += match color {
            WHITE => perft_internal::<BLACK>(depth - 1, board),
            BLACK => perft_internal::<WHITE>(depth - 1, board),
            _ => u32::MAX,
        };
        board.undo_move(&r#move);
    }

    count
}
