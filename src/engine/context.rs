use self::params::SearchParameters;
use super::stats::SearchStatistics;
use super::*;
use crate::cache::counters::CountermovesTable;
use crate::cache::history::HistoryTable;
use crate::cache::killers::KillersTable;
use crate::cache::pawns::PawnHashTable;
use crate::cache::search::TranspositionTable;
use crate::engine::clock;
use crate::state::movescan::Move;
use crate::state::representation::Board;
use crate::utils::panic_fast;
use std::cmp;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::sync::RwLock;
use std::thread;
use std::time::SystemTime;

pub struct SearchContext {
    pub board: Board,
    pub parameters: SearchParameters,
    pub search_id: u8,
    pub time: u32,
    pub inc_time: u32,
    pub current_depth: i8,
    pub forced_depth: i8,
    pub max_nodes_count: u64,
    pub max_move_time: u32,
    pub moves_to_go: u32,
    pub moves_to_search: Vec<Move>,
    pub search_time_start: SystemTime,
    pub deadline: u32,
    pub multipv: bool,
    pub lines: Vec<SearchResultLine>,
    pub search_done: bool,
    pub uci_debug: bool,
    pub ponder_mode: bool,
    pub helper_thread: bool,
    pub syzygy_enabled: bool,
    pub syzygy_probe_limit: u32,
    pub syzygy_probe_depth: i8,
    pub transposition_table: Arc<TranspositionTable>,
    pub pawn_hashtable: Arc<PawnHashTable>,
    pub killers_table: KillersTable,
    pub history_table: HistoryTable,
    pub countermoves_table: CountermovesTable,
    pub helper_contexts: Arc<RwLock<Vec<SearchContext>>>,
    pub abort_flag: Arc<AtomicBool>,
    pub ponder_flag: Arc<AtomicBool>,
    pub statistics: SearchStatistics,
    pub last_score: i16,
}

pub struct SearchResult {
    pub time: u32,
    pub depth: i8,
}

pub struct SearchResultLine {
    pub score: i16,
    pub pv_line: Vec<Move>,
}

impl SearchContext {
    /// Constructs a new instance of [SearchContext] with parameters as follows:
    ///  - `board` - initial position of the board
    ///  - `parameters` - structure with all search parameters
    ///  - `helper_thread` - enables additional features when the thread is a helper in Lazy SMP (like random noise in move ordering)
    ///  - `syzygy_enabled` - enables or disables Syzygy probing
    ///  - `syzygy_probe_limit` - number of pieces for which the probing should be started
    ///  - `syzygy_probe_depth` - minimal depth at which the probing will be started
    ///  - `transposition_table`, `pawn_hashtable`, `killers_table`, `history_table`, `countermoves_table` - hashtables used during search
    ///  - `abort_flag` - flag used to abort search from the outside of the context
    ///  - `ponder_flag` - flag used to change a search mode from pondering to the regular one
    pub fn new(
        board: Board,
        parameters: SearchParameters,
        helper_thread: bool,
        transposition_table: Arc<TranspositionTable>,
        pawn_hashtable: Arc<PawnHashTable>,
        killers_table: KillersTable,
        history_table: HistoryTable,
        countermoves_table: CountermovesTable,
        abort_flag: Arc<AtomicBool>,
        ponder_flag: Arc<AtomicBool>,
    ) -> Self {
        Self {
            board,
            parameters,
            search_id: 0,
            time: 0,
            inc_time: 0,
            current_depth: 1,
            forced_depth: 0,
            max_nodes_count: 0,
            max_move_time: 0,
            moves_to_go: 0,
            moves_to_search: Vec::new(),
            search_time_start: SystemTime::now(),
            deadline: 0,
            multipv: false,
            lines: Vec::new(),
            search_done: false,
            uci_debug: false,
            ponder_mode: false,
            helper_thread,
            syzygy_enabled: false,
            syzygy_probe_limit: 0,
            syzygy_probe_depth: 0,
            transposition_table,
            pawn_hashtable,
            killers_table,
            history_table,
            countermoves_table,
            helper_contexts: Arc::new(RwLock::new(Vec::new())),
            abort_flag,
            ponder_flag,
            statistics: Default::default(),
            last_score: 0,
        }
    }
}

impl Iterator for SearchContext {
    type Item = SearchResult;

    /// Performs the next iteration of the search, using data stored in the context. Returns [None] if any of the following conditions is true:
    ///  - the search has been done in the previous iteration or the current depth is about to exceed [MAX_DEPTH] value
    ///  - `self.forced_depth` is not 0 and the current depth is about to exceed this value
    ///  - instant move is possible
    ///  - Syzygy tablebase move is possible
    ///  - time allocated for the current search has expired
    ///  - mate score has detected and was recognized as reliable
    ///  - search was aborted
    fn next(&mut self) -> Option<Self::Item> {
        // This loop works here as goto, which allows restarting search when switching from pondering mode to regular search within the same iteration
        loop {
            if self.search_done || self.current_depth >= MAX_DEPTH {
                return None;
            }

            if self.forced_depth != 0 && self.current_depth > self.forced_depth {
                return None;
            }

            // If the max depth was reached, but search is in ponder mode, wait for "ponderhit" or "stop" command before executing the last iteration
            if self.ponder_mode && self.forced_depth != 0 && self.current_depth == self.forced_depth {
                loop {
                    if self.abort_flag.load(Ordering::Relaxed) {
                        break;
                    }
                }
            }

            // Check instant move and Syzygy tablebase move only if there's no forced depth to reach
            if self.forced_depth == 0 && self.current_depth == 1 {
                if let Some(r#move) = self.board.get_instant_move() {
                    self.search_done = true;
                    self.lines.push(SearchResultLine::new(0, vec![r#move]));

                    return Some(SearchResult::new(0, self.current_depth));
                }

                if self.syzygy_enabled {
                    if let Some((r#move, score)) = self.board.get_tablebase_move(self.syzygy_probe_limit) {
                        self.search_done = true;
                        self.statistics.tb_hits = 1;
                        self.lines.push(SearchResultLine::new(score, vec![r#move]));

                        return Some(SearchResult::new(0, self.current_depth));
                    }
                }
            }

            let desired_time = if self.max_move_time != 0 {
                self.max_move_time
            } else {
                let desired_time = clock::get_time_for_move(self.board.fullmove_number, self.time, self.inc_time, self.moves_to_go);

                // Desired time can't exceed the whole available time
                cmp::min(desired_time, self.time)
            };

            self.deadline = if self.max_move_time != 0 {
                self.max_move_time
            } else if self.current_depth > 1 {
                let deadline = ((desired_time as f32) * DEADLINE_MULTIPLIER) as u32;

                // Deadline can't exceed the whole available time
                cmp::min(deadline, self.time)
            } else {
                u32::MAX
            };

            self.lines.clear();

            let helper_contexts_arc = self.helper_contexts.clone();
            let mut helper_contexts_lock = helper_contexts_arc.write().unwrap();

            thread::scope(|scope| {
                let depth = self.current_depth;
                let mut threads = Vec::new();

                for helper_context in helper_contexts_lock.iter_mut() {
                    helper_context.forced_depth = depth;
                    threads.push(scope.spawn(move || {
                        search::run(helper_context, depth);
                    }));
                }

                search::run(self, self.current_depth);

                let reset_abort_flag = !self.abort_flag.load(Ordering::Relaxed);
                self.abort_flag.store(true, Ordering::Relaxed);

                for thread in threads {
                    thread.join().unwrap();
                }

                if reset_abort_flag {
                    self.abort_flag.store(false, Ordering::Relaxed);
                }
            });

            for helper_context in helper_contexts_lock.iter() {
                self.statistics += &helper_context.statistics;
            }

            if self.abort_flag.load(Ordering::Relaxed) {
                // If ponder flag is set, the search is completly restarted within the same iteration
                if self.ponder_flag.load(Ordering::Relaxed) {
                    self.current_depth = 1;
                    self.search_time_start = SystemTime::now();
                    self.statistics = Default::default();

                    self.ponder_flag.store(false, Ordering::Relaxed);
                    self.abort_flag.store(false, Ordering::Relaxed);

                    continue;
                } else {
                    if self.uci_debug {
                        println!("info string Search aborted");
                    }

                    return None;
                }
            }

            if self.lines.is_empty() || self.lines[0].pv_line.is_empty() {
                println!("info string Invalid position");
                return None;
            }

            if self.lines[0].pv_line[0].is_empty() {
                panic_fast!("Invalid PV move: {}", self.lines[0].pv_line[0]);
            }

            let search_time = self.search_time_start.elapsed().unwrap().as_millis() as u32;

            self.lines.sort_by(|a, b| a.score.cmp(&b.score).reverse());
            self.current_depth += 1;

            if self.forced_depth == 0 && self.max_nodes_count == 0 {
                if search_time > ((desired_time as f32) * TIME_THRESHOLD_RATIO) as u32 {
                    self.search_done = true;
                }

                // Checkmate score must indicate that the depth it was found is equal or smaller than the current one, to prevent endless move sequences
                if is_score_near_checkmate(self.lines[0].score) && self.current_depth >= (CHECKMATE_SCORE - self.lines[0].score.abs()) as i8 {
                    self.search_done = true;
                }
            }

            return Some(SearchResult::new(search_time, self.current_depth - 1));
        }
    }
}

impl SearchResult {
    /// Constructs a new instance of [SearchResult] with stored `time` and `depth`.
    pub fn new(time: u32, depth: i8) -> Self {
        Self { time, depth }
    }
}

impl SearchResultLine {
    /// Constructs a new instance of [SearchResultLine] with stored `score` and `pv_line`.
    pub fn new(score: i16, pv_line: Vec<Move>) -> Self {
        Self { score, pv_line }
    }
}
