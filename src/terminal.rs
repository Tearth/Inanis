use crate::board::Bitboard;
use crate::movegen;
use crate::perft;
use chrono::Utc;
use std::io;
use std::process;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const AUTHOR: &str = env!("CARGO_PKG_AUTHORS");
const REPOSITORY: &str = env!("CARGO_PKG_REPOSITORY");
const DATE: &str = env!("DATE");
const HASH: &str = env!("HASH");

pub fn init() {
    println!("Ina v{} ({}), created by {}", VERSION, DATE, AUTHOR);
    println!("Executable hash: {}", HASH);
    println!("Homepage: {}", REPOSITORY);
    println!();
    println!("Type \"help\" to get a list of available commands");
}

pub fn run() {
    loop {
        let input = read_line();
        let split: Vec<&str> = input.split(' ').collect();
        let trimmed = split[0].trim().to_lowercase();

        match trimmed.as_str() {
            "help" => handle_help(),
            "magic" => handle_magic(),
            "perft" => handle_perft(split),
            "dperft" => handle_dperft(split),
            "qperft" => handle_qperft(split),
            "wah" => handle_wah(),
            "quit" => handle_quit(),
            _ => handle_unknown_command(),
        }
    }
}

fn handle_help() {
    println!("List of available commands:");
    println!(" === General ===");
    println!("  magic - generate magic numbers");
    println!("  quit - close the application");
    println!();
    println!(" === Perft === ");
    println!("  perft [depth]");
    println!("  perft [depth] fen [fen]");
    println!("  perft [depth] moves [moves]");
    println!();
    println!(" === Divided Perft === ");
    println!("  dperft [depth]");
    println!("  dperft [depth] fen [fen]");
    println!("  dperft [depth] moves [moves]");
    println!();
    println!(" === Quick Perft === ");
    println!("  qperft [depth] [threads_count] [hashtable_size_mb]");
    println!("  qperft [depth] [threads_count] [hashtable_size_mb] fen [fen]");
    println!("  qperft [depth] [threads_count] [hashtable_size_mb] moves [moves]");
}

fn handle_magic() {
    let now = Utc::now();
    println!("Generating magic numbers for rook...");

    for index in 0..64 {
        println!("{},", movegen::generate_rook_number_for_field(index));
    }

    println!();
    println!("Generating magic numbers for bishop...");

    for index in 0..64 {
        println!("{},", movegen::generate_bishop_number_for_field(index));
    }

    let diff = (Utc::now() - now).num_milliseconds();
    println!("Done! Magic numbers generated in {} ms", diff);
}

fn handle_perft(input: Vec<&str>) {
    if input.len() < 2 {
        println!("Depth parameter not found");
        return;
    }

    let max_depth: i32 = match input[1].trim().parse() {
        Ok(result) => result,
        Err(_) => {
            println!("Invalid depth parameter");
            return;
        }
    };

    let mut board = match prepare_board(&input[2..]) {
        Ok(board) => board,
        Err(message) => {
            println!("{}", message);
            return;
        }
    };

    for depth in 1..max_depth + 1 {
        let now = Utc::now();
        let count = perft::run(depth, &mut board, false);

        let diff = ((Utc::now() - now).num_milliseconds() as f64) / 1000.0;
        let mnps = ((count as f64) / 1000000.0) / diff;

        println!("Depth {}: {} leafs in {:.2} s ({:.2} ML/s)", depth, count, diff, mnps);
    }

    println!("Perft done!");
}

fn handle_dperft(input: Vec<&str>) {
    if input.len() < 2 {
        println!("Depth parameter not found");
        return;
    }

    let depth: i32 = match input[1].trim().parse() {
        Ok(result) => result,
        Err(_) => {
            println!("Invalid depth parameter");
            return;
        }
    };

    let mut board = match prepare_board(&input[2..]) {
        Ok(board) => board,
        Err(message) => {
            println!("{}", message);
            return;
        }
    };

    let result = perft::run_divided(depth, &mut board);

    let mut total_leafs = 0;
    for r#move in result {
        println!("{}: {} leafs", r#move.0, r#move.1);
        total_leafs += r#move.1;
    }

    println!();
    println!("{} leafs total", total_leafs);
    println!("Perft done!");
}

fn handle_qperft(input: Vec<&str>) {
    if input.len() < 2 {
        println!("Depth parameter not found");
        return;
    }

    if input.len() < 3 {
        println!("Hashtable size not found");
        return;
    }

    if input.len() < 4 {
        println!("Threads count not found");
        return;
    }

    let max_depth: i32 = match input[1].trim().parse() {
        Ok(result) => result,
        Err(_) => {
            println!("Invalid depth parameter");
            return;
        }
    };

    let threads_count: usize = match input[2].trim().parse() {
        Ok(result) => result,
        Err(_) => {
            println!("Invalid threads count parameter");
            return;
        }
    };

    let hashtable_size: usize = match input[3].trim().parse() {
        Ok(result) => result,
        Err(_) => {
            println!("Invalid hashtable size parameter");
            return;
        }
    };
    let hashtable_size_bytes = hashtable_size * 1024 * 1024;

    if hashtable_size_bytes == 0 {
        println!("Hashtable size must be greater than zero");
        return;
    }

    let mut board = match prepare_board(&input[4..]) {
        Ok(board) => board,
        Err(message) => {
            println!("{}", message);
            return;
        }
    };

    for depth in 1..=max_depth {
        let now = Utc::now();
        let (count, hashtable_usage) = perft::run_fast(depth, &mut board, hashtable_size_bytes, threads_count);

        let diff = ((Utc::now() - now).num_milliseconds() as f64) / 1000.0;
        let mnps = ((count as f64) / 1000000.0) / diff;

        println!(
            "Depth {}: {} leafs in {:.2} s ({:.2} ML/s, {:.2}% of hashtable used)",
            depth, count, diff, mnps, hashtable_usage
        );
    }

    println!("Perft done!");
}

fn handle_wah() {
    println!("WAH");
}

fn handle_quit() {
    process::exit(0);
}

fn handle_unknown_command() {
    println!("Unknown command, type \"help\" to get a list of available ones");
}

fn read_line() -> String {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();

    buffer
}

fn prepare_board(parameters: &[&str]) -> Result<Bitboard, &'static str> {
    if parameters.is_empty() {
        return Ok(Bitboard::new_default());
    }

    match parameters[0] {
        "fen" => {
            let fen = parameters[1..].join(" ");
            Bitboard::new_from_fen(fen.as_str())
        }
        "moves" => Bitboard::new_from_moves(&parameters[1..]),
        _ => Err("Invalid parameters"),
    }
}
