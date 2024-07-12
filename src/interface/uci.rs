use crate::cache::allocator;
use crate::cache::history::HistoryTable;
use crate::cache::killers::KillersTable;
use crate::cache::pawns::PawnHashTable;
use crate::cache::search::TranspositionTable;
use crate::engine;
use crate::engine::context::HelperThreadContext;
use crate::engine::context::SearchContext;
use crate::engine::parameters::SearchParameters;
use crate::perft;
use crate::state::movescan::Move;
use crate::state::representation::Board;
use crate::state::*;
use crate::tablebases::syzygy;
use crate::utils::minmax::MinMax;
use std::cmp;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io;
use std::io::Write;
use std::panic;
use std::path::Path;
use std::process;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::SystemTime;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const AUTHOR: &str = env!("CARGO_PKG_AUTHORS");

pub struct UciState {
    board: Board,
    options: HashMap<String, UciOption>,
    transposition_table: Arc<TranspositionTable>,
    pawn_hashtable: Arc<PawnHashTable>,
    killers_table: Arc<KillersTable>,
    history_table: Arc<HistoryTable>,
    abort_flag: Arc<AtomicBool>,
    ponder_flag: Arc<AtomicBool>,
    debug_mode: AtomicBool,
}

#[derive(Clone)]
pub struct UciOption {
    pub order: u32,
    pub r#type: String,
    pub min: String,
    pub max: String,
    pub default: String,
    pub value: String,
}

impl UciOption {
    pub fn new<T>(order: u32, r#type: &str, min: T, max: T, default: T) -> Self
    where
        T: ToString,
    {
        Self { order, r#type: r#type.to_string(), min: min.to_string(), max: max.to_string(), default: default.to_string(), value: default.to_string() }
    }

    pub fn new_wide<T>(order: u32, default: T) -> Self
    where
        T: ToString + MinMax,
    {
        Self {
            order,
            r#type: "spin".to_string(),
            min: T::min().to_string(),
            max: T::max().to_string(),
            default: default.to_string(),
            value: default.to_string(),
        }
    }
}

impl Default for UciState {
    /// Constructs a default instance of [UciState] with zeroed elements and hashtables with their default sizes.
    fn default() -> Self {
        UciState {
            board: Board::new_initial_position(None, None, None, None, None),
            options: HashMap::new(),
            transposition_table: Arc::new(TranspositionTable::new(1 * 1024 * 1024)),
            pawn_hashtable: Arc::new(PawnHashTable::new(1 * 1024 * 1024)),
            killers_table: Arc::new(Default::default()),
            history_table: Arc::new(Default::default()),
            abort_flag: Arc::new(AtomicBool::new(false)),
            ponder_flag: Arc::new(AtomicBool::new(false)),
            debug_mode: AtomicBool::new(false),
        }
    }
}

/// Entry point of the UCI (Universal Chess Interface) and command loop.
pub fn run() {
    let state: Arc<Mutex<UciState>> = Arc::new(Mutex::new(Default::default()));
    let mut state_lock = state.lock().unwrap();

    println!("id name Inanis {}", VERSION);
    println!("id author {}", AUTHOR);

    state_lock.options.insert("Hash".to_string(), UciOption::new(0, "spin", 1, 1048576, 2));
    state_lock.options.insert("Move Overhead".to_string(), UciOption::new(1, "spin", 0, 3600000, 10));
    state_lock.options.insert("MultiPV".to_string(), UciOption::new(2, "spin", 1, 256, 1));
    state_lock.options.insert("Threads".to_string(), UciOption::new(3, "spin", 1, 1024, 1));
    state_lock.options.insert("SyzygyPath".to_string(), UciOption::new(4, "string", "", "", "<empty>"));
    state_lock.options.insert("SyzygyProbeLimit".to_string(), UciOption::new(5, "spin", 1, 9, 8));
    state_lock.options.insert("SyzygyProbeDepth".to_string(), UciOption::new(6, "spin", 1, 32, 6));
    state_lock.options.insert("Ponder".to_string(), UciOption::new(7, "check", false, false, false));
    state_lock.options.insert("Crash Files".to_string(), UciOption::new(8, "check", false, false, false));
    state_lock.options.insert("Clear Hash".to_string(), UciOption::new(9, "button", "", "", ""));

    #[cfg(feature = "dev")]
    {
        let parameters = SearchParameters::default();
        state_lock.options.insert("iir_min_depth".to_string(), UciOption::new_wide(99, parameters.iir_min_depth));
        state_lock.options.insert("iir_reduction_base".to_string(), UciOption::new_wide(99, parameters.iir_reduction_base));
        state_lock.options.insert("iir_reduction_step".to_string(), UciOption::new_wide(99, parameters.iir_reduction_step));
        state_lock.options.insert("iir_max_reduction".to_string(), UciOption::new_wide(99, parameters.iir_max_reduction));

        state_lock.options.insert("razoring_min_depth".to_string(), UciOption::new_wide(99, parameters.razoring_min_depth));
        state_lock.options.insert("razoring_max_depth".to_string(), UciOption::new_wide(99, parameters.razoring_max_depth));
        state_lock.options.insert("razoring_depth_margin_base".to_string(), UciOption::new_wide(99, parameters.razoring_depth_margin_base));
        state_lock.options.insert("razoring_depth_margin_multiplier".to_string(), UciOption::new_wide(99, parameters.razoring_depth_margin_multiplier));

        state_lock.options.insert("snmp_min_depth".to_string(), UciOption::new_wide(99, parameters.snmp_min_depth));
        state_lock.options.insert("snmp_max_depth".to_string(), UciOption::new_wide(99, parameters.snmp_max_depth));
        state_lock.options.insert("snmp_depth_margin_base".to_string(), UciOption::new_wide(99, parameters.snmp_depth_margin_base));
        state_lock.options.insert("snmp_depth_margin_multiplier".to_string(), UciOption::new_wide(99, parameters.snmp_depth_margin_multiplier));

        state_lock.options.insert("nmp_min_depth".to_string(), UciOption::new_wide(99, parameters.nmp_min_depth));
        state_lock.options.insert("nmp_min_game_phase".to_string(), UciOption::new_wide(99, parameters.nmp_min_game_phase));
        state_lock.options.insert("nmp_margin".to_string(), UciOption::new_wide(99, parameters.nmp_margin));
        state_lock.options.insert("nmp_depth_base".to_string(), UciOption::new_wide(99, parameters.nmp_depth_base));
        state_lock.options.insert("nmp_depth_divider".to_string(), UciOption::new_wide(99, parameters.nmp_depth_divider));

        state_lock.options.insert("lmp_min_depth".to_string(), UciOption::new_wide(99, parameters.lmp_min_depth));
        state_lock.options.insert("lmp_max_depth".to_string(), UciOption::new_wide(99, parameters.lmp_max_depth));
        state_lock.options.insert("lmp_move_index_margin_base".to_string(), UciOption::new_wide(99, parameters.lmp_move_index_margin_base));
        state_lock.options.insert("lmp_move_index_margin_multiplier".to_string(), UciOption::new_wide(99, parameters.lmp_move_index_margin_multiplier));
        state_lock.options.insert("lmp_max_score".to_string(), UciOption::new_wide(99, parameters.lmp_max_score));

        state_lock.options.insert("lmr_min_depth".to_string(), UciOption::new_wide(99, parameters.lmr_min_depth));
        state_lock.options.insert("lmr_max_score".to_string(), UciOption::new_wide(99, parameters.lmr_max_score));
        state_lock.options.insert("lmr_min_move_index".to_string(), UciOption::new_wide(99, parameters.lmr_min_move_index));
        state_lock.options.insert("lmr_reduction_base".to_string(), UciOption::new_wide(99, parameters.lmr_reduction_base));
        state_lock.options.insert("lmr_reduction_step".to_string(), UciOption::new_wide(99, parameters.lmr_reduction_step));
        state_lock.options.insert("lmr_max_reduction".to_string(), UciOption::new_wide(99, parameters.lmr_max_reduction));
        state_lock.options.insert("lmr_pv_min_move_index".to_string(), UciOption::new_wide(99, parameters.lmr_pv_min_move_index));
        state_lock.options.insert("lmr_pv_reduction_base".to_string(), UciOption::new_wide(99, parameters.lmr_pv_reduction_base));
        state_lock.options.insert("lmr_pv_reduction_step".to_string(), UciOption::new_wide(99, parameters.lmr_pv_reduction_step));
        state_lock.options.insert("lmr_pv_max_reduction".to_string(), UciOption::new_wide(99, parameters.lmr_pv_max_reduction));

        state_lock.options.insert("q_score_pruning_treshold".to_string(), UciOption::new_wide(99, parameters.q_score_pruning_treshold));
        state_lock.options.insert("q_futility_pruning_margin".to_string(), UciOption::new_wide(99, parameters.q_futility_pruning_margin));
    }

    let mut options_sorted = state_lock.options.iter().collect::<Vec<_>>();
    options_sorted.sort_by_key(|(_, option)| option.order);

    for (name, option) in options_sorted {
        match option.r#type.as_str() {
            "spin" => println!("option name {} type {} default {} min {} max {}", name, option.r#type, option.default, option.min, option.max),
            "string" => println!("option name {} type {} default {}", name, option.r#type, option.default),
            "check" => println!("option name {} type {} default {}", name, option.r#type, option.default),
            "button" => println!("option name {} type {}", name, option.r#type),
            _ => panic!("Invalid value: option.r#type={}", option.r#type),
        };
    }

    drop(state_lock);
    println!("uciok");

    loop {
        let mut input = String::new();
        let read_bytes = io::stdin().read_line(&mut input).unwrap();

        if read_bytes == 0 {
            process::exit(0);
        }

        let tokens: Vec<String> = input.split(' ').map(|v| v.trim().to_string()).collect();
        match tokens[0].to_lowercase().as_str() {
            "debug" => handle_debug(&tokens, state.clone()),
            "fen" => handle_fen(state.clone()),
            "go" => handle_go(&tokens, state.clone()),
            "isready" => handle_isready(),
            "ponderhit" => handle_ponderhit(state.clone()),
            "position" => handle_position(&tokens, state.clone()),
            "setoption" => handle_setoption(&tokens, state.clone()),
            "ucinewgame" => handle_ucinewgame(state.clone()),
            "stop" => handle_stop(state.clone()),
            "quit" => handle_quit(),
            _ => {}
        }
    }
}

/// Handles `debug [on/off]` command by setting the proper flag.
fn handle_debug(parameters: &[String], state: Arc<Mutex<UciState>>) {
    if parameters.len() < 2 {
        return;
    }

    state.lock().unwrap().debug_mode.store(matches!(parameters[1].as_str(), "on"), Ordering::Relaxed);
}

/// Handles non-standard `fen` command by printing FEN of the current position.
fn handle_fen(state: Arc<Mutex<UciState>>) {
    println!("info string {}", state.lock().unwrap().board);
}

/// Handles `go [parameters]` command by running a new search for a position which was set using `position` command. Supported parameters:
///  - `wtime x` - amount of total time for white in milliseconds
///  - `btime x` - amount of total time for black in milliseconds
///  - `winc x` - incremental time for white
///  - `binc x` - incremental time for black
///  - `depth x` - fixed depth, where the search will stop
///  - `nodes x` - fixed nodes count, after which the search will try to stop as soon as possible
///  - `movetime x` - fixed time allocated for the search in milliseconds
///  - `movestogo x` - amount of moves, after which the time will be increased
///  - `infinite` - tells the search to run until it reaches the maximal depth for the engine
///  - `searchmoves [moves]` - restricts search to the provided moves list
///  - `ponder` - tells the search to run in the ponder mode (thinking on the opponent's time)
fn handle_go(parameters: &[String], state: Arc<Mutex<UciState>>) {
    let mut white_time = u32::MAX;
    let mut black_time = u32::MAX;
    let mut white_inc_time = 0;
    let mut black_inc_time = 0;
    let mut forced_depth = 0;
    let mut max_nodes_count = 0;
    let mut max_move_time = 0;
    let mut moves_to_go = 0;
    let mut moves_to_search = Vec::new();
    let mut ponder_mode = false;
    let mut perft_mode = false;
    let mut perft_depth = 0;

    let mut state_lock = state.lock().unwrap();
    let mut iter = parameters[1..].iter().peekable();

    while let Some(token) = iter.next() {
        match token.as_str() {
            "wtime" => {
                white_time = match iter.peek() {
                    Some(value) => value.parse().unwrap_or(white_time),
                    None => white_time,
                }
            }
            "btime" => {
                black_time = match iter.peek() {
                    Some(value) => value.parse().unwrap_or(black_time),
                    None => black_time,
                }
            }
            "winc" => {
                white_inc_time = match iter.peek() {
                    Some(value) => value.parse().unwrap_or(white_inc_time),
                    None => white_inc_time,
                }
            }
            "binc" => {
                black_inc_time = match iter.peek() {
                    Some(value) => value.parse().unwrap_or(black_inc_time),
                    None => black_inc_time,
                }
            }
            "depth" => {
                forced_depth = match iter.peek() {
                    Some(value) => value.parse().unwrap_or(forced_depth),
                    None => forced_depth,
                }
            }
            "nodes" => {
                max_nodes_count = match iter.peek() {
                    Some(value) => value.parse().unwrap_or(max_nodes_count),
                    None => max_nodes_count,
                }
            }
            "movetime" => {
                max_move_time = match iter.peek() {
                    Some(value) => value.parse().unwrap_or(max_move_time),
                    None => max_move_time,
                }
            }
            "movestogo" => {
                moves_to_go = match iter.peek() {
                    Some(value) => value.parse().unwrap_or(moves_to_go),
                    None => moves_to_go,
                }
            }
            "infinite" => {
                forced_depth = engine::MAX_DEPTH;
            }
            "searchmoves" => {
                let keywords = ["wtime", "btime", "winc", "binc", "depth", "nodes", "movetime", "movestogo", "infinite", "searchmoves", "ponder"];

                while let Some(value) = iter.peek() {
                    if keywords.contains(&value.as_str()) {
                        break;
                    }

                    let parsed_move = match Move::from_long_notation(value, &state_lock.board) {
                        Ok(r#move) => r#move,
                        Err(error) => {
                            println!("info string Error: {}", error);
                            return;
                        }
                    };

                    moves_to_search.push(parsed_move);
                    iter.next();
                }
            }
            "ponder" => {
                ponder_mode = true;
                forced_depth = engine::MAX_DEPTH;
            }
            "perft" => {
                perft_mode = true;
                perft_depth = match iter.peek() {
                    Some(value) => value.parse().unwrap_or(perft_depth),
                    None => perft_depth,
                }
            }
            _ => {}
        }
    }

    if perft_mode {
        for depth in 1..=perft_depth {
            let now = SystemTime::now();
            let result = perft::normal::run(depth, &mut state_lock.board, false);
            let time = now.elapsed().unwrap().as_millis();

            println!("info time {} depth {} nodes {}", time, depth, result.nodes);
        }

        return;
    }

    let mut time = match state_lock.board.active_color {
        WHITE => white_time,
        BLACK => black_time,
        _ => panic!("Invalid value: state_lock.board.active_color={}", state_lock.board.active_color),
    };
    time -= cmp::min(time, state_lock.options["Move Overhead"].value.parse::<u32>().unwrap());

    let inc_time = match state_lock.board.active_color {
        WHITE => white_inc_time,
        BLACK => black_inc_time,
        _ => panic!("Invalid value: state_lock.board.active_color={}", state_lock.board.active_color),
    };

    state_lock.abort_flag.store(false, Ordering::Relaxed);
    state_lock.ponder_flag.store(false, Ordering::Relaxed);
    drop(state_lock);

    let state_arc = state.clone();
    thread::spawn(move || {
        let state_lock = state_arc.lock().unwrap();

        let multipv = state_lock.options["MultiPV"].value.parse::<u32>().unwrap();
        let threads = state_lock.options["Threads"].value.parse::<usize>().unwrap();
        let ponder = state_lock.options["Ponder"].value.parse::<bool>().unwrap();
        let syzygy_path = state_lock.options["SyzygyPath"].value.clone();
        let syzygy_enabled = !syzygy_path.is_empty() && syzygy_path != "<empty>";
        let syzygy_probe_limit = state_lock.options["SyzygyProbeLimit"].value.parse::<u32>().unwrap();
        let syzygy_probe_depth = state_lock.options["SyzygyProbeDepth"].value.parse::<i8>().unwrap();

        #[cfg(not(feature = "dev"))]
        let search_parameters = SearchParameters::default();

        #[cfg(feature = "dev")]
        let search_parameters = SearchParameters {
            iir_min_depth: state_lock.options["iir_min_depth"].value.parse().unwrap(),
            iir_reduction_base: state_lock.options["iir_reduction_base"].value.parse().unwrap(),
            iir_reduction_step: state_lock.options["iir_reduction_step"].value.parse().unwrap(),
            iir_max_reduction: state_lock.options["iir_max_reduction"].value.parse().unwrap(),

            razoring_min_depth: state_lock.options["razoring_min_depth"].value.parse().unwrap(),
            razoring_max_depth: state_lock.options["razoring_max_depth"].value.parse().unwrap(),
            razoring_depth_margin_base: state_lock.options["razoring_depth_margin_base"].value.parse().unwrap(),
            razoring_depth_margin_multiplier: state_lock.options["razoring_depth_margin_multiplier"].value.parse().unwrap(),

            snmp_min_depth: state_lock.options["snmp_min_depth"].value.parse().unwrap(),
            snmp_max_depth: state_lock.options["snmp_max_depth"].value.parse().unwrap(),
            snmp_depth_margin_base: state_lock.options["snmp_depth_margin_base"].value.parse().unwrap(),
            snmp_depth_margin_multiplier: state_lock.options["snmp_depth_margin_multiplier"].value.parse().unwrap(),

            nmp_min_depth: state_lock.options["nmp_min_depth"].value.parse().unwrap(),
            nmp_min_game_phase: state_lock.options["nmp_min_game_phase"].value.parse().unwrap(),
            nmp_margin: state_lock.options["nmp_margin"].value.parse().unwrap(),
            nmp_depth_base: state_lock.options["nmp_depth_base"].value.parse().unwrap(),
            nmp_depth_divider: state_lock.options["nmp_depth_divider"].value.parse().unwrap(),

            lmp_min_depth: state_lock.options["lmp_min_depth"].value.parse().unwrap(),
            lmp_max_depth: state_lock.options["lmp_max_depth"].value.parse().unwrap(),
            lmp_move_index_margin_base: state_lock.options["lmp_move_index_margin_base"].value.parse().unwrap(),
            lmp_move_index_margin_multiplier: state_lock.options["lmp_move_index_margin_multiplier"].value.parse().unwrap(),
            lmp_max_score: state_lock.options["lmp_max_score"].value.parse().unwrap(),

            lmr_min_depth: state_lock.options["lmr_min_depth"].value.parse().unwrap(),
            lmr_max_score: state_lock.options["lmr_max_score"].value.parse().unwrap(),
            lmr_min_move_index: state_lock.options["lmr_min_move_index"].value.parse().unwrap(),
            lmr_reduction_base: state_lock.options["lmr_reduction_base"].value.parse().unwrap(),
            lmr_reduction_step: state_lock.options["lmr_reduction_step"].value.parse().unwrap(),
            lmr_max_reduction: state_lock.options["lmr_max_reduction"].value.parse().unwrap(),
            lmr_pv_min_move_index: state_lock.options["lmr_pv_min_move_index"].value.parse().unwrap(),
            lmr_pv_reduction_base: state_lock.options["lmr_pv_reduction_base"].value.parse().unwrap(),
            lmr_pv_reduction_step: state_lock.options["lmr_pv_reduction_step"].value.parse().unwrap(),
            lmr_pv_max_reduction: state_lock.options["lmr_pv_max_reduction"].value.parse().unwrap(),

            q_score_pruning_treshold: state_lock.options["q_score_pruning_treshold"].value.parse().unwrap(),
            q_futility_pruning_margin: state_lock.options["q_futility_pruning_margin"].value.parse().unwrap(),
        };

        let mut context = SearchContext::new(
            state_lock.board.clone(),
            search_parameters.clone(),
            state_lock.board.state_stack.len() as u8,
            time,
            inc_time,
            forced_depth,
            max_nodes_count,
            max_move_time,
            moves_to_go,
            moves_to_search.clone(),
            multipv > 1,
            state_lock.debug_mode.load(Ordering::Relaxed),
            ponder_mode,
            false,
            false,
            syzygy_enabled,
            syzygy_probe_limit,
            syzygy_probe_depth,
            state_lock.transposition_table.clone(),
            state_lock.pawn_hashtable.clone(),
            state_lock.killers_table.clone(),
            state_lock.history_table.clone(),
            state_lock.abort_flag.clone(),
            state_lock.ponder_flag.clone(),
        );
        drop(state_lock);

        if threads > 1 {
            for _ in 0..threads {
                let state_lock = state_arc.lock().unwrap();
                let helper_context = SearchContext::new(
                    state_lock.board.clone(),
                    search_parameters.clone(),
                    state_lock.board.state_stack.len() as u8,
                    time,
                    inc_time,
                    forced_depth,
                    max_nodes_count,
                    max_move_time,
                    moves_to_go,
                    moves_to_search.clone(),
                    false,
                    state_lock.debug_mode.load(Ordering::Relaxed),
                    false,
                    false,
                    true,
                    false,
                    0,
                    0,
                    state_lock.transposition_table.clone(),
                    state_lock.pawn_hashtable.clone(),
                    state_lock.killers_table.clone(),
                    state_lock.history_table.clone(),
                    state_lock.abort_flag.clone(),
                    state_lock.ponder_flag.clone(),
                );
                drop(state_lock);

                let data = HelperThreadContext::new(
                    context.board.clone(),
                    Arc::new((*context.pawn_hashtable).clone()),
                    Arc::new((*context.killers_table).clone()),
                    Arc::new((*context.history_table).clone()),
                    helper_context,
                );

                context.helper_contexts.push(data);
            }
        }

        let mut best_move = Default::default();
        let mut ponder_move = Default::default();

        for depth_result in context {
            for (multipv_index, multipv_entry) in depth_result.lines.iter().take(multipv as usize).enumerate() {
                let pv_line: Vec<String> = multipv_entry.pv_line.iter().map(|v| v.to_long_notation()).collect();
                let formatted_score = if engine::is_score_near_checkmate(multipv_entry.score) {
                    let mut moves_to_mate = (multipv_entry.score.abs() - engine::CHECKMATE_SCORE).abs() / 2;
                    moves_to_mate *= multipv_entry.score.signum();

                    format!("score mate {}", moves_to_mate).to_string()
                } else {
                    format!("score cp {}", multipv_entry.score).to_string()
                };

                println!(
                    "{}",
                    &format!(
                        "info time {} {} depth {} seldepth {} multipv {} nodes {} hashfull {} tbhits {} pv {}",
                        depth_result.time,
                        formatted_score,
                        depth_result.depth,
                        depth_result.statistics.max_ply,
                        multipv_index + 1,
                        depth_result.statistics.nodes_count + depth_result.statistics.q_nodes_count,
                        (depth_result.transposition_table_usage * 10.0) as u32,
                        depth_result.statistics.tb_hits,
                        pv_line.join(" ").as_str()
                    )
                );
            }

            // Ignore result when no legal move was found, to prevent crash further
            if depth_result.lines[0].pv_line.is_empty() {
                continue;
            }

            best_move = depth_result.lines[0].pv_line[0];

            // Check if the ponder move is legal
            if ponder && depth_result.lines[0].pv_line.len() >= 2 {
                let mut board = state_arc.lock().unwrap().board.clone();
                let mut allow_ponder = true;

                board.make_move(depth_result.lines[0].pv_line[0]);
                board.make_move(depth_result.lines[0].pv_line[1]);

                if board.is_king_checked(board.active_color ^ 1) {
                    allow_ponder = false;
                }

                if board.is_repetition_draw(3) || board.is_fifty_move_rule_draw() || board.is_insufficient_material_draw() {
                    allow_ponder = false;
                }

                board.undo_move(depth_result.lines[0].pv_line[1]);
                board.undo_move(depth_result.lines[0].pv_line[0]);

                if allow_ponder {
                    ponder_move = depth_result.lines[0].pv_line[1];
                } else {
                    ponder_move = Default::default();
                }
            } else {
                ponder_move = Default::default();
            }
        }

        if ponder && ponder_move != Default::default() {
            println!("bestmove {} ponder {}", best_move, ponder_move);
        } else {
            println!("bestmove {}", best_move);
        }

        let state_lock = state_arc.lock().unwrap();
        state_lock.killers_table.age_moves();
        state_lock.history_table.age_values();
    });
}

/// Handles `isready` command by waiting for the busy flag, and then printing response as fast as possible.
fn handle_isready() {
    println!("readyok");
}

/// Handles `ponderhit` command by setting abort and ponder flags, which should switch a search mode from the ponder to the regular one.
fn handle_ponderhit(state: Arc<Mutex<UciState>>) {
    let state_lock = state.lock().unwrap();
    state_lock.ponder_flag.store(true, Ordering::Relaxed);
    state_lock.abort_flag.store(true, Ordering::Relaxed);
}

/// Handles `position ...` command with the following variants:
///  - `position startpos` - sets a default position
///  - `position startpos moves [list of moves]` - sets a default position and applies a list of moves
///  - `position fen [fen]` - sets a FEN position
///  - `position fen [fen] moves [list of moves]` - sets a FEN position and applies a list of moves
fn handle_position(parameters: &[String], state: Arc<Mutex<UciState>>) {
    if parameters.len() < 2 {
        return;
    }

    state.lock().unwrap().board = match parameters[1].as_str() {
        "fen" => {
            let fen = parameters[2..].join(" ");
            match Board::new_from_fen(fen.as_str(), None, None, None, None, None) {
                Ok(board) => board,
                Err(error) => {
                    println!("info string Error: {}", error);
                    return;
                }
            }
        }
        _ => Board::new_initial_position(None, None, None, None, None),
    };

    if let Some(index) = parameters.iter().position(|s| s == "moves") {
        for premade_move in &parameters[index + 1..] {
            let parsed_move = match Move::from_long_notation(premade_move, &state.lock().unwrap().board) {
                Ok(r#move) => r#move,
                Err(error) => {
                    println!("info string Error: {}", error);
                    return;
                }
            };

            state.lock().unwrap().board.make_move(parsed_move);
        }
    };
}

/// Handles `setoption [name] value [value]` command by creating or overwriting a `name` option with the specified `value`. Recreates tables if `Hash` or
/// `Clear Hash` options are modified.
fn handle_setoption(parameters: &[String], state: Arc<Mutex<UciState>>) {
    let mut reading_name = false;
    let mut reading_value = false;
    let mut name_tokens = Vec::new();
    let mut value_tokens = Vec::new();

    for parameter in parameters {
        match parameter.as_str() {
            "name" => {
                reading_name = true;
                reading_value = false;
            }
            "value" => {
                reading_name = false;
                reading_value = true;
            }
            _ => {
                if reading_name {
                    name_tokens.push(parameter.to_owned());
                } else if reading_value {
                    value_tokens.push(parameter.to_owned());
                }
            }
        }
    }

    let name = name_tokens.join(" ");
    let value = value_tokens.join(" ");

    if !name.is_empty() && !value.is_empty() {
        if let Some(option) = state.lock().unwrap().options.get_mut(&name) {
            option.value = value.to_string();
        } else {
            #[cfg(feature = "dev")]
            panic!("Invalid value: name={}, value={}", name, value);
        }
    }

    match name.as_str() {
        "Hash" => {
            recreate_state_tables(state);
        }
        "SyzygyPath" => {
            if !value.is_empty() && value != "<empty>" {
                syzygy::probe::init(&value);
            }

            #[cfg(not(feature = "syzygy"))]
            println!("info string Syzygy tablebases not supported in this build");
        }
        "Clear Hash" => {
            recreate_state_tables(state);
        }
        "Crash Files" => match value.parse::<bool>().unwrap() {
            true => enable_crash_files(),
            false => disable_crash_files(),
        },
        _ => {}
    }
}

/// Handles `ucinewgame` command by resetting a board state, recreating abort flag and clearing tables.
fn handle_ucinewgame(state: Arc<Mutex<UciState>>) {
    let mut state_lock = state.lock().unwrap();
    state_lock.abort_flag.store(true, Ordering::Relaxed);
    state_lock.board = Board::new_initial_position(None, None, None, None, None);
    state_lock.abort_flag = Default::default();
    drop(state_lock);

    recreate_state_tables(state.clone());
}

/// Handles `stop` command by setting abort flag, which should stop ongoing search as fast as possible.
fn handle_stop(state: Arc<Mutex<UciState>>) {
    state.lock().unwrap().abort_flag.store(true, Ordering::Relaxed);
}

/// Handles `quit` command by terminating engine process.
fn handle_quit() {
    process::exit(0);
}

/// Recreates transposition table, pawn hashtable, killers table and history table.
fn recreate_state_tables(state: Arc<Mutex<UciState>>) {
    let mut state_lock = state.lock().unwrap();
    let total_size = state_lock.options["Hash"].value.parse::<usize>().unwrap();
    let allocation_result = allocator::get_allocation(total_size);

    state_lock.transposition_table = Arc::new(TranspositionTable::new(allocation_result.transposition_table_size * 1024 * 1024));
    state_lock.pawn_hashtable = Arc::new(PawnHashTable::new(allocation_result.pawn_hashtable_size * 1024 * 1024));
    state_lock.killers_table = Default::default();
    state_lock.history_table = Default::default();
}

/// Enables saving of crash files by setting a custom panic hook.
fn enable_crash_files() {
    panic::set_hook(Box::new(|panic| {
        let path = Path::new("./crash");
        fs::create_dir_all(path).unwrap();

        let path = path.join(format!("{}.txt", common::time::get_unix_timestamp()));
        write!(&mut File::create(path.clone()).unwrap(), "{}", panic).unwrap();

        let absolute_path = fs::canonicalize(path).unwrap();
        println!("info string Crash file saved as {}", absolute_path.into_os_string().into_string().unwrap());
    }));
}

/// Disables saving of crash files by reverting a panic hook to the default one.
fn disable_crash_files() {
    let _ = panic::take_hook();
}
