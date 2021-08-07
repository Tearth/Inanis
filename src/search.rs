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
}

impl<'a> SearchContext<'a> {
    pub fn new(board: &mut Bitboard) -> SearchContext {
        SearchContext {
            board: board,
            statistics: SearchStatistics::new(),
        }
    }
}

pub struct SearchStatistics {
    pub nodes_count: u64,
    pub q_nodes_count: u64,
    pub leafs_count: u64,
    pub q_leafs_count: u64,
    pub beta_cutoffs: u64,
    pub q_beta_cutoffs: u64,
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
        }
    }
}

pub fn run(board: &mut Bitboard, time: u32, inc_time: u32) -> Move {
    let time_for_move = clock::get_time_for_move(time, inc_time);
    let mut context = SearchContext::new(board);

    let mut last_search_time = 1.0;
    for depth in 1..32 {
        let search_time_start = Utc::now();

        let (score, best_move) = run_internal!(context.board.active_color, &mut context, depth, -32000, 32000, false);
        let search_time = (Utc::now() - search_time_start).num_microseconds().unwrap() as f64 / 1000.0;
        let time_ratio = search_time / (last_search_time as f64);

        // Temporary
        println!(
            "info score cp {} nodes 0 depth {} time {} pv {}",
            score,
            depth,
            search_time as u32,
            best_move.to_text()
        );

        if search_time * time_ratio > time_for_move as f64 {
            return best_move;
        }

        last_search_time = search_time;
    }

    Move::new(0, 0, MoveFlags::QUIET)
}

pub fn run_fixed_depth(board: &mut Bitboard, depth: i32) -> (Move, SearchStatistics) {
    let mut context = SearchContext::new(board);
    let mut best_move = Move::new(0, 0, MoveFlags::QUIET);

    for depth in 1..=depth {
        let (_, r#move) = run_internal!(context.board.active_color, &mut context, depth, -32000, 32000, false);
        best_move = r#move;
    }

    (best_move, context.statistics)
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
    for r#move in &moves[0..moves_count] {
        context.board.make_move::<COLOR>(r#move);
        let (search_score, _) = run_internal!(COLOR, context, depth - 1, -beta, -alpha, true);
        let score = -search_score;
        context.board.undo_move::<COLOR>(r#move);

        if score > alpha {
            alpha = score;
            best_move = *r#move;

            if alpha >= beta {
                context.statistics.beta_cutoffs += 1;
                break;
            }
        }
    }

    (alpha, best_move)
}
