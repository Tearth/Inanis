use super::history::HistoryTable;
use super::killers::KillersTable;
use super::*;
use crate::cache::pawns::PawnHashTable;
use crate::cache::search::TranspositionTable;
use crate::engine::clock;
use crate::state::board::Bitboard;
use crate::state::movescan::Move;
use chrono::DateTime;
use chrono::Utc;

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
    pub pawn_hash_table: &'a mut PawnHashTable,
    pub killers_table: &'a mut KillersTable,
    pub history_table: &'a mut HistoryTable,
}

pub struct SearchResult {
    pub time: u64,
    pub depth: i32,
    pub score: i16,
    pub best_move: Move,
    pub statistics: SearchStatistics,
}

#[derive(Default, Copy, Clone)]
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

    pub pvs_full_window_searches: u64,
    pub pvs_zero_window_searches: u64,
    pub pvs_rejected_searches: u64,

    pub null_window_searches: u64,
    pub null_window_accepted: u64,
    pub null_window_rejected: u64,

    pub tt_added: u64,
    pub tt_hits: u64,
    pub tt_misses: u64,
    pub tt_collisions: u64,

    pub pawn_hash_table_added: u64,
    pub pawn_hash_table_hits: u64,
    pub pawn_hash_table_misses: u64,
    pub pawn_hash_table_collisions: u64,
}

impl<'a> SearchContext<'a> {
    pub fn new(
        board: &'a mut Bitboard,
        time: u32,
        inc_time: u32,
        transposition_table: &'a mut TranspositionTable,
        pawn_hash_table: &'a mut PawnHashTable,
        killers_table: &'a mut KillersTable,
        history_table: &'a mut HistoryTable,
    ) -> SearchContext<'a> {
        SearchContext {
            board,
            statistics: Default::default(),
            time,
            inc_time,
            current_depth: 1,
            search_time_start: Utc::now(),
            last_search_time: 1.0,
            deadline: 0,
            search_done: false,
            aborted: false,
            transposition_table,
            pawn_hash_table,
            killers_table,
            history_table,
        }
    }
}

impl<'a> Iterator for SearchContext<'a> {
    type Item = SearchResult;

    fn next(&mut self) -> Option<Self::Item> {
        if self.search_done || self.current_depth >= 32 {
            return None;
        }

        // Make sure we have at least one depth done before abort
        if self.current_depth > 1 {
            self.deadline = clock::get_time_for_move(self.time, self.inc_time) * 2;
        } else {
            self.deadline = u32::MAX;
        }

        let score = search::run::<true>(self, self.current_depth, 0, -32000, 32000, true);
        let search_time = (Utc::now() - self.search_time_start).num_milliseconds() as f64;
        let time_ratio = search_time / (self.last_search_time as f64);

        if self.aborted {
            return None;
        }

        if is_score_near_checkmate(score) || search_time * time_ratio > clock::get_time_for_move(self.time, self.inc_time) as f64 {
            self.search_done = true;
        }

        if search_time > 0.0 {
            self.last_search_time = search_time;
        }

        self.current_depth += 1;

        let total_search_time = (Utc::now() - self.search_time_start).num_milliseconds() as u64;
        let best_move = self.transposition_table.get_best_move(self.board.hash).unwrap();

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
