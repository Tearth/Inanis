use crate::cache::counters::CMTable;
use crate::cache::history::HTable;
use crate::cache::killers::KTable;
use crate::cache::pawns::PHTable;
use crate::cache::search::TTable;
use crate::engine;
use crate::engine::context::SearchContext;
use crate::engine::params::SearchParams;
use crate::perft;
use crate::state::movescan::Move;
use crate::state::representation::Board;
use crate::state::*;
use crate::stats::SearchStats;
use crate::tablebases::syzygy;
use crate::utils::minmax::MinMax;
use crate::utils::panic_fast;
use std::cmp;
use std::collections::HashMap;
use std::io;
use std::panic;
use std::process;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::sync::RwLock;
use std::thread;
use std::time::SystemTime;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const AUTHOR: &str = env!("CARGO_PKG_AUTHORS");
const PAWN_HASHTABLE_SIZE: usize = 1 * 1024 * 1024;

pub struct UciState {
    context: Arc<RwLock<SearchContext>>,
    options: Arc<RwLock<HashMap<String, UciOption>>>,
    abort_flag: Arc<AtomicBool>,
    ponder_flag: Arc<AtomicBool>,
    debug_mode: bool,
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
    /// Constructs a new instance of [UciOption] with `order`, `r#type`, `min`, `max` and `default`.
    pub fn new<T>(order: u32, r#type: &str, min: T, max: T, default: T) -> Self
    where
        T: ToString,
    {
        Self { order, r#type: r#type.to_string(), min: min.to_string(), max: max.to_string(), default: default.to_string(), value: default.to_string() }
    }

    /// Constructs a new instance of [UciOption] with `order` and `default`, where `min` and `max` are set to make the biggest range possible.
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
    /// Constructs a default instance of [UciState] with zeroed elements and hashtables with their default sizes (1 MB).
    fn default() -> Self {
        let abort_flag = Arc::new(AtomicBool::new(false));
        let ponder_flag = Arc::new(AtomicBool::new(false));

        UciState {
            context: Arc::new(RwLock::new(SearchContext::new(
                Board::new_initial_position(),
                Arc::new(TTable::new(1 * 1024 * 1024)),
                Arc::new(PHTable::new(1 * 1024 * 1024)),
                abort_flag.clone(),
                ponder_flag.clone(),
            ))),
            options: Arc::new(RwLock::new(HashMap::new())),
            abort_flag,
            ponder_flag,
            debug_mode: false,
        }
    }
}

/// Entry point of the UCI (Universal Chess Interface) and command loop.
pub fn run() {
    let mut state = UciState::default();
    let options_arc = state.options.clone();
    let mut options_lock = options_arc.write().unwrap();

    println!("id name Inanis {}", VERSION);
    println!("id author {}", AUTHOR);

    options_lock.insert("Hash".to_string(), UciOption::new(0, "spin", 1, 1048576, 2));
    options_lock.insert("Move Overhead".to_string(), UciOption::new(1, "spin", 0, 3600000, 10));
    options_lock.insert("MultiPV".to_string(), UciOption::new(2, "spin", 1, 256, 1));
    options_lock.insert("Threads".to_string(), UciOption::new(3, "spin", 1, 1024, 1));
    options_lock.insert("SyzygyPath".to_string(), UciOption::new(4, "string", "", "", "<empty>"));
    options_lock.insert("SyzygyProbeLimit".to_string(), UciOption::new(5, "spin", 1, 9, 8));
    options_lock.insert("SyzygyProbeDepth".to_string(), UciOption::new(6, "spin", 1, 32, 6));
    options_lock.insert("Ponder".to_string(), UciOption::new(7, "check", false, false, false));
    options_lock.insert("Clear Hash".to_string(), UciOption::new(8, "button", "", "", ""));

    #[cfg(feature = "dev")]
    options_lock.insert("Crash Files".to_string(), UciOption::new(50, "check", false, false, false));

    #[cfg(feature = "dev")]
    {
        let params = SearchParams::default();
        options_lock.insert("aspwin_delta".to_string(), UciOption::new_wide(99, params.aspwin_delta));
        options_lock.insert("aspwin_min_depth".to_string(), UciOption::new_wide(99, params.aspwin_min_depth));
        options_lock.insert("aspwin_max_width".to_string(), UciOption::new_wide(99, params.aspwin_max_width));

        options_lock.insert("iir_min_depth".to_string(), UciOption::new_wide(99, params.iir_min_depth));
        options_lock.insert("iir_reduction_base".to_string(), UciOption::new_wide(99, params.iir_reduction_base));
        options_lock.insert("iir_reduction_step".to_string(), UciOption::new_wide(99, params.iir_reduction_step));
        options_lock.insert("iir_max_reduction".to_string(), UciOption::new_wide(99, params.iir_max_reduction));

        options_lock.insert("razoring_min_depth".to_string(), UciOption::new_wide(99, params.razoring_min_depth));
        options_lock.insert("razoring_max_depth".to_string(), UciOption::new_wide(99, params.razoring_max_depth));
        options_lock.insert("razoring_depth_margin_base".to_string(), UciOption::new_wide(99, params.razoring_depth_margin_base));
        options_lock.insert("razoring_depth_margin_multiplier".to_string(), UciOption::new_wide(99, params.razoring_depth_margin_multiplier));

        options_lock.insert("snmp_min_depth".to_string(), UciOption::new_wide(99, params.snmp_min_depth));
        options_lock.insert("snmp_max_depth".to_string(), UciOption::new_wide(99, params.snmp_max_depth));
        options_lock.insert("snmp_depth_margin_base".to_string(), UciOption::new_wide(99, params.snmp_depth_margin_base));
        options_lock.insert("snmp_depth_margin_multiplier".to_string(), UciOption::new_wide(99, params.snmp_depth_margin_multiplier));

        options_lock.insert("nmp_min_depth".to_string(), UciOption::new_wide(99, params.nmp_min_depth));
        options_lock.insert("nmp_min_game_phase".to_string(), UciOption::new_wide(99, params.nmp_min_game_phase));
        options_lock.insert("nmp_margin".to_string(), UciOption::new_wide(99, params.nmp_margin));
        options_lock.insert("nmp_depth_base".to_string(), UciOption::new_wide(99, params.nmp_depth_base));
        options_lock.insert("nmp_depth_divider".to_string(), UciOption::new_wide(99, params.nmp_depth_divider));

        options_lock.insert("lmp_min_depth".to_string(), UciOption::new_wide(99, params.lmp_min_depth));
        options_lock.insert("lmp_max_depth".to_string(), UciOption::new_wide(99, params.lmp_max_depth));
        options_lock.insert("lmp_move_index_margin_base".to_string(), UciOption::new_wide(99, params.lmp_move_index_margin_base));
        options_lock.insert("lmp_move_index_margin_multiplier".to_string(), UciOption::new_wide(99, params.lmp_move_index_margin_multiplier));
        options_lock.insert("lmp_max_score".to_string(), UciOption::new_wide(99, params.lmp_max_score));

        options_lock.insert("lmr_min_depth".to_string(), UciOption::new_wide(99, params.lmr_min_depth));
        options_lock.insert("lmr_max_score".to_string(), UciOption::new_wide(99, params.lmr_max_score));
        options_lock.insert("lmr_min_move_index".to_string(), UciOption::new_wide(99, params.lmr_min_move_index));
        options_lock.insert("lmr_reduction_base".to_string(), UciOption::new_wide(99, params.lmr_reduction_base));
        options_lock.insert("lmr_reduction_step".to_string(), UciOption::new_wide(99, params.lmr_reduction_step));
        options_lock.insert("lmr_max_reduction".to_string(), UciOption::new_wide(99, params.lmr_max_reduction));
        options_lock.insert("lmr_pv_min_move_index".to_string(), UciOption::new_wide(99, params.lmr_pv_min_move_index));
        options_lock.insert("lmr_pv_reduction_base".to_string(), UciOption::new_wide(99, params.lmr_pv_reduction_base));
        options_lock.insert("lmr_pv_reduction_step".to_string(), UciOption::new_wide(99, params.lmr_pv_reduction_step));
        options_lock.insert("lmr_pv_max_reduction".to_string(), UciOption::new_wide(99, params.lmr_pv_max_reduction));

        options_lock.insert("q_score_pruning_treshold".to_string(), UciOption::new_wide(99, params.q_score_pruning_treshold));
        options_lock.insert("q_futility_pruning_margin".to_string(), UciOption::new_wide(99, params.q_futility_pruning_margin));
    }

    let mut options_sorted = options_lock.iter().collect::<Vec<_>>();
    options_sorted.sort_by_key(|(_, option)| option.order);

    for (name, option) in options_sorted {
        match option.r#type.as_str() {
            "spin" => println!("option name {} type {} default {} min {} max {}", name, option.r#type, option.default, option.min, option.max),
            "string" => println!("option name {} type {} default {}", name, option.r#type, option.default),
            "check" => println!("option name {} type {} default {}", name, option.r#type, option.default),
            "button" => println!("option name {} type {}", name, option.r#type),
            _ => panic_fast!("Invalid value: option.r#type={}", option.r#type),
        };
    }
    drop(options_lock);

    println!("uciok");

    loop {
        let mut input = String::new();
        let read_bytes = io::stdin().read_line(&mut input).unwrap();

        if read_bytes == 0 {
            process::exit(0);
        }

        let tokens: Vec<String> = input.split(' ').map(|v| v.trim().to_string()).collect();
        match tokens[0].to_lowercase().as_str() {
            "debug" => handle_debug(&tokens, &mut state),
            "fen" => handle_fen(&state),
            "go" => handle_go(&tokens, &state),
            "isready" => handle_isready(),
            "ponderhit" => handle_ponderhit(&state),
            "position" => handle_position(&tokens, &state),
            "setoption" => handle_setoption(&tokens, &mut state),
            "ucinewgame" => handle_ucinewgame(&mut state),
            "stop" => handle_stop(&state),
            "quit" => handle_quit(),
            _ => {}
        }
    }
}

/// Handles `debug [on/off]` command by setting the proper flag.
fn handle_debug(params: &[String], state: &mut UciState) {
    if params.len() < 2 {
        return;
    }

    state.debug_mode = params[1].as_str() == "on";
}

/// Handles non-standard `fen` command by printing FEN of the current position.
fn handle_fen(state: &UciState) {
    println!("info string {}", state.context.read().unwrap().board);
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
fn handle_go(params: &[String], state: &UciState) {
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

    let mut context_lock = state.context.write().unwrap();
    let options_lock = state.options.read().unwrap();
    let mut iter = params[1..].iter().peekable();

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

                    let parsed_move = match Move::from_long_notation(value, &context_lock.board) {
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
            let result = perft::normal::run(depth, &mut context_lock.board, false);
            let time = now.elapsed().unwrap().as_millis();

            println!("info time {} depth {} nodes {}", time, depth, result.nodes);
        }

        return;
    }

    let mut time = match context_lock.board.stm {
        WHITE => white_time,
        BLACK => black_time,
        _ => panic_fast!("Invalid value: context_lock.board.stm={}", context_lock.board.stm),
    };
    time -= cmp::min(time, options_lock["Move Overhead"].value.parse::<u32>().unwrap());

    let inc_time = match context_lock.board.stm {
        WHITE => white_inc_time,
        BLACK => black_inc_time,
        _ => panic_fast!("Invalid value: context_lock.board.stm={}", context_lock.board.stm),
    };

    state.abort_flag.store(false, Ordering::Relaxed);
    state.ponder_flag.store(false, Ordering::Relaxed);

    drop(options_lock);
    drop(context_lock);

    let context_arc = state.context.clone();
    let options_arc = state.options.clone();
    let debug_mode = state.debug_mode;

    thread::spawn(move || {
        let mut context_lock = context_arc.write().unwrap();
        let options_lock = options_arc.read().unwrap();

        let multipv = options_lock["MultiPV"].value.parse::<u32>().unwrap();
        let threads = options_lock["Threads"].value.parse::<usize>().unwrap();
        let ponder = options_lock["Ponder"].value.parse::<bool>().unwrap();
        let syzygy_path = options_lock["SyzygyPath"].value.clone();
        let syzygy_enabled = !syzygy_path.is_empty() && syzygy_path != "<empty>";
        let syzygy_probe_limit = options_lock["SyzygyProbeLimit"].value.parse::<u32>().unwrap();
        let syzygy_probe_depth = options_lock["SyzygyProbeDepth"].value.parse::<i8>().unwrap();

        #[cfg(not(feature = "dev"))]
        let search_params = SearchParams::default();

        #[cfg(feature = "dev")]
        let search_params = SearchParams {
            aspwin_delta: options_lock["aspwin_delta"].value.parse().unwrap(),
            aspwin_min_depth: options_lock["aspwin_min_depth"].value.parse().unwrap(),
            aspwin_max_width: options_lock["aspwin_max_width"].value.parse().unwrap(),

            iir_min_depth: options_lock["iir_min_depth"].value.parse().unwrap(),
            iir_reduction_base: options_lock["iir_reduction_base"].value.parse().unwrap(),
            iir_reduction_step: options_lock["iir_reduction_step"].value.parse().unwrap(),
            iir_max_reduction: options_lock["iir_max_reduction"].value.parse().unwrap(),

            razoring_min_depth: options_lock["razoring_min_depth"].value.parse().unwrap(),
            razoring_max_depth: options_lock["razoring_max_depth"].value.parse().unwrap(),
            razoring_depth_margin_base: options_lock["razoring_depth_margin_base"].value.parse().unwrap(),
            razoring_depth_margin_multiplier: options_lock["razoring_depth_margin_multiplier"].value.parse().unwrap(),

            snmp_min_depth: options_lock["snmp_min_depth"].value.parse().unwrap(),
            snmp_max_depth: options_lock["snmp_max_depth"].value.parse().unwrap(),
            snmp_depth_margin_base: options_lock["snmp_depth_margin_base"].value.parse().unwrap(),
            snmp_depth_margin_multiplier: options_lock["snmp_depth_margin_multiplier"].value.parse().unwrap(),

            nmp_min_depth: options_lock["nmp_min_depth"].value.parse().unwrap(),
            nmp_min_game_phase: options_lock["nmp_min_game_phase"].value.parse().unwrap(),
            nmp_margin: options_lock["nmp_margin"].value.parse().unwrap(),
            nmp_depth_base: options_lock["nmp_depth_base"].value.parse().unwrap(),
            nmp_depth_divider: options_lock["nmp_depth_divider"].value.parse().unwrap(),

            lmp_min_depth: options_lock["lmp_min_depth"].value.parse().unwrap(),
            lmp_max_depth: options_lock["lmp_max_depth"].value.parse().unwrap(),
            lmp_move_index_margin_base: options_lock["lmp_move_index_margin_base"].value.parse().unwrap(),
            lmp_move_index_margin_multiplier: options_lock["lmp_move_index_margin_multiplier"].value.parse().unwrap(),
            lmp_max_score: options_lock["lmp_max_score"].value.parse().unwrap(),

            lmr_min_depth: options_lock["lmr_min_depth"].value.parse().unwrap(),
            lmr_max_score: options_lock["lmr_max_score"].value.parse().unwrap(),
            lmr_min_move_index: options_lock["lmr_min_move_index"].value.parse().unwrap(),
            lmr_reduction_base: options_lock["lmr_reduction_base"].value.parse().unwrap(),
            lmr_reduction_step: options_lock["lmr_reduction_step"].value.parse().unwrap(),
            lmr_max_reduction: options_lock["lmr_max_reduction"].value.parse().unwrap(),
            lmr_pv_min_move_index: options_lock["lmr_pv_min_move_index"].value.parse().unwrap(),
            lmr_pv_reduction_base: options_lock["lmr_pv_reduction_base"].value.parse().unwrap(),
            lmr_pv_reduction_step: options_lock["lmr_pv_reduction_step"].value.parse().unwrap(),
            lmr_pv_max_reduction: options_lock["lmr_pv_max_reduction"].value.parse().unwrap(),

            q_score_pruning_treshold: options_lock["q_score_pruning_treshold"].value.parse().unwrap(),
            q_futility_pruning_margin: options_lock["q_futility_pruning_margin"].value.parse().unwrap(),
        };

        context_lock.params = search_params.clone();
        context_lock.search_id = context_lock.board.state_stack.len() as u8;
        context_lock.time = time;
        context_lock.inc_time = inc_time;
        context_lock.current_depth = 1;
        context_lock.forced_depth = forced_depth;
        context_lock.max_nodes_count = max_nodes_count;
        context_lock.max_move_time = max_move_time;
        context_lock.moves_to_go = moves_to_go;
        context_lock.moves_to_search = moves_to_search.clone();
        context_lock.search_time_start = SystemTime::now();
        context_lock.multipv = multipv > 1;
        context_lock.search_done = false;
        context_lock.uci_debug = debug_mode;
        context_lock.ponder_mode = ponder_mode;
        context_lock.syzygy_enabled = syzygy_enabled;
        context_lock.syzygy_probe_limit = syzygy_probe_limit;
        context_lock.syzygy_probe_depth = syzygy_probe_depth;
        context_lock.stats = SearchStats::default();

        context_lock.lines.clear();
        context_lock.helper_contexts.write().unwrap().clear();

        for _ in 0..threads - 1 {
            let mut helper_context = SearchContext::new(
                context_lock.board.clone(),
                context_lock.ttable.clone(),
                context_lock.phtable.clone(),
                context_lock.abort_flag.clone(),
                context_lock.ponder_flag.clone(),
            );
            helper_context.params = search_params.clone();
            context_lock.helper_contexts.write().unwrap().push(helper_context);
        }

        let mut best_move = Move::default();
        let mut ponder_move = Move::default();

        while let Some(depth_result) = context_lock.next() {
            for (line_index, line) in context_lock.lines.iter().take(multipv as usize).enumerate() {
                let pv_line: Vec<String> = line.pv_line.iter().map(|v| v.to_long_notation()).collect();
                let formatted_score = if engine::is_score_near_checkmate(line.score) {
                    let mut moves_to_mate = (line.score.abs() - engine::CHECKMATE_SCORE).abs() / 2;
                    moves_to_mate *= line.score.signum();

                    format!("score mate {}", moves_to_mate).to_string()
                } else {
                    format!("score cp {}", line.score).to_string()
                };

                println!(
                    "info time {} {} depth {} seldepth {} multipv {} nodes {} hashfull {} tbhits {} pv {}",
                    depth_result.time,
                    formatted_score,
                    depth_result.depth,
                    context_lock.stats.max_ply,
                    line_index + 1,
                    context_lock.stats.nodes_count + context_lock.stats.q_nodes_count,
                    (context_lock.ttable.get_usage(1000) * 10.0) as u32,
                    context_lock.stats.tb_hits,
                    pv_line.join(" ").as_str()
                );
            }

            // Ignore result when no legal move was found, to prevent crash further
            if context_lock.lines[0].pv_line.is_empty() {
                continue;
            }

            best_move = context_lock.lines[0].pv_line[0];

            // Check if the ponder move is legal
            if ponder && context_lock.lines[0].pv_line.len() >= 2 {
                let mut board = context_lock.board.clone();
                let mut allow_ponder = true;

                board.make_move(context_lock.lines[0].pv_line[0]);
                board.make_move(context_lock.lines[0].pv_line[1]);

                if board.is_king_checked(board.stm ^ 1) {
                    allow_ponder = false;
                }

                if board.is_repetition_draw(3) || board.is_fifty_move_rule_draw() || board.is_insufficient_material_draw() {
                    allow_ponder = false;
                }

                board.undo_move(context_lock.lines[0].pv_line[1]);
                board.undo_move(context_lock.lines[0].pv_line[0]);

                if allow_ponder {
                    ponder_move = context_lock.lines[0].pv_line[1];
                } else {
                    ponder_move = Move::default();
                }
            } else {
                ponder_move = Move::default();
            }
        }

        if ponder && ponder_move.is_some() {
            println!("bestmove {} ponder {}", best_move, ponder_move);
        } else {
            println!("bestmove {}", best_move);
        }

        context_lock.ktable.age_moves();
        context_lock.htable.age_values();
    });
}

/// Handles `isready` command by printing response as fast as possible.
fn handle_isready() {
    println!("readyok");
}

/// Handles `ponderhit` command by setting abort and ponder flags, which should switch a search mode from the ponder to the regular one.
fn handle_ponderhit(state: &UciState) {
    state.ponder_flag.store(true, Ordering::Relaxed);
    state.abort_flag.store(true, Ordering::Relaxed);
}

/// Handles `position ...` command with the following variants:
///  - `position startpos` - sets a default position
///  - `position startpos moves [list of moves]` - sets a default position and applies a list of moves
///  - `position fen [fen]` - sets a FEN position
///  - `position fen [fen] moves [list of moves]` - sets a FEN position and applies a list of moves
fn handle_position(params: &[String], state: &UciState) {
    if params.len() < 2 {
        return;
    }

    let mut context_lock = state.context.write().unwrap();
    context_lock.board = match params[1].as_str() {
        "fen" => {
            let fen = params[2..].join(" ");
            match Board::new_from_fen(fen.as_str()) {
                Ok(board) => board,
                Err(error) => {
                    println!("info string Error: {}", error);
                    return;
                }
            }
        }
        _ => Board::new_initial_position(),
    };

    if let Some(index) = params.iter().position(|s| s == "moves") {
        for premade_move in &params[index + 1..] {
            let parsed_move = match Move::from_long_notation(premade_move, &context_lock.board) {
                Ok(r#move) => r#move,
                Err(error) => {
                    println!("info string Error: {}", error);
                    return;
                }
            };

            context_lock.board.make_move(parsed_move);
        }
    };
}

/// Handles `setoption [name] value [value]` command by creating or overwriting a `name` option with the specified `value`.
/// Recreates tables if `Hash` or `Clear Hash` options are modified.
fn handle_setoption(params: &[String], state: &mut UciState) {
    let options_arc = state.options.clone();
    let mut options_lock = options_arc.write().unwrap();

    let mut reading_name = false;
    let mut reading_value = false;
    let mut name_tokens = Vec::new();
    let mut value_tokens = Vec::new();

    for parameter in params {
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
        if let Some(option) = options_lock.get_mut(&name) {
            option.value = value.to_string();
        } else {
            #[cfg(feature = "dev")]
            panic_fast!("Invalid value: name={}, value={}", name, value);
        }
    }

    drop(options_lock);

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
        #[cfg(feature = "dev")]
        "Crash Files" => match value.parse::<bool>().unwrap() {
            true => enable_crash_files(),
            false => disable_crash_files(),
        },
        _ => {}
    }
}

/// Handles `ucinewgame` command by resetting a board state, recreating abort flag and clearing tables.
fn handle_ucinewgame(state: &mut UciState) {
    let mut context_lock = state.context.write().unwrap();

    state.abort_flag.store(true, Ordering::Relaxed);
    context_lock.board = Board::new_initial_position();
    drop(context_lock);

    recreate_state_tables(state);
}

/// Handles `stop` command by setting abort flag, which should stop ongoing search as fast as possible.
fn handle_stop(state: &UciState) {
    state.abort_flag.store(true, Ordering::Relaxed);
}

/// Handles `quit` command by terminating engine process.
fn handle_quit() {
    process::exit(0);
}

/// Recreates transposition table, pawn hashtable, killers table and history table.
fn recreate_state_tables(state: &mut UciState) {
    let mut context_lock = state.context.write().unwrap();
    let options_lock = state.options.read().unwrap();
    let ttable_size = options_lock["Hash"].value.parse::<usize>().unwrap();

    context_lock.ttable = Arc::new(TTable::new(ttable_size * 1024 * 1024));
    context_lock.phtable = Arc::new(PHTable::new(PAWN_HASHTABLE_SIZE));
    context_lock.ktable = KTable::default();
    context_lock.htable = HTable::default();
    context_lock.cmtable = CMTable::default();
}

/// Enables saving of crash files by setting a custom panic hook.
#[cfg(feature = "dev")]
fn enable_crash_files() {
    use std::fs;
    use std::fs::File;
    use std::io::Write;
    use std::path::Path;

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
#[cfg(feature = "dev")]
fn disable_crash_files() {
    let _ = panic::take_hook();
}
