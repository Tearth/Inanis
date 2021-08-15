use super::*;
use crate::cache::search::TranspositionTable;
use crate::engine::clock;
use crate::run_search;
use crate::state::board::Bitboard;
use crate::state::movescan::Move;
use chrono::DateTime;
use chrono::Utc;
use std::sync::Arc;

pub struct SearchContext<'a> {
    pub board: &'a mut Bitboard,
    pub statistics: SearchStatistics,
    pub time: u32,
    pub inc_time: u32,
    pub current_depth: i32,
    pub search_time_start: DateTime<Utc>,
    pub last_search_time: f64,
    pub deadline: u32,
    pub search_done: bool,
    pub aborted: bool,
    pub transposition_table: &'a mut TranspositionTable,
}

pub struct SearchResult {
    pub time: u64,
    pub depth: i32,
    pub score: i16,
    pub best_move: Move,
    pub statistics: SearchStatistics,
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

    pub tt_hits: u64,
    pub tt_misses: u64,
    pub tt_added_entries: u64,
}

impl<'a> SearchContext<'a> {
    pub fn new(board: &'a mut Bitboard, time: u32, inc_time: u32, transposition_table: &'a mut TranspositionTable) -> SearchContext<'a> {
        SearchContext {
            board,
            statistics: SearchStatistics::new(),
            time,
            inc_time,
            current_depth: 1,
            search_time_start: Utc::now(),
            last_search_time: 1.0,
            deadline: 0,
            search_done: false,
            aborted: false,
            transposition_table,
        }
    }
}

impl<'a> Iterator for SearchContext<'a> {
    type Item = SearchResult;

    fn next(&mut self) -> Option<Self::Item> {
        if self.search_done {
            return None;
        }

        // Make sure we have at least one depth done before abort
        if self.current_depth > 1 {
            self.deadline = clock::get_time_for_move(self.time, self.inc_time) * 2;
        } else {
            self.deadline = u32::MAX;
        }

        let score = run_search!(self.board.active_color, self, self.current_depth, 0, -32000, 32000, false);
        let search_time = (Utc::now() - self.search_time_start).num_milliseconds() as f64;
        let time_ratio = search_time / (self.last_search_time as f64);

        if self.aborted {
            return None;
        }

        if is_score_near_checkmate(score) {
            self.search_done = true;
        }

        if search_time * time_ratio > clock::get_time_for_move(self.time, self.inc_time) as f64 {
            self.search_done = true;
        }

        if search_time > 0.0 {
            self.last_search_time = search_time;
        }

        self.current_depth += 1;

        let total_search_time = (Utc::now() - self.search_time_start).num_milliseconds() as u64;
        let best_move = self.transposition_table.get(self.board.hash, 0).best_move;

        Some(SearchResult::new(
            total_search_time,
            self.current_depth - 1,
            score,
            best_move,
            self.statistics,
        ))
    }
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

            tt_hits: 0,
            tt_misses: 0,
            tt_added_entries: 0,
        }
    }
}
