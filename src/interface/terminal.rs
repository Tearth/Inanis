use super::uci;
use crate::evaluation::material;
use crate::evaluation::mobility;
use crate::evaluation::pawns;
use crate::evaluation::pst;
use crate::evaluation::safety;
use crate::perft;
use crate::state::board::Bitboard;
use crate::state::movegen::MagicContainer;
use crate::state::*;
use crate::utils::benchmark;
use crate::utils::test;
use crate::utils::tuner;
use crate::utils::tunerset;
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
const COMPILER: &str = env!("COMPILER");

/// Entry point of the terminal interface and command loop.
pub fn run(target_features: Vec<String>) {
    let header = if target_features.is_empty() {
        format!("Inanis {} ({}), created by {}", VERSION, DATE, AUTHOR)
    } else {
        format!("Inanis {} {} ({}), created by {}", VERSION, target_features.join(" "), DATE, AUTHOR)
    };

    println!("{}", header);
    println!("Executable hash: {}", HASH);
    println!("Compiler: {}", COMPILER);
    println!("Homepage: {}", REPOSITORY);
    println!();
    println!("Type \"help\" to get a list of available commands");

    loop {
        let mut input = String::new();
        let read_bytes = io::stdin().read_line(&mut input).unwrap();

        if read_bytes == 0 {
            process::exit(0);
        }

        let tokens: Vec<&str> = input.split(' ').map(|v| v.trim()).collect();
        match tokens[0] {
            "help" => handle_help(),
            "benchmark" => handle_benchmark(),
            "evaluate" => handle_evaluate(tokens),
            "magic" => handle_magic(),
            "perft" => handle_perft(tokens),
            "dperft" => handle_dperft(tokens),
            "qperft" => handle_qperft(tokens),
            "test" => handle_test(tokens),
            "tuner" => handle_tuner(tokens),
            "tunerset" => handle_tunerset(tokens),
            "uci" => handle_uci(),
            "wah" => handle_wah(),
            "quit" => handle_quit(),
            _ => handle_unknown_command(),
        }
    }
}

/// Handles `help` command by printing all available ones.
fn handle_help() {
    println!("=== General ===");
    println!(" benchmark - run test for a set of positions");
    println!(" evaluate [fen] - show score for the position");
    println!(" magic - generate magic numbers");
    println!(" test [epd] [depth] [transposition_table_size] [threads_count] - run test of positions");
    println!(" tuner [epd] [output] [lock_material] [randomize] [threads_count] - run tuning");
    println!(" tunerset [pgn] [output] [min_ply] [max_score] [max_diff] [density] - dataset generator");
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

/// Handles `benchmark` command by running a fixed-depth search for a set of static positions and printing diagnostic data.
fn handle_benchmark() {
    println!("Starting benchmark...");
    let result = benchmark::run();
    println!("Benchmark done in {:.2} s", result.time);

    let mut search_table = Table::new();
    search_table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
    search_table.set_titles(row!["", "Normal", "Quiescence", "Total"]);

    let t_nodes_count = result.nodes_count + result.q_nodes_count;
    let t_leafs_count = result.leafs_count + result.q_leafs_count;

    let nodes_count_percent = percent(result.nodes_count, t_nodes_count);
    let q_nodes_count_percent = percent(result.q_nodes_count, t_nodes_count);
    let t_mnps = (((result.nodes_count + result.q_nodes_count) as f32) / 1000000.0) / result.time;
    search_table.add_row(row![
        "Nodes count",
        format!("{} ({:.2}%)", result.nodes_count, nodes_count_percent),
        format!("{} ({:.2}%)", result.q_nodes_count, q_nodes_count_percent),
        format!("{} ({:.2} MN/s)", t_nodes_count, t_mnps)
    ]);

    let leafs_count_percent = percent(result.leafs_count, t_leafs_count);
    let q_leafs_count_percent = percent(result.q_leafs_count, t_leafs_count);
    let t_leafs_count_percent = percent(result.leafs_count + result.q_leafs_count, t_nodes_count);
    search_table.add_row(row![
        "Leafs count",
        format!("{} ({:.2}%)", result.leafs_count, leafs_count_percent),
        format!("{} ({:.2}%)", result.q_leafs_count, q_leafs_count_percent),
        format!("{} ({:.2}%)", t_leafs_count, t_leafs_count_percent)
    ]);

    let beta_cutoffs_percent = percent(result.beta_cutoffs, result.nodes_count);
    let q_beta_cutoffs_percent = percent(result.q_beta_cutoffs, result.q_nodes_count);
    let t_beta_cutoffs_percent = percent(result.beta_cutoffs + result.q_beta_cutoffs, t_nodes_count);
    search_table.add_row(row![
        "Beta cutoffs",
        format!("{} ({:.2}%)", result.beta_cutoffs, beta_cutoffs_percent),
        format!("{} ({:.2}%)", result.q_beta_cutoffs, q_beta_cutoffs_percent),
        format!("{} ({:.2}%)", result.beta_cutoffs + result.q_beta_cutoffs, t_beta_cutoffs_percent)
    ]);

    let ordering_hits = result.perfect_cutoffs + result.non_perfect_cutoffs;
    let q_ordering_hits = result.q_perfect_cutoffs + result.q_non_perfect_cutoffs;
    let t_ordering_hits = ordering_hits + q_ordering_hits;

    let ordering_quality = percent(result.perfect_cutoffs, ordering_hits);
    let q_ordering_quality = percent(result.q_perfect_cutoffs, q_ordering_hits);
    let t_ordering_quality = percent(result.perfect_cutoffs + result.q_perfect_cutoffs, t_ordering_hits);
    search_table.add_row(row![
        "Ordering quality",
        format!("{:.2}%", ordering_quality),
        format!("{:.2}%", q_ordering_quality),
        format!("{:.2}%", t_ordering_quality)
    ]);

    let branching_factor = (result.nodes_count as f64) / ((result.nodes_count - result.leafs_count) as f64);
    let q_branching_factor = (result.q_nodes_count as f64) / ((result.q_nodes_count - result.q_leafs_count) as f64);
    let t_branching_factor = (t_nodes_count as f64) / ((t_nodes_count - t_leafs_count) as f64);
    search_table.add_row(row![
        "Branching factor",
        format!("{:.2}", branching_factor),
        format!("{:.2}", q_branching_factor),
        format!("{:.2}", t_branching_factor)
    ]);

    search_table.printstd();

    let mut cache_table = Table::new();
    cache_table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
    cache_table.set_titles(row!["", "Added", "Hits", "Misses", "Collisions"]);

    let tt_misses_percent = percent(result.tt_misses, result.tt_hits);
    let tt_collisions_percent = percent(result.tt_collisions, result.tt_hits);
    cache_table.add_row(row![
        "Transposition table",
        format!("{}", result.tt_added),
        format!("{}", result.tt_hits),
        format!("{} ({:.2}%)", result.tt_misses, tt_misses_percent),
        format!("{} ({:.2}%)", result.tt_collisions, tt_collisions_percent)
    ]);

    let pawn_hashtable_misses_percent = percent(result.pawn_hashtable_misses, result.pawn_hashtable_hits);
    let pawn_hashtable_collisions_percent = percent(result.pawn_hashtable_collisions, result.pawn_hashtable_hits);
    cache_table.add_row(row![
        "Pawn hashtable",
        format!("{}", result.pawn_hashtable_added),
        format!("{}", result.pawn_hashtable_hits),
        format!("{} ({:.2}%)", result.pawn_hashtable_misses, pawn_hashtable_misses_percent),
        format!("{} ({:.2}%)", result.pawn_hashtable_collisions, pawn_hashtable_collisions_percent)
    ]);

    cache_table.printstd();

    let mut prunings_reductions_table = Table::new();
    prunings_reductions_table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
    prunings_reductions_table.set_titles(row!["", "Attempts", "Accepted", "Rejected"]);

    let static_null_move_pruning_accepted_percent = percent(result.static_null_move_pruning_accepted, result.static_null_move_pruning_attempts);
    let static_null_move_pruning_rejected_percent = percent(result.static_null_move_pruning_rejected, result.static_null_move_pruning_attempts);
    prunings_reductions_table.add_row(row![
        "Static null move pruning",
        format!("{:.2}", result.static_null_move_pruning_attempts),
        format!(
            "{} ({:.2}%)",
            result.static_null_move_pruning_accepted, static_null_move_pruning_accepted_percent
        ),
        format!(
            "{} ({:.2}%)",
            result.static_null_move_pruning_rejected, static_null_move_pruning_rejected_percent
        )
    ]);

    let null_move_pruning_accepted_percent = percent(result.null_move_pruning_accepted, result.null_move_pruning_attempts);
    let null_move_pruning_rejected_percent = percent(result.null_move_pruning_rejected, result.null_move_pruning_attempts);
    prunings_reductions_table.add_row(row![
        "Null move pruning",
        format!("{:.2}", result.null_move_pruning_attempts),
        format!("{} ({:.2}%)", result.null_move_pruning_accepted, null_move_pruning_accepted_percent),
        format!("{} ({:.2}%)", result.null_move_pruning_rejected, null_move_pruning_rejected_percent)
    ]);

    let late_move_pruning_attempts = result.late_move_pruning_accepted + result.late_move_pruning_rejected;
    let late_move_pruning_accepted_percent = percent(result.late_move_pruning_accepted, late_move_pruning_attempts);
    let late_move_pruning_rejected_percent = percent(result.late_move_pruning_rejected, late_move_pruning_attempts);
    prunings_reductions_table.add_row(row![
        "Late move pruning",
        format!("{:.2}", late_move_pruning_attempts),
        format!("{} ({:.2}%)", result.late_move_pruning_accepted, late_move_pruning_accepted_percent),
        format!("{} ({:.2}%)", result.late_move_pruning_rejected, late_move_pruning_rejected_percent)
    ]);

    let razoring_accepted_percent = percent(result.razoring_accepted, result.razoring_attempts);
    let razoring_rejected_percent = percent(result.razoring_rejected, result.razoring_attempts);
    prunings_reductions_table.add_row(row![
        "Razoring",
        format!("{:.2}", result.razoring_attempts),
        format!("{} ({:.2}%)", result.razoring_accepted, razoring_accepted_percent),
        format!("{} ({:.2}%)", result.razoring_rejected, razoring_rejected_percent)
    ]);

    let total_q_score_pruning_attempts = result.q_score_pruning_accepted + result.q_score_pruning_rejected;
    let q_score_pruning_accepted_percent = percent(result.q_score_pruning_accepted, total_q_score_pruning_attempts);
    let q_score_pruning_rejected_percent = percent(result.q_score_pruning_rejected, total_q_score_pruning_attempts);
    prunings_reductions_table.add_row(row![
        "Q score pruning",
        format!("{:.2}", total_q_score_pruning_attempts),
        format!("{} ({:.2}%)", result.q_score_pruning_accepted, q_score_pruning_accepted_percent),
        format!("{} ({:.2}%)", result.q_score_pruning_rejected, q_score_pruning_rejected_percent)
    ]);

    let total_q_futility_prunings_attempts = result.q_futility_pruning_accepted + result.q_futility_pruning_rejected;
    let q_futility_pruning_accepted_percent = percent(result.q_futility_pruning_accepted, total_q_futility_prunings_attempts);
    let q_futility_pruning_rejected_percent = percent(result.q_futility_pruning_rejected, total_q_futility_prunings_attempts);
    prunings_reductions_table.add_row(row![
        "Q futility pruning",
        format!("{:.2}", total_q_futility_prunings_attempts),
        format!("{} ({:.2}%)", result.q_futility_pruning_accepted, q_futility_pruning_accepted_percent),
        format!("{} ({:.2}%)", result.q_futility_pruning_rejected, q_futility_pruning_rejected_percent)
    ]);

    prunings_reductions_table.printstd();

    let pvs_rejected_searches_percent = percent(result.pvs_rejected_searches, result.pvs_zero_window_searches);
    println!(
        "PVS: {} full-window searches, {} zero-window searches, {} rejected ({:.2}%)",
        result.pvs_full_window_searches, result.pvs_zero_window_searches, result.pvs_rejected_searches, pvs_rejected_searches_percent
    );

    println!(
        "Move generator stages: {} hash moves, {} captures, {} quiet",
        result.move_generator_hash_move_stages, result.move_generator_captures_stages, result.move_generator_quiet_moves_stages
    );

    println!(
        "Transposition table move legality check: {} legal, {} illegal",
        result.tt_legal_hashmoves, result.tt_illegal_hashmoves
    );

    println!("Result hash: {}", result.result_hash);
}

/// Handles `evaluate [fen]` command by printing evaluation for the position specified by FEN.
fn handle_evaluate(input: Vec<&str>) {
    if input.len() < 2 {
        println!("FEN not found");
        return;
    }

    let fen = input[1..].join(" ");
    let board = match Bitboard::new_from_fen(fen.as_str(), None, None, None, None, None) {
        Ok(board) => board,
        Err(message) => {
            println!("{}", message);
            return;
        }
    };

    let mut white_attack_mask = 0;
    let mut black_attack_mask = 0;

    let material_evaluation = material::evaluate(&board);
    let pst_evaluation = pst::evaluate(&board);
    let mobility_evaluation = mobility::evaluate(&board, &mut white_attack_mask, &mut black_attack_mask);
    let safety_evaluation = safety::evaluate(&board, white_attack_mask, black_attack_mask);
    let pawns_evaluation = pawns::evaluate_without_cache(&board);

    println!("Material: {}", material_evaluation);
    println!("Piece-square tables: {}", pst_evaluation);
    println!("Mobility: {}", mobility_evaluation);
    println!("Safety: {}", safety_evaluation);
    println!("Pawns: {}", pawns_evaluation);

    let sum = material_evaluation + pst_evaluation + mobility_evaluation + safety_evaluation + pawns_evaluation;
    let board_evaluation = board.evaluate_without_cache(WHITE);

    if sum != board_evaluation {
        println!(" --- Integration check failed --- ");
    } else {
        println!(" --- Total: {} --- ", sum);
    }
}

/// Handles `magic` command by printing a fresh set of magic numbers.
fn handle_magic() {
    let now = Utc::now();
    let magic = MagicContainer::default();
    println!("Generating magic numbers for rook...");

    for index in 0..64 {
        println!("{},", magic.generate_rook_magic_number(index));
    }

    println!();
    println!("Generating magic numbers for bishop...");

    for index in 0..64 {
        println!("{},", magic.generate_bishop_magic_number(index));
    }

    let diff = (Utc::now() - now).num_milliseconds();
    println!("Done! Magic numbers generated in {} ms", diff);
}

/// Handles `perft [depth]`, `perft [depth] fen [fen]` and `perft [depth] moves [moves]` commands by running a perft test to the depth specified
/// by `depth` parameter. The initial position can be specified by FEN, a list of moves, or just omitted (so the default start position will be taken).
fn handle_perft(input: Vec<&str>) {
    if input.len() < 2 {
        println!("Depth parameter not found");
        return;
    }

    let max_depth: i32 = match input[1].parse() {
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

/// Handles `dperft [depth]`, `dperft [depth] fen [fen]` and `dperft [depth] moves [moves]` commands by running a divided perft test to the depth specified
/// by `depth` parameter. The initial position can be specified by FEN, a list of moves, or just omitted (so the default start position will be taken).
fn handle_dperft(input: Vec<&str>) {
    if input.len() < 2 {
        println!("Depth parameter not found");
        return;
    }

    let depth: i32 = match input[1].parse() {
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

/// Handles `qperft [depth] [threads_count] [hashtable_size_mb]`, `qperft [depth] [threads_count] [hashtable_size_mb] fen [fen]` and
/// `qperft [depth] [threads_count] [hashtable_size_mb] moves [moves]` commands by running a quick perft test to the depth specified by `depth` parameter.
/// This kind of perft also supports multithreading (specified by `threads_count`) and caching results in the hashtable (with size specified by `hashtable_size_mb`).
/// The initial position can be specified by FEN, a list of moves, or just omitted (so the default start position will be taken).
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

    let max_depth: i32 = match input[1].parse() {
        Ok(result) => result,
        Err(_) => {
            println!("Invalid depth parameter");
            return;
        }
    };

    let threads_count: usize = match input[2].parse() {
        Ok(result) => result,
        Err(_) => {
            println!("Invalid threads count parameter");
            return;
        }
    };

    let hashtable_size: usize = match input[3].parse() {
        Ok(result) => result,
        Err(_) => {
            println!("Invalid hashtable size parameter");
            return;
        }
    };

    if hashtable_size == 0 {
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
        let (count, hashtable_usage) = perft::fast::run(depth, &mut board, hashtable_size * 1024 * 1024, threads_count);

        let diff = ((Utc::now() - now).num_milliseconds() as f64) / 1000.0;
        let mnps = ((count as f64) / 1000000.0) / diff;

        println!(
            "Depth {}: {} leafs in {:.2} s ({:.2} ML/s, {:.2}% of hashtable used)",
            depth, count, diff, mnps, hashtable_usage
        );
    }

    println!("Perft done!");
}

/// Handles `test [epd] [depth] [transposition_table_size] [threads_count]` command by running a fixed-`depth` search of positions stored in the `epd` file,
/// using hashtable with size specified in `transposition_table_size`. To classify the test as successful, the last iteration has to return the correct best move.
fn handle_test(input: Vec<&str>) {
    if input.len() < 2 {
        println!("EPD filename parameter not found");
        return;
    }

    if input.len() < 3 {
        println!("Depth parameter not found");
        return;
    }

    if input.len() < 4 {
        println!("Transposition table size not found");
        return;
    }

    if input.len() < 5 {
        println!("Threads count not found");
        return;
    }

    let depth = match input[2].parse() {
        Ok(value) => value,
        Err(_) => {
            println!("Invalid depth");
            return;
        }
    };

    let transposition_table_size: usize = match input[3].parse() {
        Ok(value) => value,
        Err(_) => {
            println!("Invalid transposition table size parameter");
            return;
        }
    };

    if transposition_table_size == 0 {
        println!("Transposition table size must be greater than zero");
        return;
    }

    let threads_count = match input[4].parse() {
        Ok(value) => value,
        Err(_) => {
            println!("Invalid threads count");
            return;
        }
    };

    test::run(input[1], depth, transposition_table_size * 1024 * 1024, threads_count);
}

/// Handles `tuner [epd] [output] [lock_material] [randomize] [threads_count]` command by running the evaluation parameters tuner. The input file is specified by `epd`
/// file with a list of positions and their expected results, and the `output` directory is used to store generated Rust sources with the optimized values. Use
/// `lock_material` to disable tuner for piece values, and `randomize` to initialize evaluation parameters with random values. Multithreading is supported by `threads_count`.
fn handle_tuner(input: Vec<&str>) {
    if input.len() < 2 {
        println!("EPD filename parameter not found");
        return;
    }

    if input.len() < 3 {
        println!("Output directory parameter not found");
        return;
    }

    if input.len() < 4 {
        println!("Lock material parameter not found");
        return;
    }

    if input.len() < 5 {
        println!("Random values parameter not found");
        return;
    }

    if input.len() < 6 {
        println!("Threads count not found");
        return;
    }

    let lock_material = match input[3].parse() {
        Ok(value) => value,
        Err(_) => {
            println!("Invalid flag");
            return;
        }
    };

    let random_values = match input[4].parse() {
        Ok(value) => value,
        Err(_) => {
            println!("Invalid flag");
            return;
        }
    };

    let threads_count = match input[5].parse() {
        Ok(value) => value,
        Err(_) => {
            println!("Invalid threads count");
            return;
        }
    };

    tuner::run(input[1], input[2], lock_material, random_values, threads_count);
}

fn handle_tunerset(input: Vec<&str>) {
    if input.len() < 2 {
        println!("PGN filename parameter not found");
        return;
    }

    if input.len() < 3 {
        println!("Output directory parameter not found");
        return;
    }

    if input.len() < 4 {
        println!("Minimal ply parameter not found");
        return;
    }

    if input.len() < 5 {
        println!("Maximal score parameter not found");
        return;
    }

    if input.len() < 6 {
        println!("Maximal score difference parameter not found");
        return;
    }

    if input.len() < 7 {
        println!("Maximal density parameter not found");
        return;
    }

    let min_ply = match input[3].parse() {
        Ok(value) => value,
        Err(_) => {
            println!("Invalid minimal ply value");
            return;
        }
    };

    let max_score = match input[4].parse() {
        Ok(value) => value,
        Err(_) => {
            println!("Invalid maximal quiescence score value");
            return;
        }
    };

    let max_diff = match input[5].parse() {
        Ok(value) => value,
        Err(_) => {
            println!("Invalid maximal score difference score value");
            return;
        }
    };

    let density = match input[6].parse() {
        Ok(value) => value,
        Err(_) => {
            println!("Invalid positions per game value");
            return;
        }
    };

    tunerset::run(input[1], input[2], min_ply, max_score, max_diff, density);
}

/// Handles `uci` command by entering into the UCI (Universal Chess Interface) mode.
fn handle_uci() {
    uci::run();
}

/// Handles `wah` command by printing WAH.                  
fn handle_wah() {
    println!("WAH");
}

/// Handles `quit` command by exiting process.
fn handle_quit() {
    process::exit(0);
}

/// Handles unknown command by printing warning message.
fn handle_unknown_command() {
    println!("Unknown command, type \"help\" to get a list of available ones");
}

/// Creates a new board based on the input with FEN or moves list - returns [Err] if internal parser failed.
fn prepare_board(parameters: &[&str]) -> Result<Bitboard, &'static str> {
    if parameters.is_empty() {
        return Ok(Bitboard::new_initial_position(None, None, None, None, None));
    }

    match parameters[0] {
        "fen" => {
            let fen = parameters[1..].join(" ");
            Bitboard::new_from_fen(fen.as_str(), None, None, None, None, None)
        }
        "moves" => Bitboard::new_from_moves(&parameters[1..], None, None, None, None, None),
        _ => Err("Invalid parameters"),
    }
}

/// Helper function to calculate percent of `from` within `all`.
fn percent(from: u64, all: u64) -> f32 {
    ((from as f32) / (all as f32)) * 100.0
}
