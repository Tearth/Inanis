use crate::board::common::*;
use crate::board::movescan::Move;
use crate::board::movescan::MoveFlags;
use crate::board::representation::Bitboard;
use crate::engine::context::SearchContext;
use std::collections::HashMap;
use std::io;
use std::process;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const AUTHOR: &str = env!("CARGO_PKG_AUTHORS");
const DATE: &str = env!("DATE");

struct UciState {
    board: Bitboard,
    options: HashMap<String, String>,
}

impl UciState {
    pub fn new() -> UciState {
        UciState {
            board: Bitboard::new_default(),
            options: HashMap::new(),
        }
    }
}

pub fn run() {
    let mut state = UciState::new();
    state.options.insert("Hash".to_string(), "1".to_string());

    println!("id name Ina v{} ({})", VERSION, DATE);
    println!("id author {}", AUTHOR);
    println!("option name Hash type spin default 1 min 1 max 128");
    println!("uciok");

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let tokens: Vec<String> = input.split(' ').map(|p| p.trim().to_lowercase()).collect();
        match tokens[0].as_str() {
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

    let context = SearchContext::new(&mut state.board, time, inc_time);
    let mut best_move = Move::new(0, 0, MoveFlags::QUIET);

    for depth_result in context {
        println!(
            "info score cp {} nodes {} depth {} time {} pv {}",
            depth_result.score,
            depth_result.statistics.nodes_count + depth_result.statistics.q_nodes_count,
            depth_result.depth,
            depth_result.time,
            depth_result.best_move.to_text()
        );

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
                Err(_) => {
                    return;
                }
            }
        }
        _ => Bitboard::new_default(),
    };

    if let Some(index) = parameters.iter().position(|s| s == "moves") {
        for premade_move in &parameters[index + 1..] {
            let parsed_move = match Move::from_text(premade_move.trim(), &state.board) {
                Ok(r#move) => r#move,
                Err(_) => {
                    return;
                }
            };
            state.board.make_move_active_color(&parsed_move);
        }
    };
}

fn handle_setoption(parameters: &[String], state: &mut UciState) {
    if parameters.len() < 2 {
        return;
    }

    state.options.insert(parameters[0].to_string(), parameters[1].to_string());
}

fn handle_ucinewgame(state: &mut UciState) {
    state.board = Bitboard::new_default();
}

fn handle_quit() {
    process::exit(0);
}
