use super::bindings::*;
use crate::state::board::Bitboard;
use crate::state::movescan::{Move, MoveFlags};
use crate::tablebases::WdlResult;
use crate::{engine, state::*};
use std::ffi::CString;
use std::mem::MaybeUninit;
use std::ptr;

pub fn init(syzygy_path: &str) {
    unsafe {
        tb_init(CString::new(syzygy_path).unwrap().as_ptr());
    }
}

pub fn get_max_pieces_count() -> u8 {
    unsafe { TB_LARGEST as u8 }
}

pub fn get_wdl(board: &Bitboard) -> Option<WdlResult> {
    let wdl = unsafe {
        tb_probe_wdl(
            board.occupancy[WHITE as usize],
            board.occupancy[BLACK as usize],
            board.pieces[WHITE as usize][KING as usize] | board.pieces[BLACK as usize][KING as usize],
            board.pieces[WHITE as usize][QUEEN as usize] | board.pieces[BLACK as usize][QUEEN as usize],
            board.pieces[WHITE as usize][ROOK as usize] | board.pieces[BLACK as usize][ROOK as usize],
            board.pieces[WHITE as usize][BISHOP as usize] | board.pieces[BLACK as usize][BISHOP as usize],
            board.pieces[WHITE as usize][KNIGHT as usize] | board.pieces[BLACK as usize][KNIGHT as usize],
            board.pieces[WHITE as usize][PAWN as usize] | board.pieces[BLACK as usize][PAWN as usize],
            0,
            0,
            0,
            board.active_color == WHITE,
        )
    };

    match wdl {
        TB_WIN => Some(WdlResult::Win),
        TB_LOSS => Some(WdlResult::Loss),
        TB_DRAW | TB_CURSED_WIN | TB_BLESSED_LOSS => Some(WdlResult::Draw),
        _ => None,
    }
}

pub fn get_root_wdl_dtz(board: &Bitboard) -> (bool, WdlResult, u32, Move) {
    let result = unsafe {
        tb_probe_root(
            board.occupancy[WHITE as usize],
            board.occupancy[BLACK as usize],
            board.pieces[WHITE as usize][KING as usize] | board.pieces[BLACK as usize][KING as usize],
            board.pieces[WHITE as usize][QUEEN as usize] | board.pieces[BLACK as usize][QUEEN as usize],
            board.pieces[WHITE as usize][ROOK as usize] | board.pieces[BLACK as usize][ROOK as usize],
            board.pieces[WHITE as usize][BISHOP as usize] | board.pieces[BLACK as usize][BISHOP as usize],
            board.pieces[WHITE as usize][KNIGHT as usize] | board.pieces[BLACK as usize][KNIGHT as usize],
            board.pieces[WHITE as usize][PAWN as usize] | board.pieces[BLACK as usize][PAWN as usize],
            board.halfmove_clock as u32,
            0,
            0,
            board.active_color == WHITE,
            ptr::null_mut(),
        )
    };

    let wdl = ((result & TB_RESULT_WDL_MASK) >> TB_RESULT_WDL_SHIFT);
    let wdl = match wdl {
        TB_WIN => WdlResult::Win,
        TB_LOSS => WdlResult::Loss,
        _ => WdlResult::Draw,
    };
    let dtz = ((result & TB_RESULT_DTZ_MASK) >> TB_RESULT_DTZ_SHIFT);
    let success = result != TB_RESULT_FAILED;

    if !success {
        return (false, wdl, dtz, Default::default());
    }

    let mut moves: [MaybeUninit<Move>; engine::MAX_MOVES_COUNT] = [MaybeUninit::uninit(); engine::MAX_MOVES_COUNT];
    let moves_count = board.get_all_moves(&mut moves, u64::MAX);

    let from = ((result & TB_RESULT_FROM_MASK) >> TB_RESULT_FROM_SHIFT) as u8;
    let to = ((result & TB_RESULT_TO_MASK) >> TB_RESULT_TO_SHIFT) as u8;
    let promotion = ((result & TB_RESULT_PROMOTES_MASK) >> TB_RESULT_PROMOTES_SHIFT);

    let promotion_flags = match promotion {
        TB_PROMOTES_QUEEN => MoveFlags::QUEEN_PROMOTION,
        TB_PROMOTES_ROOK => MoveFlags::ROOK_PROMOTION,
        TB_PROMOTES_BISHOP => MoveFlags::BISHOP_PROMOTION,
        TB_PROMOTES_KNIGHT => MoveFlags::KNIGHT_PROMOTION,
        _ => MoveFlags::QUIET,
    };

    for r#move in &moves[0..moves_count] {
        let r#move = unsafe { r#move.assume_init() };
        if r#move.get_from() == from && r#move.get_to() == to {
            let flags = r#move.get_flags();
            if promotion == 0 || (flags & promotion_flags).bits() == flags.bits() {
                return (success, wdl, dtz, r#move);
            }
        }
    }

    (false, wdl, dtz, Default::default())
}
