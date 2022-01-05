use crate::cache::pawns::PawnHashTable;
use crate::cache::search::TranspositionTable;
use crate::engine::context::AbortToken;
use crate::engine::context::SearchContext;
use crate::engine::history::HistoryTable;
use crate::engine::killers::KillersTable;
use crate::engine::*;
use crate::state::board::Bitboard;
use crate::state::movescan::Move;
use crate::state::*;
use std::cell::UnsafeCell;
use std::collections::HashMap;
use std::io;
use std::process;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::thread;
use std::thread::JoinHandle;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const AUTHOR: &str = env!("CARGO_PKG_AUTHORS");

struct UciState {
    board: UnsafeCell<Bitboard>,
    options: UnsafeCell<HashMap<String, String>>,
    transposition_table: UnsafeCell<TranspositionTable>,
    pawn_hashtable: UnsafeCell<PawnHashTable>,
    killers_table: UnsafeCell<KillersTable>,
    history_table: UnsafeCell<HistoryTable>,
    search_thread: UnsafeCell<Option<JoinHandle<()>>>,
    abort_token: UnsafeCell<AbortToken>,
    busy_flag: AtomicBool,
    debug_mode: AtomicBool,
}

impl Default for UciState {
    fn default() -> Self {
        UciState {
            board: UnsafeCell::new(Bitboard::new_initial_position()),
            options: UnsafeCell::new(HashMap::new()),
            transposition_table: UnsafeCell::new(TranspositionTable::new(1 * 1024 * 1024)),
            pawn_hashtable: UnsafeCell::new(PawnHashTable::new(1 * 1024 * 1024)),
            history_table: UnsafeCell::new(Default::default()),
            killers_table: UnsafeCell::new(Default::default()),
            search_thread: UnsafeCell::new(None),
            abort_token: UnsafeCell::new(Default::default()),
            busy_flag: AtomicBool::new(false),
            debug_mode: AtomicBool::new(false),
        }
    }
}

unsafe impl Sync for UciState {}

pub fn run() {
    let mut state: Arc<UciState> = Arc::new(Default::default());
    unsafe { (*state.options.get()).insert("Hash".to_string(), "1".to_string()) };

    println!("id name Inanis {}", VERSION);
    println!("id author {}", AUTHOR);
    println!("option name Hash type spin default 1 min 1 max 32768");
    println!("option name Clear Hash type button");
    println!("uciok");

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let tokens: Vec<String> = input.split(' ').map(|v| v.trim().to_string()).collect();
        match tokens[0].to_lowercase().as_str() {
            "debug" => handle_debug(&tokens, &mut state),
            "go" => handle_go(&tokens, &mut state),
            "isready" => handle_isready(&mut state),
            "position" => handle_position(&tokens, &mut state),
            "setoption" => handle_setoption(&tokens, &mut state),
            "ucinewgame" => handle_ucinewgame(&mut state),
            "stop" => handle_stop(&mut state),
            "quit" => handle_quit(),
            _ => {}
        }
    }
}

fn handle_debug(parameters: &[String], state: &mut Arc<UciState>) {
    if parameters.len() < 2 {
        return;
    }

    (*state).debug_mode.store(matches!(parameters[1].as_str(), "on"), Ordering::Relaxed);
}

fn handle_go(parameters: &[String], state: &mut Arc<UciState>) {
    wait_for_busy_flag(state);
    unsafe {
        let mut white_time = u32::MAX;
        let mut black_time = u32::MAX;
        let mut white_inc_time = 0;
        let mut black_inc_time = 0;
        let mut forced_depth = 0;
        let mut max_nodes_count = 0;
        let mut max_move_time = 0;
        let mut moves_to_go = 0;

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
                    forced_depth = MAX_DEPTH;
                }
                _ => {}
            }
        }

        let time = match (*state.board.get()).active_color {
            WHITE => white_time,
            BLACK => black_time,
            _ => panic!("Invalid value: state.board.active_color={}", (*state.board.get()).active_color),
        };

        let inc_time = match (*state.board.get()).active_color {
            WHITE => white_inc_time,
            BLACK => black_inc_time,
            _ => panic!("Invalid value: state.board.active_color={}", (*state.board.get()).active_color),
        };

        let state_arc = state.clone();

        (*state.abort_token.get()).aborted = false;
        (*state).busy_flag.store(true, Ordering::Relaxed);

        *state.search_thread.get() = Some(thread::spawn(move || {
            let context = SearchContext::new(
                &mut *state_arc.board.get(),
                time,
                inc_time,
                forced_depth,
                max_nodes_count,
                max_move_time,
                moves_to_go,
                state_arc.debug_mode.load(Ordering::Relaxed),
                &mut *state_arc.transposition_table.get(),
                &mut *state_arc.pawn_hashtable.get(),
                &mut *state_arc.killers_table.get(),
                &mut *state_arc.history_table.get(),
                &mut *state_arc.abort_token.get(),
            );

            let mut best_move = Default::default();

            for depth_result in context {
                let pv_line: Vec<String> = depth_result.pv_line.iter().map(|v| v.to_long_notation()).collect();
                let formatted_score = if is_score_near_checkmate(depth_result.score) {
                    let mut moves_to_mate = (depth_result.score.abs() - CHECKMATE_SCORE).abs() / 2;
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
            }

            println!("bestmove {}", best_move.to_long_notation());

            (*state_arc.search_thread.get()) = None;
            (*state_arc.transposition_table.get()).age_entries();
            (*state_arc.killers_table.get()).age_moves();
            (*state_arc.history_table.get()).age_values();
            (*state_arc).busy_flag.store(false, Ordering::Relaxed);
        }));
    }
}

fn handle_isready(state: &mut Arc<UciState>) {
    wait_for_busy_flag(state);
    println!("readyok");
}

fn handle_position(parameters: &[String], state: &mut Arc<UciState>) {
    wait_for_busy_flag(state);

    if parameters.len() < 2 {
        return;
    }

    unsafe {
        while (*state).busy_flag.fetch_and(true, Ordering::Release) {}
        *state.board.get() = match parameters[1].as_str() {
            "fen" => {
                let fen = parameters[2..].join(" ");
                match Bitboard::new_from_fen(fen.as_str()) {
                    Ok(board) => board,
                    Err(message) => {
                        println!("info string Error: {}", message);
                        return;
                    }
                }
            }
            _ => Bitboard::new_initial_position(),
        };
    }

    if let Some(index) = parameters.iter().position(|s| s == "moves") {
        for premade_move in &parameters[index + 1..] {
            let parsed_move = match Move::from_long_notation(premade_move, unsafe { &mut *state.board.get() }) {
                Ok(r#move) => r#move,
                Err(message) => {
                    println!("info string Error: {}", message);
                    return;
                }
            };
            unsafe { (*state.board.get()).make_move(&parsed_move) };
        }
    };
}

fn handle_setoption(parameters: &[String], state: &mut Arc<UciState>) {
    wait_for_busy_flag(state);

    if parameters.len() == 4 {
        if parameters[2] == "Clear" && parameters[3] == "Hash" {
            unsafe {
                let transposition_table_size = (*state.options.get())["Hash"].parse::<usize>().unwrap() * 1024 * 1024;
                *state.transposition_table.get() = TranspositionTable::new(transposition_table_size);
                *state.pawn_hashtable.get() = PawnHashTable::new(1 * 1024 * 1024);
                *state.killers_table.get() = Default::default();
                *state.history_table.get() = Default::default();
            }
        }
    } else if parameters.len() == 5 {
        unsafe { (*state.options.get()).insert(parameters[2].to_string(), parameters[4].to_string()) };

        if parameters[2] == "Hash" {
            unsafe {
                let transposition_table_size = parameters[4].parse::<usize>().unwrap() * 1024 * 1024;
                *state.transposition_table.get() = TranspositionTable::new(transposition_table_size);
            }
        }
    }
}

fn handle_ucinewgame(state: &mut Arc<UciState>) {
    unsafe {
        (*state.abort_token.get()).aborted = true;
        wait_for_busy_flag(state);

        let transposition_table_size = (*state.options.get())["Hash"].parse::<usize>().unwrap() * 1024 * 1024;
        *state.board.get() = Bitboard::new_initial_position();
        *state.transposition_table.get() = TranspositionTable::new(transposition_table_size);
        *state.pawn_hashtable.get() = PawnHashTable::new(1 * 1024 * 1024);
        *state.killers_table.get() = Default::default();
        *state.history_table.get() = Default::default();
        *state.abort_token.get() = Default::default();
    }
}

fn handle_stop(state: &mut Arc<UciState>) {
    unsafe {
        (*state.abort_token.get()).aborted = true;
    }
}

fn handle_quit() {
    process::exit(0);
}

fn wait_for_busy_flag(state: &mut Arc<UciState>) {
    while (*state).busy_flag.fetch_and(true, Ordering::Release) {}
}
