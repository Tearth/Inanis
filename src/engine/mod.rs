pub mod clock;
pub mod context;
pub mod movesort;
pub mod params;
pub mod qsearch;
pub mod search;
pub mod see;
pub mod stats;

pub const MAX_DEPTH: i8 = 64;
pub const MIN_ALPHA: i16 = -32000;
pub const MIN_BETA: i16 = 32000;
pub const MAX_MOVES_COUNT: usize = 218;

pub const INVALID_SCORE: i16 = -32700;
pub const DRAW_SCORE: i16 = 0;
pub const CHECKMATE_SCORE: i16 = 31900;
pub const TBMATE_SCORE: i16 = 10000;

/// Checks if `score` is within mate range (from -[CHECKMATE_SCORE] to -[CHECKMATE_SCORE] + [MAX_DEPTH] and
/// from [CHECKMATE_SCORE] - [MAX_DEPTH] to [CHECKMATE_SCORE]).
pub fn is_score_near_checkmate(score: i16) -> bool {
    score.abs() >= CHECKMATE_SCORE - (MAX_DEPTH as i16) && score.abs() <= CHECKMATE_SCORE + (MAX_DEPTH as i16)
}
