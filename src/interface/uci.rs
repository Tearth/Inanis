use crate::cache::allocator;
use crate::cache::pawns::PawnHashTable;
use crate::cache::search::TranspositionTable;
use crate::engine;
use crate::engine::context::HelperThreadContext;
use crate::engine::context::SearchContext;
use crate::engine::history::HistoryTable;
use crate::engine::killers::KillersTable;
use crate::state::board::Bitboard;
use crate::state::movescan::Move;
use crate::state::*;
use chrono::Utc;
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
use std::thread::JoinHandle;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const AUTHOR: &str = env!("CARGO_PKG_AUTHORS");

struct UciState {
    board: Bitboard,
    options: HashMap<String, String>,
    transposition_table: Arc<TranspositionTable>,
    pawn_hashtable: Arc<PawnHashTable>,
    killers_table: Arc<KillersTable>,
    history_table: Arc<HistoryTable>,
    search_thread: Option<JoinHandle<()>>,
    abort_token: Arc<AtomicBool>,
    ponder_token: Arc<AtomicBool>,
    busy_flag: AtomicBool,
    debug_mode: AtomicBool,
}

impl Default for UciState {
    /// Constructs a default instance of [UciState] with zeroed elements and hashtables with their default sizes.
    fn default() -> Self {
        UciState {
            board: Bitboard::new_initial_position(None, None, None, None, None),
            options: HashMap::new(),
            transposition_table: Arc::new(TranspositionTable::new(1 * 1024 * 1024)),
            pawn_hashtable: Arc::new(PawnHashTable::new(1 * 1024 * 1024)),
            killers_table: Arc::new(Default::default()),
            history_table: Arc::new(Default::default()),
            search_thread: None,
            abort_token: Arc::new(AtomicBool::new(false)),
            ponder_token: Arc::new(AtomicBool::new(false)),
            busy_flag: AtomicBool::new(false),
            debug_mode: AtomicBool::new(false),
        }
    }
}

/// Entry point of the UCI (Universal Chess Interface) and command loop.
pub fn run() {
    let state: Arc<Mutex<UciState>> = Arc::new(Mutex::new(Default::default()));
    state.lock().unwrap().options.insert("Hash".to_string(), "2".to_string());
    state.lock().unwrap().options.insert("Move Overhead".to_string(), "10".to_string());
    state.lock().unwrap().options.insert("Threads".to_string(), "1".to_string());
    state.lock().unwrap().options.insert("Ponder".to_string(), "false".to_string());
    state.lock().unwrap().options.insert("Crash Files".to_string(), "false".to_string());

    println!("id name Inanis {}", VERSION);
    println!("id author {}", AUTHOR);
    println!("option name Hash type spin default 2 min 2 max 1048576");
    println!("option name Move Overhead type spin default 10 min 0 max 3600000");
    println!("option name Threads type spin default 1 min 1 max 1024");
    println!("option name Ponder type check default false");
    println!("option name Crash Files type check default false");
    println!("option name Clear Hash type button");
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
            "go" => handle_go(&tokens, state.clone()),
            "isready" => handle_isready(state.clone()),
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

    state
        .lock()
        .unwrap()
        .debug_mode
        .store(matches!(parameters[1].as_str(), "on"), Ordering::Relaxed);
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
///  - `infinite` - tells the search to run until it reaches the maximum depth for the engine
///  - `ponder` - tells the search to run in the ponder mode (thinking on the opponent's time)
fn handle_go(parameters: &[String], state: Arc<Mutex<UciState>>) {
    wait_for_busy_flag(state.clone());

    let mut white_time = u32::MAX;
    let mut black_time = u32::MAX;
    let mut white_inc_time = 0;
    let mut black_inc_time = 0;
    let mut forced_depth = 0;
    let mut max_nodes_count = 0;
    let mut max_move_time = 0;
    let mut moves_to_go = 0;
    let mut moves_to_search = Vec::new();

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
                let keywords = [
                    "wtime",
                    "btime",
                    "winc",
                    "binc",
                    "depth",
                    "nodes",
                    "movetime",
                    "movestogo",
                    "infinite",
                    "searchmoves",
                    "ponder",
                ];

                while let Some(value) = iter.peek() {
                    if keywords.contains(&value.as_str()) {
                        break;
                    }

                    let parsed_move = match Move::from_long_notation(value, &state.lock().unwrap().board) {
                        Ok(r#move) => r#move,
                        Err(message) => {
                            println!("info string Error: {}", message);
                            return;
                        }
                    };

                    moves_to_search.push(parsed_move);
                    iter.next();
                }
            }
            "ponder" => {
                forced_depth = engine::MAX_DEPTH;
            }
            _ => {}
        }
    }

    let mut time = match state.lock().unwrap().board.active_color {
        WHITE => white_time,
        BLACK => black_time,
        _ => panic!("Invalid value: state.board.active_color={}", state.lock().unwrap().board.active_color),
    };
    time -= cmp::min(time, state.lock().unwrap().options["Move Overhead"].parse::<u32>().unwrap());

    let inc_time = match state.lock().unwrap().board.active_color {
        WHITE => white_inc_time,
        BLACK => black_inc_time,
        _ => panic!("Invalid value: state.board.active_color={}", state.lock().unwrap().board.active_color),
    };

    let state_arc = state.clone();

    state.lock().unwrap().abort_token.store(false, Ordering::Relaxed);
    state.lock().unwrap().ponder_token.store(false, Ordering::Relaxed);
    state.lock().unwrap().busy_flag.store(true, Ordering::Relaxed);

    let search_thread = Some(thread::spawn(move || {
        let threads = state_arc.lock().unwrap().options["Threads"].parse::<usize>().unwrap();
        let ponder = state_arc.lock().unwrap().options["Ponder"].parse::<bool>().unwrap();

        let state_lock = state_arc.lock().unwrap();
        let mut context = SearchContext::new(
            state_lock.board.clone(),
            time,
            inc_time,
            forced_depth,
            max_nodes_count,
            max_move_time,
            moves_to_go,
            state_lock.debug_mode.load(Ordering::Relaxed),
            false,
            moves_to_search.clone(),
            state_lock.transposition_table.clone(),
            state_lock.pawn_hashtable.clone(),
            state_lock.killers_table.clone(),
            state_lock.history_table.clone(),
            state_lock.abort_token.clone(),
            state_lock.ponder_token.clone(),
        );
        drop(state_lock);

        if threads > 1 {
            for _ in 0..threads {
                let l = state_arc.lock().unwrap();
                let helper_context = SearchContext::new(
                    l.board.clone(),
                    time,
                    inc_time,
                    forced_depth,
                    max_nodes_count,
                    max_move_time,
                    moves_to_go,
                    l.debug_mode.load(Ordering::Relaxed),
                    true,
                    moves_to_search.clone(),
                    l.transposition_table.clone(),
                    l.pawn_hashtable.clone(),
                    l.killers_table.clone(),
                    l.history_table.clone(),
                    l.abort_token.clone(),
                    l.ponder_token.clone(),
                );
                drop(l);

                let data = HelperThreadContext {
                    board: context.board.clone(),
                    pawn_hashtable: Arc::new((*context.pawn_hashtable).clone()),
                    killers_table: Arc::new((*context.killers_table).clone()),
                    history_table: Arc::new((*context.history_table).clone()),
                    context: helper_context,
                };

                context.helper_contexts.push(data);
            }
        }

        let mut best_move = Default::default();
        let mut ponder_move = Default::default();

        for depth_result in context {
            let pv_line: Vec<String> = depth_result.pv_line.iter().map(|v| v.to_long_notation()).collect();
            let formatted_score = if engine::is_score_near_checkmate(depth_result.score) {
                let mut moves_to_mate = (depth_result.score.abs() - engine::CHECKMATE_SCORE).abs() / 2;
                moves_to_mate *= depth_result.score.signum();

                format!("score mate {}", moves_to_mate).to_string()
            } else {
                format!("score cp {}", depth_result.score).to_string()
            };

            best_move = depth_result.pv_line[0];
            println!(
                "{}",
                &format!(
                    "info time {} {} depth {} seldepth {} nodes {} pv {}",
                    depth_result.time,
                    formatted_score,
                    depth_result.depth,
                    depth_result.statistics.max_ply,
                    depth_result.statistics.nodes_count + depth_result.statistics.q_nodes_count,
                    pv_line.join(" ").as_str()
                )
            );

            // Check if the ponder move is legal
            if ponder && depth_result.pv_line.len() >= 2 {
                let mut board = state_arc.lock().unwrap().board.clone();
                let mut allow_ponder = true;

                board.make_move(depth_result.pv_line[0]);
                board.make_move(depth_result.pv_line[1]);

                if board.is_king_checked(board.active_color ^ 1) {
                    allow_ponder = false;
                }

                if board.is_repetition_draw(3) || board.is_fifty_move_rule_draw() || board.is_insufficient_material_draw() {
                    allow_ponder = false;
                }

                board.undo_move(depth_result.pv_line[1]);
                board.undo_move(depth_result.pv_line[0]);

                if allow_ponder {
                    ponder_move = depth_result.pv_line[1];
                } else {
                    ponder_move = Default::default();
                }
            } else {
                ponder_move = Default::default();
            }
        }

        if ponder && ponder_move != Default::default() {
            println!("bestmove {} ponder {}", best_move.to_long_notation(), ponder_move.to_long_notation());
        } else {
            println!("bestmove {}", best_move.to_long_notation());
        }

        state_arc.lock().unwrap().search_thread = None;
        state_arc.lock().unwrap().transposition_table.age_entries();
        state_arc.lock().unwrap().killers_table.age_moves();
        state_arc.lock().unwrap().history_table.age_values();
        state_arc.lock().unwrap().busy_flag.store(false, Ordering::Relaxed);
    }));

    state.lock().unwrap().search_thread = search_thread;
}

/// Handles `isready` command by waiting for the busy flag, and then printing response as fast as possible.
fn handle_isready(state: Arc<Mutex<UciState>>) {
    wait_for_busy_flag(state);
    println!("readyok");
}

/// Handles `ponderhit` command by setting abort and ponder tokens, which should switch a search mode from the ponder to the regular one.
fn handle_ponderhit(state: Arc<Mutex<UciState>>) {
    state.lock().unwrap().ponder_token.store(true, Ordering::Relaxed);
    state.lock().unwrap().abort_token.store(true, Ordering::Relaxed);
}

/// Handles `position ...` command with the following variants:
///  - `position startpos` - sets a default position
///  - `position startpos moves [list of moves]` - sets a default position and applies a list of moves
///  - `position fen [fen]` - sets a FEN position
///  - `position fen [fen] moves [list of moves]` - sets a FEN position and applies a list of moves
fn handle_position(parameters: &[String], state: Arc<Mutex<UciState>>) {
    wait_for_busy_flag(state.clone());

    if parameters.len() < 2 {
        return;
    }

    state.lock().unwrap().board = match parameters[1].as_str() {
        "fen" => {
            let fen = parameters[2..].join(" ");
            match Bitboard::new_from_fen(fen.as_str(), None, None, None, None, None) {
                Ok(board) => board,
                Err(message) => {
                    println!("info string Error: {}", message);
                    return;
                }
            }
        }
        _ => Bitboard::new_initial_position(None, None, None, None, None),
    };

    if let Some(index) = parameters.iter().position(|s| s == "moves") {
        for premade_move in &parameters[index + 1..] {
            let parsed_move = match Move::from_long_notation(premade_move, &state.lock().unwrap().board) {
                Ok(r#move) => r#move,
                Err(message) => {
                    println!("info string Error: {}", message);
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
    wait_for_busy_flag(state.clone());

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
        state.lock().unwrap().options.insert(name.to_owned(), value.to_owned());
    }

    match name.as_str() {
        "Hash" => {
            recreate_state_tables(state);
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

/// Handles `ucinewgame` command by resetting a board state, recreating abort token and clearing tables.
fn handle_ucinewgame(state: Arc<Mutex<UciState>>) {
    wait_for_busy_flag(state.clone());

    state.lock().unwrap().abort_token.store(true, Ordering::Relaxed);
    state.lock().unwrap().board = Bitboard::new_initial_position(None, None, None, None, None);
    state.lock().unwrap().abort_token = Default::default();

    recreate_state_tables(state);
}

/// Handles `stop` command by setting abort token, which should stop ongoing search as fast as possible.
fn handle_stop(state: Arc<Mutex<UciState>>) {
    state.lock().unwrap().abort_token.store(true, Ordering::Relaxed);
}

/// Handles `quit` command by terminating engine process.
fn handle_quit() {
    process::exit(0);
}

/// Waits for the busy flag before continuing. If the deadline is exceeded, the engine process is terminated.
fn wait_for_busy_flag(state: Arc<Mutex<UciState>>) {
    let now = Utc::now();
    while (*state).lock().unwrap().busy_flag.fetch_and(true, Ordering::Relaxed) {
        if (Utc::now() - now).num_seconds() >= 10 {
            process::exit(-1);
        }
    }
}

/// Recreates transposition table, pawn hashtable, killers table and history table.
fn recreate_state_tables(state: Arc<Mutex<UciState>>) {
    let total_size = state.lock().unwrap().options["Hash"].parse::<usize>().unwrap();
    let allocation_result = allocator::get_allocation(total_size);

    state.lock().unwrap().transposition_table = Arc::new(TranspositionTable::new(allocation_result.transposition_table_size * 1024 * 1024));
    state.lock().unwrap().pawn_hashtable = Arc::new(PawnHashTable::new(allocation_result.pawn_hashtable_size * 1024 * 1024));
    state.lock().unwrap().killers_table = Default::default();
    state.lock().unwrap().history_table = Default::default();
}

/// Enables saving of crash files by setting a custom panic hook.
fn enable_crash_files() {
    panic::set_hook(Box::new(|panic| {
        let path = Path::new("crash");
        fs::create_dir_all(path).unwrap();

        let path = Path::new("crash").join(format!("{}.txt", Utc::now().timestamp_millis()));
        write!(&mut File::create(path.clone()).unwrap(), "{}", panic).unwrap();

        let absolute_path = fs::canonicalize(path).unwrap();
        println!("Crash file saved as {}", absolute_path.into_os_string().into_string().unwrap());
    }));
}

/// Disables saving of crash files by reverting a panic hook to the default one.
fn disable_crash_files() {
    let _ = panic::take_hook();
}
