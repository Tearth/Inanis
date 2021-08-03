use crate::board::Bitboard;
use crate::movescan::Move;
use std::collections::HashMap;
use std::io;
use std::ops::Index;
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

        let split: Vec<&str> = input.split(' ').collect();
        let trimmed = split[0].trim().to_lowercase();

        match trimmed.as_str() {
            "go" => handle_go(),
            "isready" => handle_isready(),
            "position" => handle_position(&split, &mut state),
            "setoption" => handle_setoption(&split, &mut state),
            "ucinewgame" => handle_ucinewgame(&mut state),
            "quit" => handle_quit(),
            _ => {}
        }
    }
}

fn handle_go() {}

fn handle_isready() {
    println!("readyok");
}

fn handle_position(parameters: &[&str], state: &mut UciState) {
    if parameters.len() < 2 {
        return;
    }

    state.board = match parameters[1] {
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

    if let Some(index) = parameters.iter().position(|&s| s == "moves") {
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

fn handle_setoption(parameters: &[&str], state: &mut UciState) {
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
