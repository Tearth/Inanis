use super::uci;
use crate::evaluation::material;
use crate::evaluation::mobility;
use crate::evaluation::mobility::EvalAux;
use crate::evaluation::pawns;
use crate::evaluation::pst;
use crate::evaluation::safety;
use crate::perft;
use crate::state::representation::Board;
use crate::testing::benchmark;
use crate::utils::percent;
use std::ffi::OsString;
use std::io;
use std::process;
use std::time::SystemTime;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const AUTHOR: &str = env!("CARGO_PKG_AUTHORS");
const REPOSITORY: &str = env!("CARGO_PKG_REPOSITORY");
const LICENSE: &str = env!("CARGO_PKG_LICENSE");
const DATE: &str = env!("DATE");
const COMPILER: &str = env!("COMPILER");
const TARGET: &str = env!("TARGET");
const PROFILE: &str = env!("PROFILE");

/// Entry point of the terminal interface and command loop. If there's a command passed through `args`, the program will end immediately
/// after completing it and printing the result.
pub fn run(args: Vec<OsString>, features: Vec<&'static str>) {
    let use_args = args.len() > 1;

    let mut header = String::new();
    header.push_str(&format!("Inanis {}", VERSION));
    if !features.is_empty() {
        header.push_str(&format!(" {}", features.join(" ")));
    }
    header.push_str(&format!(" ({}), created by {}", DATE, AUTHOR));

    println!("{}", header);
    println!("Compiler: {}", COMPILER);
    println!("Target: {}, profile: {}", TARGET, PROFILE);
    println!("Homepage: {}, license: {}", REPOSITORY, LICENSE);
    println!();
    println!("Type \"help\" to get a list of available commands");

    loop {
        let mut input = String::new();
        let tokens: Vec<&str> = if use_args {
            args[1..].iter().filter_map(|s| s.to_str()).collect()
        } else {
            let read_bytes = io::stdin().read_line(&mut input).unwrap();

            // Input stream has reached EOF, according to https://doc.rust-lang.org/stable/std/io/trait.BufRead.html#method.read_line
            if read_bytes == 0 {
                process::exit(0);
            }

            input.split(' ').map(|v| v.trim()).collect()
        };

        match tokens[0] {
            "help" => handle_help(),
            "benchmark" => handle_benchmark(),

            #[cfg(feature = "dev")]
            "dataset" => handle_dataset(tokens),

            "evaluate" => handle_evaluate(tokens),

            #[cfg(feature = "dev")]
            "magic" => handle_magic(),

            "perft" => handle_perft(tokens),
            "dperft" => handle_dperft(tokens),
            "qperft" => handle_qperft(tokens),

            #[cfg(feature = "dev")]
            "testset" => handle_testset(tokens),
            #[cfg(feature = "dev")]
            "tuner" => handle_tuner(tokens),

            "uci" => handle_uci(),
            "wah" => handle_wah(),
            "quit" => handle_quit(),
            _ => handle_unknown_command(),
        }

        if use_args {
            break;
        }
    }
}

/// Handles `help` command by printing all available ones.
fn handle_help() {
    println!("=== General ===");
    println!(" benchmark - run test for a set of positions");
    println!(" evaluate [fen] - show score for the position");
    println!(" uci - run Universal Chess Interface");
    println!(" quit - close the application");
    println!();

    #[cfg(feature = "dev")]
    {
        println!("=== Development ===");
        println!(" dataset [pgn] [output] [min_ply] [max_score] [max_diff] [density] - dataset generator");
        println!(" magic - generate magic numbers");
        println!(" testset [epd] [depth] [ttable_size] [threads_count] - run test of positions");
        println!(" tuner [epd] [output] [randomize] [k] [wdl_ratio] [threads_count] - run tuning");
        println!();
    }

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

    println!();
    println!("Benchmark done in {:.2} s", result.time);
    println!();

    let t_nodes_count = result.nodes_count + result.q_nodes_count;
    let nodes_count_percent = percent!(result.nodes_count, t_nodes_count);
    let q_nodes_count_percent = percent!(result.q_nodes_count, t_nodes_count);
    let t_mnps = (((result.nodes_count + result.q_nodes_count) as f32) / 1000000.0) / result.time;

    #[cfg(not(feature = "dev"))]
    {
        println!("Nodes:");
        println!(" Normal: {} ({:.2}%)", result.nodes_count, nodes_count_percent);
        println!(" Quiescence: {} ({:.2}%)", result.q_nodes_count, q_nodes_count_percent);
        println!(" Total: {} ({:.2} MN/s)", t_nodes_count, t_mnps);
    }

    #[cfg(feature = "dev")]
    {
        const HEADER_INDENT: usize = 25;
        const VALUE_INDENT: usize = 20;

        println!("{: <H$} {: <V$} {: <V$} {: <V$}", "", "Normal", "Quiescence", "Total", H = HEADER_INDENT, V = VALUE_INDENT);
        println!(
            "{: <H$} {: <V$} {: <V$} {: <V$}",
            "Nodes count",
            format!("{} ({:.2}%)", result.nodes_count, nodes_count_percent),
            format!("{} ({:.2}%)", result.q_nodes_count, q_nodes_count_percent),
            format!("{} ({:.2} MN/s)", t_nodes_count, t_mnps),
            H = HEADER_INDENT,
            V = VALUE_INDENT
        );

        let t_leafs_count = result.leafs_count + result.q_leafs_count;
        let leafs_count_percent = percent!(result.leafs_count, t_leafs_count);
        let q_leafs_count_percent = percent!(result.q_leafs_count, t_leafs_count);
        let t_leafs_count_percent = percent!(result.leafs_count + result.q_leafs_count, t_nodes_count);
        println!(
            "{: <H$} {: <V$} {: <V$} {: <V$}",
            "Leafs count",
            format!("{} ({:.2}%)", result.leafs_count, leafs_count_percent),
            format!("{} ({:.2}%)", result.q_leafs_count, q_leafs_count_percent),
            format!("{} ({:.2}%)", t_leafs_count, t_leafs_count_percent),
            H = HEADER_INDENT,
            V = VALUE_INDENT
        );

        let beta_cutoffs_percent = percent!(result.beta_cutoffs, result.nodes_count);
        let q_beta_cutoffs_percent = percent!(result.q_beta_cutoffs, result.q_nodes_count);
        let t_beta_cutoffs_percent = percent!(result.beta_cutoffs + result.q_beta_cutoffs, t_nodes_count);
        println!(
            "{: <H$} {: <V$} {: <V$} {: <V$}",
            "Beta cutoffs",
            format!("{} ({:.2}%)", result.beta_cutoffs, beta_cutoffs_percent),
            format!("{} ({:.2}%)", result.q_beta_cutoffs, q_beta_cutoffs_percent),
            format!("{} ({:.2}%)", result.beta_cutoffs + result.q_beta_cutoffs, t_beta_cutoffs_percent),
            H = HEADER_INDENT,
            V = VALUE_INDENT
        );

        let ordering_hits = result.perfect_cutoffs + result.non_perfect_cutoffs;
        let q_ordering_hits = result.q_perfect_cutoffs + result.q_non_perfect_cutoffs;
        let t_ordering_hits = ordering_hits + q_ordering_hits;

        let ordering_quality = percent!(result.perfect_cutoffs, ordering_hits);
        let q_ordering_quality = percent!(result.q_perfect_cutoffs, q_ordering_hits);
        let t_ordering_quality = percent!(result.perfect_cutoffs + result.q_perfect_cutoffs, t_ordering_hits);
        println!(
            "{: <H$} {: <V$} {: <V$} {: <V$}",
            "Ordering quality",
            format!("{:.2}%", ordering_quality),
            format!("{:.2}%", q_ordering_quality),
            format!("{:.2}%", t_ordering_quality),
            H = HEADER_INDENT,
            V = VALUE_INDENT
        );

        let branching_factor = (result.nodes_count as f64) / ((result.nodes_count - result.leafs_count) as f64);
        let q_branching_factor = (result.q_nodes_count as f64) / ((result.q_nodes_count - result.q_leafs_count) as f64);
        let t_branching_factor = (t_nodes_count as f64) / ((t_nodes_count - t_leafs_count) as f64);
        println!(
            "{: <H$} {: <V$} {: <V$} {: <V$}",
            "Branching factor",
            format!("{:.2}", branching_factor),
            format!("{:.2}", q_branching_factor),
            format!("{:.2}", t_branching_factor),
            H = HEADER_INDENT,
            V = VALUE_INDENT
        );

        println!();
        println!("{: <H$} {: <V$} {: <V$} {: <V$}", "", "Added", "Hits", "Misses", H = HEADER_INDENT, V = VALUE_INDENT);

        let tt_attempts = result.tt_hits + result.tt_misses;
        let tt_hits_percent = percent!(result.tt_hits, tt_attempts);
        let tt_misses_percent = percent!(result.tt_misses, tt_attempts);
        println!(
            "{: <H$} {: <V$} {: <V$} {: <V$}",
            "Transposition table",
            format!("{}", result.tt_added),
            format!("{} ({:.2}%)", result.tt_hits, tt_hits_percent),
            format!("{} ({:.2}%)", result.tt_misses, tt_misses_percent),
            H = HEADER_INDENT,
            V = VALUE_INDENT
        );

        let phtable_attempts = result.phtable_hits + result.phtable_misses;
        let phtable_hits_percent = percent!(result.phtable_hits, phtable_attempts);
        let phtable_misses_percent = percent!(result.phtable_misses, phtable_attempts);
        println!(
            "{: <H$} {: <V$} {: <V$} {: <V$}",
            "Pawn hashtable",
            format!("{}", result.phtable_added),
            format!("{} ({:.2}%)", result.phtable_hits, phtable_hits_percent),
            format!("{} ({:.2}%)", result.phtable_misses, phtable_misses_percent),
            H = HEADER_INDENT,
            V = VALUE_INDENT
        );

        println!();
        println!("{: <H$} {: <V$} {: <V$} {: <V$}", "", "Attempts", "Accepted", "Rejected", H = HEADER_INDENT, V = VALUE_INDENT);

        let snmp_accepted_percent = percent!(result.snmp_accepted, result.snmp_attempts);
        let snmp_rejected_percent = percent!(result.snmp_rejected, result.snmp_attempts);
        println!(
            "{: <H$} {: <V$} {: <V$} {: <V$}",
            "Static null move pruning",
            format!("{:.2}", result.snmp_attempts),
            format!("{} ({:.2}%)", result.snmp_accepted, snmp_accepted_percent),
            format!("{} ({:.2}%)", result.snmp_rejected, snmp_rejected_percent),
            H = HEADER_INDENT,
            V = VALUE_INDENT
        );

        let nmp_accepted_percent = percent!(result.nmp_accepted, result.nmp_attempts);
        let nmp_rejected_percent = percent!(result.nmp_rejected, result.nmp_attempts);
        println!(
            "{: <H$} {: <V$} {: <V$} {: <V$}",
            "Null move pruning",
            format!("{:.2}", result.nmp_attempts),
            format!("{} ({:.2}%)", result.nmp_accepted, nmp_accepted_percent),
            format!("{} ({:.2}%)", result.nmp_rejected, nmp_rejected_percent),
            H = HEADER_INDENT,
            V = VALUE_INDENT
        );

        let lmp_attempts = result.lmp_accepted + result.lmp_rejected;
        let lmp_accepted_percent = percent!(result.lmp_accepted, lmp_attempts);
        let lmp_rejected_percent = percent!(result.lmp_rejected, lmp_attempts);
        println!(
            "{: <H$} {: <V$} {: <V$} {: <V$}",
            "Late move pruning",
            format!("{:.2}", lmp_attempts),
            format!("{} ({:.2}%)", result.lmp_accepted, lmp_accepted_percent),
            format!("{} ({:.2}%)", result.lmp_rejected, lmp_rejected_percent),
            H = HEADER_INDENT,
            V = VALUE_INDENT
        );

        let razoring_accepted_percent = percent!(result.razoring_accepted, result.razoring_attempts);
        let razoring_rejected_percent = percent!(result.razoring_rejected, result.razoring_attempts);
        println!(
            "{: <H$} {: <V$} {: <V$} {: <V$}",
            "Razoring",
            format!("{:.2}", result.razoring_attempts),
            format!("{} ({:.2}%)", result.razoring_accepted, razoring_accepted_percent),
            format!("{} ({:.2}%)", result.razoring_rejected, razoring_rejected_percent),
            H = HEADER_INDENT,
            V = VALUE_INDENT
        );

        let total_q_score_pruning_attempts = result.q_score_pruning_accepted + result.q_score_pruning_rejected;
        let q_score_pruning_accepted_percent = percent!(result.q_score_pruning_accepted, total_q_score_pruning_attempts);
        let q_score_pruning_rejected_percent = percent!(result.q_score_pruning_rejected, total_q_score_pruning_attempts);
        println!(
            "{: <H$} {: <V$} {: <V$} {: <V$}",
            "Q score pruning",
            format!("{:.2}", total_q_score_pruning_attempts),
            format!("{} ({:.2}%)", result.q_score_pruning_accepted, q_score_pruning_accepted_percent),
            format!("{} ({:.2}%)", result.q_score_pruning_rejected, q_score_pruning_rejected_percent),
            H = HEADER_INDENT,
            V = VALUE_INDENT
        );

        let total_q_futility_prunings_attempts = result.q_futility_pruning_accepted + result.q_futility_pruning_rejected;
        let q_futility_pruning_accepted_percent = percent!(result.q_futility_pruning_accepted, total_q_futility_prunings_attempts);
        let q_futility_pruning_rejected_percent = percent!(result.q_futility_pruning_rejected, total_q_futility_prunings_attempts);
        println!(
            "{: <H$} {: <V$} {: <V$} {: <V$}",
            "Q futility pruning",
            format!("{:.2}", total_q_futility_prunings_attempts),
            format!("{} ({:.2}%)", result.q_futility_pruning_accepted, q_futility_pruning_accepted_percent),
            format!("{} ({:.2}%)", result.q_futility_pruning_rejected, q_futility_pruning_rejected_percent),
            H = HEADER_INDENT,
            V = VALUE_INDENT
        );

        println!();

        let pvs_rejected_searches_percent = percent!(result.pvs_rejected_searches, result.pvs_zero_window_searches);
        println!(
            "PVS: {} full-window searches, {} zero-window searches, {} rejected ({:.2}%)",
            result.pvs_full_window_searches, result.pvs_zero_window_searches, result.pvs_rejected_searches, pvs_rejected_searches_percent
        );

        println!(
            "Move generator stages: {} hash moves, {} captures, {} killers, {} counters, {} quiets",
            result.movegen_hash_move_stages,
            result.movegen_captures_stages,
            result.movegen_killers_stages,
            result.movegen_counters_stages,
            result.movegen_quiets_stages
        );

        println!("Transposition table move legality check: {} legal, {} illegal", result.tt_legal_hashmoves, result.tt_illegal_hashmoves);
        println!("Killers table move legality check: {} legal, {} illegal", result.ktable_legal_moves, result.ktable_illegal_moves);
        println!("Countermoves table move legality check: {} legal, {} illegal", result.cmtable_legal_moves, result.cmtable_illegal_moves);
    }

    println!();
    println!("Result hash: {}", result.result_hash);
    println!();
}

/// Handles `evaluate [fen]` command by printing evaluation for the position specified by FEN.
fn handle_evaluate(input: Vec<&str>) {
    if input.len() < 2 {
        println!("FEN parameter not found");
        return;
    }

    let fen = input[1..].join(" ");
    let board = match Board::new_from_fen(fen.as_str()) {
        Ok(board) => board,
        Err(error) => {
            println!("Invalid FEN parameter: {}", error);
            return;
        }
    };

    let mut white_aux = EvalAux::default();
    let mut black_aux = EvalAux::default();

    let material_eval = material::evaluate(&board);
    let pst_eval = pst::evaluate(&board);
    let mobility_eval = mobility::evaluate(&board, &mut white_aux, &mut black_aux);
    let safety_eval = safety::evaluate(&board, &white_aux, &black_aux);
    let pawns_eval = pawns::evaluate_without_cache(&board);

    println!("Material: {}", material_eval.taper_score(board.game_phase));
    println!("Piece-square tables: {}", pst_eval.taper_score(board.game_phase));
    println!("Mobility: {}", mobility_eval.taper_score(board.game_phase));
    println!("Safety: {}", safety_eval.taper_score(board.game_phase));
    println!("Pawns: {}", pawns_eval.taper_score(board.game_phase));

    let sum = material_eval + pst_eval + mobility_eval + safety_eval + pawns_eval;
    println!(" --- Total: {} --- ", sum.taper_score(board.game_phase));
}

/// Handles `magic` command by printing a fresh set of magic numbers.
#[cfg(feature = "dev")]
fn handle_magic() {
    use crate::state::*;

    let now = SystemTime::now();
    println!("Generating magic numbers for rook...");

    for index in ALL_SQUARES {
        println!("{},", movegen::generate_rook_magic_number(index));
    }

    println!();
    println!("Generating magic numbers for bishop...");

    for index in ALL_SQUARES {
        println!("{},", movegen::generate_bishop_magic_number(index));
    }

    let diff = now.elapsed().unwrap().as_millis();
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
        Err(error) => {
            println!("Invalid depth parameter: {}", error);
            return;
        }
    };

    let mut board = match prepare_board(&input[2..]) {
        Ok(board) => board,
        Err(error) => {
            println!("Invalid FEN parameter: {}", error);
            return;
        }
    };

    for depth in 1..=max_depth {
        let now = SystemTime::now();
        let result = perft::normal::run(depth, &mut board, false);

        let diff = (now.elapsed().unwrap().as_millis() as f64) / 1000.0;
        let mnps = ((result.nodes as f64) / 1000000.0) / diff;

        println!(
            "Depth {}: {} leafs in {:.2} s ({:.2} ML/s), {} captures, {} en passants, {} castles, {} promotions, {} checks",
            depth,
            result.nodes,
            diff,
            mnps,
            result.stats.captures,
            result.stats.en_passants,
            result.stats.castles,
            result.stats.promotions,
            result.stats.checks
        );
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
        Err(error) => {
            println!("Invalid depth parameter: {}", error);
            return;
        }
    };

    let mut board = match prepare_board(&input[2..]) {
        Ok(board) => board,
        Err(error) => {
            println!("Invalid FEN parameter: {}", error);
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
        println!("Hashtable size parameter not found");
        return;
    }

    if input.len() < 4 {
        println!("Threads count parameter not found");
        return;
    }

    let max_depth: i32 = match input[1].parse() {
        Ok(result) => result,
        Err(error) => {
            println!("Invalid depth parameter: {}", error);
            return;
        }
    };

    let threads_count: usize = match input[2].parse() {
        Ok(result) => result,
        Err(error) => {
            println!("Invalid threads count parameter: {}", error);
            return;
        }
    };

    let hashtable_size: usize = match input[3].parse() {
        Ok(result) => result,
        Err(error) => {
            println!("Invalid hashtable size parameter: {}", error);
            return;
        }
    };

    if hashtable_size == 0 {
        println!("Hashtable size must be greater than zero");
        return;
    }

    let mut board = match prepare_board(&input[4..]) {
        Ok(board) => board,
        Err(error) => {
            println!("Invalid FEN parameter: {}", error);
            return;
        }
    };

    for depth in 1..=max_depth {
        let now = SystemTime::now();
        let (count, hashtable_usage) = perft::fast::run(depth, &mut board, hashtable_size * 1024 * 1024, threads_count);

        let diff = (now.elapsed().unwrap().as_millis() as f64) / 1000.0;
        let mnps = ((count as f64) / 1000000.0) / diff;

        println!("Depth {}: {} leafs in {:.2} s ({:.2} ML/s, {:.2}% of hashtable used)", depth, count, diff, mnps, hashtable_usage);
    }

    println!("Perft done!");
}

/// Handles `testset [epd] [depth] [ttable_size] [threads_count]` command by running a fixed-`depth` search of positions stored in the `epd` file,
/// using hashtable with size specified in `ttable_size`. To classify the test as successful, the last iteration has to return the correct best move.
#[cfg(feature = "dev")]
fn handle_testset(input: Vec<&str>) {
    use crate::testing::testset;

    if input.len() < 2 {
        println!("EPD filename parameter not found");
        return;
    }

    if input.len() < 3 {
        println!("Depth parameter not found");
        return;
    }

    if input.len() < 4 {
        println!("Transposition table size parameter not found");
        return;
    }

    if input.len() < 5 {
        println!("Threads count parameter not found");
        return;
    }

    let depth = match input[2].parse() {
        Ok(value) => value,
        Err(error) => {
            println!("Invalid depth parameter: {}", error);
            return;
        }
    };

    let ttable_size: usize = match input[3].parse() {
        Ok(value) => value,
        Err(error) => {
            println!("Invalid transposition table size parameter: {}", error);
            return;
        }
    };

    if ttable_size == 0 {
        println!("Transposition table size must be greater than zero");
        return;
    }

    let threads_count = match input[4].parse() {
        Ok(value) => value,
        Err(error) => {
            println!("Invalid threads count parameter: {}", error);
            return;
        }
    };

    testset::run(input[1], depth, ttable_size * 1024 * 1024, threads_count);
}

/// Handles `tuner [epd] [output] [randomize] [k] [wdl_ratio] [threads_count]` command by running the evaluation parameters tuner. The input file is specified by `epd`
/// file with a list of positions and their expected results, and the `output` directory is used to store generated Rust sources with the optimized values.
/// Use `randomize` to initialize evaluation parameters with random values, `k` to set scaling constant and `wdl_ratio` to set the ratio between WDL and eval.
/// Multithreading is supported by `threads_count`.
#[cfg(feature = "dev")]
fn handle_tuner(input: Vec<&str>) {
    use crate::tuning::tuner;

    if input.len() < 2 {
        println!("EPD filename parameter not found");
        return;
    }

    if input.len() < 3 {
        println!("Output directory parameter not found");
        return;
    }

    if input.len() < 4 {
        println!("Random values parameter not found");
        return;
    }

    if input.len() < 5 {
        println!("Scaling constant parameter not found");
        return;
    }

    if input.len() < 6 {
        println!("WDL ratio parameter not found");
        return;
    }

    if input.len() < 7 {
        println!("Threads count parameter not found");
        return;
    }

    let random_values = match input[3].parse() {
        Ok(value) => value,
        Err(error) => {
            println!("Invalid random values parameter: {}", error);
            return;
        }
    };

    let k = if input[4] == "None" {
        None
    } else {
        match input[4].parse() {
            Ok(value) => Some(value),
            Err(error) => {
                println!("Invalid scaling constant parameter: {}", error);
                return;
            }
        }
    };

    let wdl_ratio = match input[5].parse() {
        Ok(value) => value,
        Err(error) => {
            println!("Invalid WDL ratio parameter: {}", error);
            return;
        }
    };

    let threads_count = match input[6].parse() {
        Ok(value) => value,
        Err(error) => {
            println!("Invalid threads count parameter: {}", error);
            return;
        }
    };

    tuner::run(input[1], input[2], random_values, k, wdl_ratio, threads_count);
}

/// Handles `dataset [pgn] [output] [min_ply] [max_score] [max_diff] [density]` command by running generator of the dataset for the tuner.
/// It works by parsing `pgn_filename`, and then picking random positions based on the provided restrictions like `min_ply`, `max_score`,
/// `max_differ` and `density`. Output positions are then stored in the `output_file`.
#[cfg(feature = "dev")]
fn handle_dataset(input: Vec<&str>) {
    use crate::tuning::dataset;

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
        Err(error) => {
            println!("Invalid minimal ply parameter: {}", error);
            return;
        }
    };

    let max_score = match input[4].parse() {
        Ok(value) => value,
        Err(error) => {
            println!("Invalid maximal quiescence score parameter: {}", error);
            return;
        }
    };

    let max_diff = match input[5].parse() {
        Ok(value) => value,
        Err(error) => {
            println!("Invalid maximal score difference parameter: {}", error);
            return;
        }
    };

    let density = match input[6].parse() {
        Ok(value) => value,
        Err(error) => {
            println!("Invalid density parameter: {}", error);
            return;
        }
    };

    dataset::run(input[1], input[2], min_ply, max_score, max_diff, density);
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

/// Handles unknown command by printing an error.
fn handle_unknown_command() {
    println!("Unknown command, type \"help\" to get a list of available ones");
}

/// Creates a new board based on the input with FEN or moves list - returns [Err] if internal parser failed.
fn prepare_board(params: &[&str]) -> Result<Board, String> {
    if params.is_empty() {
        return Ok(Board::new_initial_position());
    }

    match params[0] {
        "fen" => {
            let fen = params[1..].join(" ");
            Board::new_from_fen(fen.as_str())
        }
        "moves" => Board::new_from_moves(&params[1..]),
        _ => Err(format!("Invalid mode: parameter[0]={}", params[0])),
    }
}
