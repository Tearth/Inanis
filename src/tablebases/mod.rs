use crate::engine::*;
use crate::state::movescan::Move;
use crate::state::representation::Board;
use std::cmp;

pub mod syzygy;

#[derive(PartialEq, Eq, Debug)]
pub enum WdlResult {
    Win,
    Draw,
    Loss,
}

pub struct WdlDtzResult {
    pub wdl: WdlResult,
    pub dtz: u32,
    pub r#move: Move,
}

impl WdlDtzResult {
    /// Constructs a new instance of [WdlDtzResult] with stored `wdl`, `dtz` and `r#move`.
    pub fn new(wdl: WdlResult, dtz: u32, r#move: Move) -> Self {
        WdlDtzResult { wdl, dtz, r#move }
    }
}

/// Checks if there's a tablebase move (only Syzygy supported for now) and returns it as [Some], otherwise [None].
pub fn get_tablebase_move(board: &Board, probe_limit: u32) -> Option<(Move, i16)> {
    if board.get_pieces_count() > cmp::min(probe_limit as u8, syzygy::probe::get_max_pieces_count()) {
        return None;
    }

    if let Some(result) = syzygy::probe::get_root_wdl_dtz(board) {
        let score = match result.wdl {
            WdlResult::Win => TBMATE_SCORE,
            WdlResult::Draw => 0,
            WdlResult::Loss => -TBMATE_SCORE,
        };

        return Some((result.r#move, score));
    }

    None
}
