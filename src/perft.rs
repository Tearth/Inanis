use crate::{board::*, movescan::*};

pub fn run(depth: u32) {
    let board = Bitboard::new();

    let mut moves = [Move::new(0, 0, MoveFlags::Quiet); 218];

    let mut index = 0;
    index = scan_pawn_moves(&board, Color::Black, &mut moves, index);
    index = scan_knight_moves(&board, Color::Black, &mut moves, index);
    index = scan_bishop_moves(&board, Color::Black, &mut moves, index);
    index = scan_rook_moves(&board, Color::Black, &mut moves, index);
    index = scan_queen_moves(&board, Color::Black, &mut moves, index);
    index = scan_king_moves(&board, Color::Black, &mut moves, index);
    let x = 10;
}
