use chrono::DateTime;
use chrono::Utc;

use crate::board::Bitboard;
use crate::clock;
use crate::common::*;
use crate::movescan::Move;
use crate::movescan::MoveFlags;
use crate::qsearch;
use std::mem::MaybeUninit;

macro_rules! run_internal {
    ($color:expr, $context:expr, $depth:expr, $alpha:expr, $beta:expr, $invert:expr) => {
        match $invert {
            true => match $color {
                WHITE => run_internal::<BLACK>($context, $depth, $alpha, $beta),
                BLACK => run_internal::<WHITE>($context, $depth, $alpha, $beta),
                _ => panic!("Invalid value: $color={}", $color),
            },
            false => match $color {
                WHITE => run_internal::<WHITE>($context, $depth, $alpha, $beta),
                BLACK => run_internal::<BLACK>($context, $depth, $alpha, $beta),
                _ => panic!("Invalid value: $color={}", $color),
            },
        }
    };
}

pub struct SearchContext<'a> {
    pub board: &'a mut Bitboard,
    pub statistics: SearchStatistics,
    pub time: u32,
    pub inc_time: u32,
    pub current_depth: i32,
    pub search_time_start: DateTime<Utc>,
    pub last_search_time: f64,
    pub search_done: bool,
}

impl<'a> SearchContext<'a> {
    pub fn new(board: &mut Bitboard, time: u32, inc_time: u32) -> SearchContext {
        SearchContext {
            board: board,
            statistics: SearchStatistics::new(),
            time: time,
            inc_time: inc_time,
            current_depth: 1,
            search_time_start: Utc::now(),
            last_search_time: 1.0,
            search_done: false,
        }
    }
}

impl<'a> Iterator for SearchContext<'a> {
    type Item = SearchResult;

    fn next(&mut self) -> Option<Self::Item> {
        if self.search_done {
            return None;
        }

        let search_time_start = Utc::now();
        let (score, best_move) = run_internal!(self.board.active_color, self, self.current_depth, -32000, 32000, false);
        let search_time = (Utc::now() - search_time_start).num_microseconds().unwrap() as f64 / 1000.0;
        let time_ratio = search_time / (self.last_search_time as f64);

        if search_time * time_ratio > clock::get_time_for_move(self.time, self.inc_time) as f64 {
            self.search_done = true;
        }

        self.last_search_time = search_time;
        self.current_depth += 1;

        let total_search_time = (Utc::now() - self.search_time_start).num_milliseconds() as u64;
        Some(SearchResult::new(
            total_search_time,
            self.current_depth - 1,
            score,
            best_move,
            self.statistics,
        ))
    }
}

pub struct SearchResult {
    pub time: u64,
    pub depth: i32,
    pub score: i16,
    pub best_move: Move,
    pub statistics: SearchStatistics,
}

impl SearchResult {
    pub fn new(time: u64, depth: i32, score: i16, best_move: Move, statistics: SearchStatistics) -> SearchResult {
        SearchResult {
            time,
            depth,
            score,
            best_move,
            statistics,
        }
    }
}

#[derive(Copy, Clone)]
pub struct SearchStatistics {
    pub nodes_count: u64,
    pub q_nodes_count: u64,
    pub leafs_count: u64,
    pub q_leafs_count: u64,
    pub beta_cutoffs: u64,
    pub q_beta_cutoffs: u64,

    pub perfect_cutoffs: u64,
    pub q_perfect_cutoffs: u64,
    pub non_perfect_cutoffs: u64,
    pub q_non_perfect_cutoffs: u64,
}

impl SearchStatistics {
    fn new() -> SearchStatistics {
        SearchStatistics {
            nodes_count: 0,
            q_nodes_count: 0,
            leafs_count: 0,
            q_leafs_count: 0,
            beta_cutoffs: 0,
            q_beta_cutoffs: 0,

            perfect_cutoffs: 0,
            q_perfect_cutoffs: 0,
            non_perfect_cutoffs: 0,
            q_non_perfect_cutoffs: 0,
        }
    }
}

pub fn run_fixed_depth(board: &mut Bitboard, depth: i32) -> SearchResult {
    let mut context = SearchContext::new(board, 0, 0);
    let mut best_move = Move::new(0, 0, MoveFlags::QUIET);
    let mut best_score = 0;

    let search_time_start = Utc::now();
    for depth in 1..=depth {
        let (score, r#move) = run_internal!(context.board.active_color, &mut context, depth, -32000, 32000, false);

        best_score = score;
        best_move = r#move;
    }

    let time = (Utc::now() - search_time_start).num_milliseconds() as u64;
    SearchResult::new(time, depth, best_score, best_move, context.statistics)
}

fn run_internal<const COLOR: u8>(context: &mut SearchContext, depth: i32, mut alpha: i16, beta: i16) -> (i16, Move) {
    context.statistics.nodes_count += 1;

    if context.board.pieces[COLOR as usize][KING as usize] == 0 {
        context.statistics.leafs_count += 1;
        return (-31900 - (depth as i16), Move::new(0, 0, MoveFlags::QUIET));
    }

    if depth <= 0 {
        context.statistics.leafs_count += 1;
        return (
            qsearch::run::<COLOR>(context, depth, alpha, beta),
            Move::new(0, 0, MoveFlags::QUIET),
        );
    }

    let mut moves: [Move; 218] = unsafe { MaybeUninit::uninit().assume_init() };
    let moves_count = context.board.get_moves::<COLOR>(&mut moves);

    let mut best_move = Move::new(0, 0, MoveFlags::QUIET);
    for move_index in 0..moves_count {
        let r#move = moves[move_index];

        context.board.make_move::<COLOR>(&r#move);
        let (search_score, _) = run_internal!(COLOR, context, depth - 1, -beta, -alpha, true);
        let score = -search_score;
        context.board.undo_move::<COLOR>(&r#move);

        if score > alpha {
            alpha = score;
            best_move = r#move;

            if alpha >= beta {
                context.statistics.beta_cutoffs += 1;
                if move_index == 0 {
                    context.statistics.perfect_cutoffs += 1;
                } else {
                    context.statistics.non_perfect_cutoffs += 1;
                }

                break;
            }
        }
    }

    (alpha, best_move)
}
