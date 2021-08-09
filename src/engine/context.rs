use crate::board::movescan::Move;
use crate::board::representation::Bitboard;
use crate::engine::clock;
use crate::run_search;
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
    pub search_done: bool,
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
}

impl<'a> SearchContext<'a> {
    pub fn new(board: &mut Bitboard, time: u32, inc_time: u32) -> SearchContext {
        SearchContext {
            board,
            statistics: SearchStatistics::new(),
            time,
            inc_time,
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
        let (score, best_move) = run_search!(self.board.active_color, self, self.current_depth, -32000, 32000, false);
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
        }
    }
}
