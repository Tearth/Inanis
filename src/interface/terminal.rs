use super::uci;
use crate::evaluation::material;
use crate::evaluation::pst;
use crate::perft;
use crate::state::board::Bitboard;
use crate::state::movegen;
use crate::utils::benchmark;
use chrono::Utc;
use prettytable::cell;
use prettytable::format;
use prettytable::row;
use prettytable::Table;
use std::io;
use std::process;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const AUTHOR: &str = env!("CARGO_PKG_AUTHORS");
const REPOSITORY: &str = env!("CARGO_PKG_REPOSITORY");
const DATE: &str = env!("DATE");
const HASH: &str = env!("HASH");

pub fn run() {
    println!("Ina v{} ({}), created by {}", VERSION, DATE, AUTHOR);
    println!("Executable hash: {}", HASH);
    println!("Homepage: {}", REPOSITORY);
    println!();
    println!("Type \"help\" to get a list of available commands");

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let split: Vec<&str> = input.split(' ').collect();
        let trimmed = split[0].trim().to_lowercase();

        match trimmed.as_str() {
            "help" => handle_help(),
            "benchmark" => handle_benchmark(),
            "evaluate" => handle_evaluate(split),
            "magic" => handle_magic(),
            "perft" => handle_perft(split),
            "dperft" => handle_dperft(split),
            "qperft" => handle_qperft(split),
            "uci" => handle_uci(),
            "wah" => handle_wah(),
            "quit" => handle_quit(),
            _ => handle_unknown_command(),
        }
    }
}

fn handle_help() {
    println!("=== General ===");
    println!(" benchmark - run test for a set of positions");
    println!(" evaluate [fen] - show score for the position");
    println!(" magic - generate magic numbers");
    println!(" uci - run Universal Chess Interface");
    println!(" quit - close the application");
    println!();
    println!("=== Perft ===");
    println!(" perft [depth]");
    println!(" perft [depth] fen [fen]");
    println!(" perft [depth] moves [moves]");
    println!();
    println!("=== Divided Perft ===");
    println!(" dperft [depth]");
    println!(" dperft [depth] fen [fen]");
    println!(" dperft [depth] moves [moves]");
    println!();
    println!("=== Quick Perft ===");
    println!(" qperft [depth] [threads_count] [hashtable_size_mb]");
    println!(" qperft [depth] [threads_count] [hashtable_size_mb] fen [fen]");
    println!(" qperft [depth] [threads_count] [hashtable_size_mb] moves [moves]");
}

fn handle_benchmark() {
    println!("Starting benchmark...");
    let result = benchmark::run();
    println!("Benchmark done in {:.2} s", result.time);

    let t_nodes_count = result.nodes_count + result.q_nodes_count;
    let t_leafs_count = result.leafs_count + result.q_leafs_count;

    let mnps = ((result.nodes_count as f32) / 1000000.0) / result.time;
    let q_mnps = ((result.q_nodes_count as f32) / 1000000.0) / result.time;
    let t_mnps = (((result.nodes_count + result.q_nodes_count) as f32) / 1000000.0) / result.time;

    let mlps = ((result.leafs_count as f32) / 1000000.0) / result.time;
    let q_mlps = ((result.q_leafs_count as f32) / 1000000.0) / result.time;
    let t_mlps = (((result.leafs_count + result.q_leafs_count) as f32) / 1000000.0) / result.time;

    let beta_cutoffs_percent = ((result.beta_cutoffs as f32) / (result.nodes_count as f32)) * 100.0;
    let q_beta_cutoffs_percent = ((result.q_beta_cutoffs as f32) / (result.q_nodes_count as f32)) * 100.0;
    let t_beta_cutoffs_percent = (((result.beta_cutoffs + result.q_beta_cutoffs) as f32) / (t_nodes_count as f32)) * 100.0;

    let ordering_hits = result.perfect_cutoffs + result.non_perfect_cutoffs;
    let q_ordering_hits = result.q_perfect_cutoffs + result.q_non_perfect_cutoffs;
    let t_ordering_hits = ordering_hits + q_ordering_hits;

    let ordering_quality = (result.perfect_cutoffs as f32) / (ordering_hits as f32) * 100.0;
    let q_ordering_quality = (result.q_perfect_cutoffs as f32) / (q_ordering_hits as f32) * 100.0;
    let t_ordering_quality = ((result.perfect_cutoffs + result.q_perfect_cutoffs) as f32) / (t_ordering_hits as f32) * 100.0;

    let branching_factor = (result.nodes_count as f64) / ((result.nodes_count - result.leafs_count) as f64);
    let q_branching_factor = (result.q_nodes_count as f64) / ((result.q_nodes_count - result.q_leafs_count) as f64);
    let t_branching_factor = (t_nodes_count as f64) / ((t_nodes_count - t_leafs_count) as f64);

    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
    table.set_titles(row!["", "Normal", "Quiescence", "Total"]);

    table.add_row(row![
        "Nodes count",
        format!("{} ({:.2} MN/s)", result.nodes_count, mnps),
        format!("{} ({:.2} MN/s)", result.q_nodes_count, q_mnps),
        format!("{} ({:.2} MN/s)", t_nodes_count, t_mnps)
    ]);
    table.add_row(row![
        "Leafs count",
        format!("{} ({:.2} MN/s)", result.leafs_count, mlps),
        format!("{} ({:.2} MN/s)", result.q_leafs_count, q_mlps),
        format!("{} ({:.2} MN/s)", t_leafs_count, t_mlps)
    ]);
    table.add_row(row![
        "Beta cutoffs",
        format!("{} ({:.2}%)", result.beta_cutoffs, beta_cutoffs_percent),
        format!("{} ({:.2}%)", result.q_beta_cutoffs, q_beta_cutoffs_percent),
        format!("{} ({:.2}%)", result.beta_cutoffs + result.q_beta_cutoffs, t_beta_cutoffs_percent)
    ]);
    table.add_row(row![
        "Ordering quality",
        format!("{:.2}%", ordering_quality),
        format!("{:.2}%", q_ordering_quality),
        format!("{:.2}%", t_ordering_quality)
    ]);
    table.add_row(row![
        "Branching factor",
        format!("{:.2}", branching_factor),
        format!("{:.2}", q_branching_factor),
        format!("{:.2}", t_branching_factor)
    ]);

    table.printstd();

    println!(
        "Transposition table: {} added entries, {} hits, {} misses",
        result.tt_added_entries, result.tt_hits, result.tt_misses
    );

    let pvs_rejected_percent = ((result.pvs_rejected_searches as f32) / (result.pvs_zero_window_searches as f32)) * 100.0;
    println!(
        "PVS: {} full-window searches, {} zero-window searches, {} rejected ({:.2}%)",
        result.pvs_full_window_searches, result.pvs_zero_window_searches, result.pvs_rejected_searches, pvs_rejected_percent
    );

    let null_window_rejected_percent = ((result.null_window_rejected as f32) / (result.null_window_searches as f32)) * 100.0;
    println!(
        "Null window: {} searches, {} accepted, {} rejected ({:.2}%)",
        result.null_window_searches, result.null_window_accepted, result.null_window_rejected, null_window_rejected_percent
    );
}

fn handle_evaluate(input: Vec<&str>) {
    if input.len() < 2 {
        println!("FEN not found");
        return;
    }

    let fen = input[1..].join(" ");
    let board = match Bitboard::new_from_fen(fen.as_str()) {
        Ok(board) => board,
        Err(message) => {
            println!("{}", message);
            return;
        }
    };

    println!("Material: {}", material::evaluate(&board));
    println!("Piece-square table: {}", pst::evaluate(&board));
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
        let count = perft::normal::run(depth, &mut board, false);

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

    let result = perft::divided::run(depth, &mut board);

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
        let (count, hashtable_usage) = perft::fast::run(depth, &mut board, hashtable_size_bytes, threads_count);

        let diff = ((Utc::now() - now).num_milliseconds() as f64) / 1000.0;
        let mnps = ((count as f64) / 1000000.0) / diff;

        println!(
            "Depth {}: {} leafs in {:.2} s ({:.2} ML/s, {:.2}% of hashtable used)",
            depth, count, diff, mnps, hashtable_usage
        );
    }

    println!("Perft done!");
}

fn handle_uci() {
    uci::run();
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
