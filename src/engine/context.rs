use super::*;
use crate::cache::history::HistoryTable;
use crate::cache::killers::KillersTable;
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
use crate::tablebases::syzygy;
use crate::tablebases::WdlResult;
use std::cmp;
use std::mem::MaybeUninit;
use std::ops;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::thread;
use std::time::SystemTime;

pub struct SearchContext {
    pub board: Bitboard,
    pub statistics: SearchStatistics,
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
    pub multipv_lines: Vec<SearchResultLine>,
    pub search_done: bool,
    pub uci_debug: bool,
    pub ponder_mode: bool,
    pub diagnostic_mode: bool,
    pub helper_thread: bool,
    pub syzygy_enabled: bool,
    pub syzygy_probe_limit: u32,
    pub syzygy_probe_depth: i8,
    pub transposition_table: Arc<TranspositionTable>,
    pub pawn_hashtable: Arc<PawnHashTable>,
    pub killers_table: Arc<KillersTable>,
    pub history_table: Arc<HistoryTable>,
    pub helper_contexts: Vec<HelperThreadContext>,
    pub abort_flag: Arc<AtomicBool>,
    pub ponder_flag: Arc<AtomicBool>,
}

pub struct HelperThreadContext {
    pub board: Bitboard,
    pub pawn_hashtable: Arc<PawnHashTable>,
    pub killers_table: Arc<KillersTable>,
    pub history_table: Arc<HistoryTable>,
    pub context: SearchContext,
}

pub struct SearchResult {
    pub time: u32,
    pub depth: i8,
    pub transposition_table_usage: f32,
    pub lines: Vec<SearchResultLine>,
    pub statistics: SearchStatistics,
}

#[derive(Clone)]
pub struct SearchResultLine {
    pub score: i16,
    pub pv_line: Vec<Move>,
}

#[derive(Default, Copy, Clone)]
pub struct SearchStatistics {
    pub nodes_count: u64,
    pub q_nodes_count: u64,
    pub leafs_count: u64,
    pub q_leafs_count: u64,
    pub beta_cutoffs: u64,
    pub q_beta_cutoffs: u64,

    pub tb_hits: u64,

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

    pub late_move_pruning_accepted: u64,
    pub late_move_pruning_rejected: u64,

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

    pub tt_legal_hashmoves: u64,
    pub tt_illegal_hashmoves: u64,

    pub pawn_hashtable_added: u64,
    pub pawn_hashtable_hits: u64,
    pub pawn_hashtable_misses: u64,

    pub move_generator_hash_move_stages: u64,
    pub move_generator_captures_stages: u64,
    pub move_generator_quiet_moves_stages: u64,

    pub max_ply: u16,
}

impl SearchContext {
    /// Constructs a new instance of [SearchContext] with parameters as follows:
    ///  - `board` - initial position of the board
    ///  - `search_id` - search identificator used to recognize old transposition table entries
    ///  - `time` - total time for the color in a move (in milliseconds)
    ///  - `inc_time` - incremental time for the color in a move (in milliseconds)
    ///  - `forced_depth` - depth at which the search will stop (might happen earlier if mate is detected), 0 if there is no constraint
    ///  - `max_nodes_count` - total nodes count at which the search will top (might happen earlier if mate is detected), 0 if there is no constraint
    /// This value can possibly not be strictly respected due to way of how the check is performed, so expect a bit more nodes count before stop
    ///  - `max_move_time` - allocated amount of time for the search (in milliseconds), 0 if we want to use default time allocator
    ///  - `moves_to_go` - moves count, after which the time will be increased
    ///  - `moves_to_search` - a list of moves to which the root node will be restricted
    ///  - `multipv` - enables or disables analyzing multiple PV lines (might slow down search)
    ///  - `uci_debug` - enables or disables additional debug info sent to GUI by `info string` command
    ///  - `ponder_mode` - prevents search from being stopped after detecting a checkmate (ponder mode requirement)
    ///  - `diagnostic_mode` - enables gathering of additional statistics, useful for benchmarks
    ///  - `helper_thread` - enables additional features when the thread is a helper in Lazy SMP (like random noise in move ordering)
    ///  - `syzygy_enabled` - enables or disables Syzygy probing
    ///  - `syzygy_probe_limit` - number of pieces for which the probing should be started
    ///  - `syzygy_probe_depth` - minimal depth at which the probing will be started
    ///  - `transposition_table`, `pawn_hashtable`, `killers_table`, `history_table` - hashtables used during search
    ///  - `abort_flag` - flag used to abort search from the outside of the context
    ///  - `ponder_flag` - flag used to change a search mode from pondering to the regular one
    pub fn new(
        board: Bitboard,
        search_id: u8,
        time: u32,
        inc_time: u32,
        forced_depth: i8,
        max_nodes_count: u64,
        max_move_time: u32,
        moves_to_go: u32,
        moves_to_search: Vec<Move>,
        multipv: bool,
        uci_debug: bool,
        ponder_mode: bool,
        diagnostic_mode: bool,
        helper_thread: bool,
        syzygy_enabled: bool,
        syzygy_probe_limit: u32,
        syzygy_probe_depth: i8,
        transposition_table: Arc<TranspositionTable>,
        pawn_hashtable: Arc<PawnHashTable>,
        killers_table: Arc<KillersTable>,
        history_table: Arc<HistoryTable>,
        abort_flag: Arc<AtomicBool>,
        ponder_flag: Arc<AtomicBool>,
    ) -> Self {
        Self {
            board,
            statistics: Default::default(),
            search_id,
            time,
            inc_time,
            current_depth: 1,
            forced_depth,
            max_nodes_count,
            max_move_time,
            moves_to_go,
            moves_to_search,
            search_time_start: SystemTime::now(),
            deadline: 0,
            multipv,
            multipv_lines: Vec::new(),
            search_done: false,
            uci_debug,
            ponder_mode,
            diagnostic_mode,
            helper_thread,
            syzygy_enabled,
            syzygy_probe_limit,
            syzygy_probe_depth,
            transposition_table,
            pawn_hashtable,
            killers_table,
            history_table,
            helper_contexts: Vec::new(),
            abort_flag,
            ponder_flag,
        }
    }

    /// Retrieves PV line from the transposition table, using `board` position and the current `ply`.
    pub fn get_pv_line(&self, board: &mut Bitboard, ply: i8) -> Vec<Move> {
        if ply >= MAX_DEPTH {
            return Vec::new();
        }

        let mut pv_line = Vec::new();
        match self.transposition_table.get(board.hash, 0) {
            Some(entry) => {
                if entry.r#type != TranspositionTableScoreType::EXACT_SCORE {
                    return Vec::new();
                }

                let mut moves: [MaybeUninit<Move>; MAX_MOVES_COUNT] = [MaybeUninit::uninit(); MAX_MOVES_COUNT];
                let moves_count = board.get_all_moves(&mut moves, u64::MAX);

                if moves.iter().take(moves_count).any(|r#move| unsafe { r#move.assume_init() } == entry.best_move) {
                    board.make_move(entry.best_move);
                    if !board.is_king_checked(board.active_color ^ 1) {
                        pv_line.push(entry.best_move);
                        pv_line.append(&mut self.get_pv_line(board, ply + 1));
                    }
                    board.undo_move(entry.best_move);
                }
            }
            None => {
                return Vec::new();
            }
        }

        // Remove endless repetitions from PV line
        if pv_line.len() > 8 {
            if pv_line[0] == pv_line[4] && pv_line[4] == pv_line[8] {
                pv_line = pv_line[0..1].to_vec();
            }
        }

        pv_line
    }

    /// Checks if there's an instant move possible and returns it as [Some], otherwise [None].
    fn get_instant_move(&mut self) -> Option<Move> {
        if !self.board.is_king_checked(self.board.active_color) {
            return None;
        }

        let mut moves: [MaybeUninit<Move>; MAX_MOVES_COUNT] = [MaybeUninit::uninit(); MAX_MOVES_COUNT];
        let moves_count = self.board.get_all_moves(&mut moves, u64::MAX);

        let mut evading_moves_count = 0;
        let mut evading_move = Default::default();

        for r#move in &moves[0..moves_count] {
            let r#move = unsafe { r#move.assume_init() };
            self.board.make_move(r#move);

            if !self.board.is_king_checked(self.board.active_color ^ 1) {
                evading_moves_count += 1;
                evading_move = r#move;

                if evading_moves_count > 1 {
                    self.board.undo_move(r#move);
                    return None;
                }
            }

            self.board.undo_move(r#move);
        }

        if evading_moves_count == 1 {
            return Some(evading_move);
        }

        None
    }

    /// Checks if there's a Syzygy tablebase move and returns it as [Some], otherwise [None].
    fn get_syzygy_move(&mut self) -> Option<(Move, i16)> {
        let board = self.board.clone();
        if board.get_pieces_count() > cmp::min(self.syzygy_probe_limit as u8, syzygy::probe::get_max_pieces_count()) {
            return None;
        }

        let (success_root, wdl_root, _dtz_root, r#move) = syzygy::probe::get_root_wdl_dtz(&board);
        if !success_root {
            return None;
        }

        let score = match wdl_root {
            WdlResult::Win => TBMATE_SCORE,
            WdlResult::Draw => 0,
            WdlResult::Loss => -TBMATE_SCORE,
        };

        Some((r#move, score))
    }
}

impl Iterator for SearchContext {
    type Item = SearchResult;

    /// Performs the next iteration of the search, using data stored withing the context. Returns [None] if any of the following conditions is true:
    ///  - `self.forced_depth` is not 0 and the current depth is about to exceed this value
    ///  - the search has been done in the previous iteration or the current depth is about to exceed [MAX_DEPTH] value
    ///  - instant move is possible
    ///  - Syzygy tablebase move is possible
    ///  - time allocated for the current search has expired
    ///  - mate score has detected and was recognized as reliable
    ///  - search was aborted
    fn next(&mut self) -> Option<Self::Item> {
        // This loop works here as goto, which allows restarting search when switching from pondering mode to regular search within the same iteration
        loop {
            if self.forced_depth != 0 && self.current_depth == self.forced_depth + 1 {
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

            if self.search_done || self.current_depth >= MAX_DEPTH {
                return None;
            }

            if self.forced_depth == 0 && self.current_depth == 1 {
                if let Some(r#move) = self.get_instant_move() {
                    self.search_done = true;
                    return Some(SearchResult::new(
                        0,
                        self.current_depth,
                        self.transposition_table.get_usage(1000),
                        vec![SearchResultLine::new(0, vec![r#move])],
                        self.statistics,
                    ));
                }

                if self.syzygy_enabled {
                    if let Some((r#move, score)) = self.get_syzygy_move() {
                        self.search_done = true;
                        self.statistics.tb_hits = 1;

                        return Some(SearchResult::new(
                            0,
                            self.current_depth,
                            self.transposition_table.get_usage(1000),
                            vec![SearchResultLine::new(score, vec![r#move])],
                            self.statistics,
                        ));
                    }
                }
            }

            let desired_time = if self.max_move_time != 0 {
                self.max_move_time
            } else {
                let mut desired_time = clock::get_time_for_move(self.board.fullmove_number, self.time, self.inc_time, self.moves_to_go);
                if desired_time > self.time {
                    desired_time = self.time;
                }

                desired_time
            };

            self.deadline = if self.max_move_time != 0 {
                self.max_move_time
            } else if self.current_depth > 1 {
                let mut deadline = ((desired_time as f32) * DEADLINE_MULTIPLIER) as u32;
                if deadline > self.time {
                    deadline = desired_time;
                }

                deadline
            } else {
                u32::MAX
            };

            if !self.helper_contexts.is_empty() {
                thread::scope(|scope| {
                    let depth = self.current_depth;
                    let mut threads = Vec::new();

                    for helper_context in &mut self.helper_contexts {
                        helper_context.context.deadline = self.deadline;
                        threads.push(scope.spawn(move || {
                            search::run::<false>(&mut helper_context.context, depth);
                            helper_context.context.statistics
                        }));
                    }

                    for thread in threads {
                        self.statistics += thread.join().unwrap();
                    }
                });
            }

            self.multipv_lines.clear();

            match self.diagnostic_mode {
                true => search::run::<true>(self, self.current_depth),
                false => search::run::<false>(self, self.current_depth),
            };

            let search_time = self.search_time_start.elapsed().unwrap().as_millis() as u32;
            if self.uci_debug {
                let mut white_attack_mask = 0;
                let mut black_attack_mask = 0;

                let game_phase = self.board.game_phase;
                let initial_game_phase = self.board.evaluation_parameters.initial_game_phase;

                let material_evaluation = material::evaluate(&self.board);
                let pst_evaluation = pst::evaluate(&self.board);
                let mobility_evaluation = mobility::evaluate(&self.board, &mut white_attack_mask, &mut black_attack_mask);
                let safety_evaluation = safety::evaluate(&self.board, white_attack_mask, black_attack_mask);
                let pawns_evaluation = pawns::evaluate_without_cache(&self.board);

                println!(
                    "info string search_time={}, desired_time={}, game_phase={}, material={}, pst={}, mobility={}, safety={}, pawns={}",
                    search_time,
                    desired_time,
                    game_phase,
                    material_evaluation,
                    pst_evaluation.taper_score(game_phase, initial_game_phase),
                    mobility_evaluation.taper_score(game_phase, initial_game_phase),
                    safety_evaluation.taper_score(game_phase, initial_game_phase),
                    pawns_evaluation.taper_score(game_phase, initial_game_phase)
                );
            }

            if self.abort_flag.load(Ordering::Relaxed) {
                if self.ponder_flag.load(Ordering::Relaxed) {
                    self.current_depth = 1;
                    self.forced_depth = 0;
                    self.search_time_start = SystemTime::now();
                    self.statistics = Default::default();

                    for helper_context in &mut self.helper_contexts {
                        helper_context.context.current_depth = 1;
                        helper_context.context.forced_depth = 0;
                        helper_context.context.search_time_start = SystemTime::now();
                        helper_context.context.statistics = Default::default();
                    }

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

            if self.multipv_lines.is_empty() || self.multipv_lines[0].pv_line.is_empty() {
                println!("info string Invalid position");
                return None;
            }

            if self.forced_depth == 0 && self.max_nodes_count == 0 {
                if search_time > ((desired_time as f32) * TIME_THRESHOLD_RATIO) as u32 {
                    self.search_done = true;
                }

                if is_score_near_checkmate(self.multipv_lines[0].score) && self.current_depth >= (CHECKMATE_SCORE - self.multipv_lines[0].score.abs()) as i8 {
                    self.search_done = true;
                }
            }

            if !self.multipv {
                let pv_line = self.get_pv_line(&mut self.board.clone(), 0);
                self.multipv_lines.push(SearchResultLine::new(self.multipv_lines[0].score, pv_line));
            }

            let mut multipv_result = self.multipv_lines.clone();
            multipv_result.sort_by(|a, b| a.score.cmp(&b.score).reverse());

            self.current_depth += 1;

            return Some(SearchResult::new(
                search_time,
                self.current_depth - 1,
                self.transposition_table.get_usage(1000),
                multipv_result,
                self.statistics,
            ));
        }
    }
}

impl SearchResult {
    /// Constructs a new instance of [SearchResult] with stored `time`, `depth`, `lines` and `statistics`.
    pub fn new(time: u32, depth: i8, transposition_table_usage: f32, lines: Vec<SearchResultLine>, statistics: SearchStatistics) -> Self {
        Self {
            time,
            depth,
            transposition_table_usage,
            lines,
            statistics,
        }
    }
}

impl SearchResultLine {
    /// Constructs a new instance of [SearchResultLine] with stored `score` and `pv_line`.
    pub fn new(score: i16, pv_line: Vec<Move>) -> Self {
        Self { score, pv_line }
    }
}

impl ops::AddAssign<SearchStatistics> for SearchStatistics {
    /// Implements `+=` operator for [SearchStatistics] by adding all corresponding squares together (except `max_ply`, where the highest value is taken).
    fn add_assign(&mut self, rhs: SearchStatistics) {
        self.nodes_count += rhs.nodes_count;
        self.q_nodes_count += rhs.q_nodes_count;
        self.leafs_count += rhs.leafs_count;
        self.q_leafs_count += rhs.q_leafs_count;
        self.beta_cutoffs += rhs.beta_cutoffs;
        self.q_beta_cutoffs += rhs.q_beta_cutoffs;

        self.tb_hits += rhs.tb_hits;

        self.perfect_cutoffs += rhs.perfect_cutoffs;
        self.q_perfect_cutoffs += rhs.q_perfect_cutoffs;
        self.non_perfect_cutoffs += rhs.non_perfect_cutoffs;
        self.q_non_perfect_cutoffs += rhs.q_non_perfect_cutoffs;

        self.pvs_full_window_searches += rhs.pvs_full_window_searches;
        self.pvs_zero_window_searches += rhs.pvs_zero_window_searches;
        self.pvs_rejected_searches += rhs.pvs_rejected_searches;

        self.static_null_move_pruning_attempts += rhs.static_null_move_pruning_attempts;
        self.static_null_move_pruning_accepted += rhs.static_null_move_pruning_accepted;
        self.static_null_move_pruning_rejected += rhs.static_null_move_pruning_rejected;

        self.null_move_pruning_attempts += rhs.null_move_pruning_attempts;
        self.null_move_pruning_accepted += rhs.null_move_pruning_accepted;
        self.null_move_pruning_rejected += rhs.null_move_pruning_rejected;

        self.late_move_pruning_accepted += rhs.late_move_pruning_accepted;
        self.late_move_pruning_rejected += rhs.late_move_pruning_rejected;

        self.razoring_attempts += rhs.razoring_attempts;
        self.razoring_accepted += rhs.razoring_accepted;
        self.razoring_rejected += rhs.razoring_rejected;

        self.q_score_pruning_accepted += rhs.q_score_pruning_accepted;
        self.q_score_pruning_rejected += rhs.q_score_pruning_rejected;

        self.q_futility_pruning_accepted += rhs.q_futility_pruning_accepted;
        self.q_futility_pruning_rejected += rhs.q_futility_pruning_rejected;

        self.tt_added += rhs.tt_added;
        self.tt_hits += rhs.tt_hits;
        self.tt_misses += rhs.tt_misses;

        self.tt_legal_hashmoves += rhs.tt_legal_hashmoves;
        self.tt_illegal_hashmoves += rhs.tt_illegal_hashmoves;

        self.pawn_hashtable_added += rhs.pawn_hashtable_added;
        self.pawn_hashtable_hits += rhs.pawn_hashtable_hits;
        self.pawn_hashtable_misses += rhs.pawn_hashtable_misses;

        self.move_generator_hash_move_stages += rhs.move_generator_hash_move_stages;
        self.move_generator_captures_stages += rhs.move_generator_captures_stages;
        self.move_generator_quiet_moves_stages += rhs.move_generator_quiet_moves_stages;

        self.max_ply = cmp::max(self.max_ply, rhs.max_ply);
    }
}
