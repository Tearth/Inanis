use crate::cache::pawns::PawnHashTable;
use crate::cache::search::TranspositionTable;
use crate::engine::context::SearchContext;
use crate::engine::*;
use crate::state::board::Bitboard;
use crate::state::movescan::Move;
use crate::state::*;
use std::collections::HashMap;
use std::io;
use std::ops::Add;
use std::process;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const AUTHOR: &str = env!("CARGO_PKG_AUTHORS");
const DATE: &str = env!("DATE");

struct UciState {
    board: Bitboard,
    options: HashMap<String, String>,
    transposition_table: TranspositionTable,
    pawn_hash_table: PawnHashTable,
}

impl Default for UciState {
    fn default() -> Self {
        UciState {
            board: Bitboard::new_initial_position(),
            options: HashMap::new(),
            transposition_table: TranspositionTable::new(1 * 1024 * 1024),
            pawn_hash_table: PawnHashTable::new(1 * 1024 * 1024),
        }
    }
}

pub fn run() {
    let mut state: UciState = Default::default();
    state.options.insert("Hash".to_string(), "1".to_string());

    println!("id name Ina v{} ({})", VERSION, DATE);
    println!("id author {}", AUTHOR);
    println!("option name Hash type spin default 1 min 1 max 128");
    println!("uciok");

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let tokens: Vec<String> = input.split(' ').map(|p| p.trim().to_string()).collect();
        match tokens[0].to_lowercase().as_str() {
            "go" => handle_go(&tokens, &mut state),
            "isready" => handle_isready(),
            "position" => handle_position(&tokens, &mut state),
            "setoption" => handle_setoption(&tokens, &mut state),
            "ucinewgame" => handle_ucinewgame(&mut state),
            "quit" => handle_quit(),
            _ => {}
        }
    }
}

fn handle_go(parameters: &[String], state: &mut UciState) {
    let mut white_time = 1000;
    let mut black_time = 1000;

    let mut white_inc_time = 0;
    let mut black_inc_time = 0;

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
            _ => {}
        }
    }

    let time = match state.board.active_color {
        WHITE => white_time,
        BLACK => black_time,
        _ => panic!("Invalid value: state.board.active_color={}", state.board.active_color),
    };

    let inc_time = match state.board.active_color {
        WHITE => white_inc_time,
        BLACK => black_inc_time,
        _ => panic!("Invalid value: state.board.active_color={}", state.board.active_color),
    };

    let mut killers_table = Default::default();
    let mut history_table = Default::default();

    state.transposition_table.clear();
    let context = SearchContext::new(
        &mut state.board,
        time,
        inc_time,
        0,
        &mut state.transposition_table,
        &mut state.pawn_hash_table,
        &mut killers_table,
        &mut history_table,
    );

    let mut best_move = Default::default();
    for depth_result in context {
        let mut output = String::new();
        output = output.add(
            &format!(
                "info nodes {} depth {} time {} pv {}",
                depth_result.statistics.nodes_count + depth_result.statistics.q_nodes_count,
                depth_result.depth,
                depth_result.time,
                depth_result.best_move.to_text()
            )
            .to_string(),
        );

        if is_score_near_checkmate(depth_result.score) {
            let mut moves_to_mate = (depth_result.score.abs() - CHECKMATE_SCORE).abs() / 2;
            moves_to_mate *= depth_result.score.signum();

            output = output.add(&format!(" score mate {}", moves_to_mate).to_string());
        } else {
            output = output.add(&format!(" score cp {}", depth_result.score).to_string());
        }

        println!("{}", output);

        best_move = depth_result.best_move;
    }

    println!("bestmove {}", best_move.to_text());
}

fn handle_isready() {
    println!("readyok");
}

fn handle_position(parameters: &[String], state: &mut UciState) {
    if parameters.len() < 2 {
        return;
    }

    state.board = match parameters[1].as_str() {
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

    if let Some(index) = parameters.iter().position(|s| s == "moves") {
        for premade_move in &parameters[index + 1..] {
            let parsed_move = match Move::from_text(premade_move.trim(), &state.board) {
                Ok(r#move) => r#move,
                Err(message) => {
                    println!("info string Error: {}", message);
                    return;
                }
            };
            state.board.make_move(&parsed_move);
        }
    };
}

fn handle_setoption(parameters: &[String], state: &mut UciState) {
    if parameters.len() < 2 {
        return;
    }

    state.options.insert(parameters[2].to_string(), parameters[4].to_string());
}

fn handle_ucinewgame(state: &mut UciState) {
    let transposition_table_size = state.options["Hash"].parse::<usize>().unwrap() * 1024 * 1024;

    state.board = Bitboard::new_initial_position();
    state.transposition_table = TranspositionTable::new(transposition_table_size);
    state.pawn_hash_table = PawnHashTable::new(1 * 1024 * 1024);
}

fn handle_quit() {
    process::exit(0);
}
