use super::history::HistoryTable;
use super::killers::KillersTable;
use super::*;
use crate::cache::pawns::PawnHashTable;
use crate::cache::search::TranspositionTable;
use crate::cache::search::TranspositionTableScoreType;
use crate::engine::clock;
use crate::evaluation::material;
use crate::evaluation::mobility;
use crate::evaluation::pawns;
use crate::evaluation::pst;
use crate::evaluation::safety;
use crate::state::board::Bitboard;
use crate::state::movescan::Move;
use chrono::DateTime;
use chrono::Utc;
use std::mem::MaybeUninit;

#[derive(Default)]
pub struct AbortToken {
    pub aborted: bool,
}

pub struct SearchContext<'a> {
    pub board: &'a mut Bitboard,
    pub statistics: SearchStatistics,
    pub time: u32,
    pub inc_time: u32,
    pub current_depth: i8,
    pub forced_depth: i8,
    pub max_nodes_count: u64,
    pub max_move_time: u32,
    pub moves_to_go: u32,
    pub search_time_start: DateTime<Utc>,
    pub last_search_time: f64,
    pub deadline: u32,
    pub search_done: bool,
    pub uci_debug: bool,
    pub transposition_table: &'a mut TranspositionTable,
    pub pawn_hashtable: &'a mut PawnHashTable,
    pub killers_table: &'a mut KillersTable,
    pub history_table: &'a mut HistoryTable,
    pub abort_token: &'a mut AbortToken,
}

pub struct SearchResult {
    pub time: u64,
    pub depth: i8,
    pub score: i16,
    pub pv_line: Vec<Move>,
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

    pub static_null_move_pruning_attempts: u64,
    pub static_null_move_pruning_accepted: u64,
    pub static_null_move_pruning_rejected: u64,

    pub null_move_pruning_attempts: u64,
    pub null_move_pruning_accepted: u64,
    pub null_move_pruning_rejected: u64,

    pub razoring_attempts: u64,
    pub razoring_accepted: u64,
    pub razoring_rejected: u64,

    pub q_score_pruning_accepted: u64,
    pub q_score_pruning_rejected: u64,

    pub q_futility_pruning_accepted: u64,
    pub q_futility_pruning_rejected: u64,

    pub tt_added: u64,
    pub tt_hits: u64,
    pub tt_misses: u64,
    pub tt_collisions: u64,

    pub pawn_hashtable_added: u64,
    pub pawn_hashtable_hits: u64,
    pub pawn_hashtable_misses: u64,
    pub pawn_hashtable_collisions: u64,

    pub move_generator_empty_stages: u64,
    pub move_generator_hash_move_stages: u64,
    pub move_generator_all_generated_stages: u64,

    pub max_ply: u16,
}

impl<'a> SearchContext<'a> {
    pub fn new(
        board: &'a mut Bitboard,
        time: u32,
        inc_time: u32,
        forced_depth: i8,
        max_nodes_count: u64,
        max_move_time: u32,
        moves_to_go: u32,
        uci_debug: bool,
        transposition_table: &'a mut TranspositionTable,
        pawn_hashtable: &'a mut PawnHashTable,
        killers_table: &'a mut KillersTable,
        history_table: &'a mut HistoryTable,
        abort_token: &'a mut AbortToken,
    ) -> SearchContext<'a> {
        SearchContext {
            board,
            statistics: Default::default(),
            time,
            inc_time,
            current_depth: 1,
            forced_depth,
            max_nodes_count,
            max_move_time,
            moves_to_go,
            search_time_start: Utc::now(),
            last_search_time: 1.0,
            deadline: 0,
            search_done: false,
            uci_debug,
            transposition_table,
            pawn_hashtable,
            killers_table,
            history_table,
            abort_token,
        }
    }

    fn get_pv_line(&mut self, board: &mut Bitboard, ply: i8) -> Vec<Move> {
        if ply >= MAX_DEPTH {
            return Vec::new();
        }

        let mut pv_line = Vec::new();
        let mut collision = false;
        match self.transposition_table.get(board.hash, 0, &mut collision) {
            Some(entry) => {
                if entry.get_flags() != TranspositionTableScoreType::EXACT_SCORE {
                    return Vec::new();
                }

                let mut moves: [Move; MAX_MOVES_COUNT] = unsafe { MaybeUninit::uninit().assume_init() };
                let moves_count = board.get_all_moves(&mut moves, u64::MAX);
                let mut found = false;

                for r#move in &moves[0..moves_count] {
                    if *r#move == entry.best_move {
                        found = true;
                        break;
                    }
                }

                if found {
                    board.make_move(&entry.best_move);
                    if !board.is_king_checked(board.active_color ^ 1) {
                        pv_line.push(entry.best_move);
                        pv_line.append(&mut self.get_pv_line(board, ply + 1));
                    }
                    board.undo_move(&entry.best_move);
                }
            }
            None => {
                return Vec::new();
            }
        }

        pv_line
    }
}

impl<'a> Iterator for SearchContext<'a> {
    type Item = SearchResult;

    fn next(&mut self) -> Option<Self::Item> {
        if self.forced_depth != 0 && self.current_depth == self.forced_depth + 1 {
            return None;
        }

        if self.search_done || self.current_depth >= MAX_DEPTH {
            return None;
        }

        let desired_time = if self.max_move_time != 0 {
            self.max_move_time
        } else {
            clock::get_time_for_move(self.time, self.inc_time, self.moves_to_go)
        };

        self.deadline = if self.max_move_time != 0 {
            self.max_move_time
        } else if self.current_depth > 1 {
            ((desired_time as f32) * DEADLINE_MULTIPLIER) as u32
        } else {
            u32::MAX
        };

        if self.uci_debug {
            let mut white_attack_mask = 0;
            let mut black_attack_mask = 0;

            let material_evaluation = material::evaluate(self.board);
            let pst_evaluation = pst::evaluate(self.board);
            let mobility_evaluation = mobility::evaluate(self.board, &mut white_attack_mask, &mut black_attack_mask);
            let safety_evaluation = safety::evaluate(self.board, white_attack_mask, black_attack_mask);
            let pawns_evaluation = pawns::evaluate_without_cache(self.board);

            println!(
                "info string desired_time={}, material={}, pst={}, mobility={}, safety={}, pawns={}",
                desired_time, material_evaluation, pst_evaluation, mobility_evaluation, safety_evaluation, pawns_evaluation
            );
        }

        let king_checked = self.board.is_king_checked(self.board.active_color);
        let score = search::run::<true>(self, self.current_depth, 0, MIN_ALPHA, MIN_BETA, true, king_checked);
        let search_time = (Utc::now() - self.search_time_start).num_milliseconds() as f64;
        let time_ratio = search_time / (self.last_search_time as f64);

        if self.abort_token.aborted {
            return None;
        }

        if self.forced_depth == 0 && self.max_nodes_count == 0 {
            if search_time * time_ratio > desired_time as f64 {
                self.search_done = true;
            }

            if is_score_near_checkmate(score) && self.current_depth >= (CHECKMATE_SCORE - score.abs()) as i8 {
                self.search_done = true;
            }
        }

        if search_time > 0.0 {
            self.last_search_time = search_time;
        }

        self.current_depth += 1;

        let total_search_time = (Utc::now() - self.search_time_start).num_milliseconds() as u64;
        let pv_line = self.get_pv_line(&mut self.board.clone(), 0);

        Some(SearchResult::new(total_search_time, self.current_depth - 1, score, pv_line, self.statistics))
    }
}

impl SearchResult {
    pub fn new(time: u64, depth: i8, score: i16, pv_line: Vec<Move>, statistics: SearchStatistics) -> SearchResult {
        SearchResult {
            time,
            depth,
            score,
            pv_line,
            statistics,
        }
    }
}
