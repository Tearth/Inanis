use crate::cache::pawns::PHTable;
use crate::cache::search::TTable;
use crate::engine::context::SearchContext;
use crate::state::representation::Board;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::time::SystemTime;

#[derive(Default)]
pub struct BenchmarkResult {
    pub time: f32,

    pub nodes_count: u64,
    pub q_nodes_count: u64,
    pub leafs_count: u64,
    pub q_leafs_count: u64,

    pub beta_cutoffs: u64,
    pub q_beta_cutoffs: u64,

    #[cfg(feature = "dev")]
    pub perfect_cutoffs: u64,
    pub q_perfect_cutoffs: u64,
    pub non_perfect_cutoffs: u64,
    pub q_non_perfect_cutoffs: u64,

    pub pvs_full_window_searches: u64,
    pub pvs_zero_window_searches: u64,
    pub pvs_rejected_searches: u64,

    pub snmp_attempts: u64,
    pub snmp_accepted: u64,
    pub snmp_rejected: u64,

    pub nmp_attempts: u64,
    pub nmp_accepted: u64,
    pub nmp_rejected: u64,

    pub lmp_accepted: u64,
    pub lmp_rejected: u64,

    pub razoring_attempts: u64,
    pub razoring_accepted: u64,
    pub razoring_rejected: u64,

    pub q_score_pruning_accepted: u64,
    pub q_score_pruning_rejected: u64,

    pub q_futility_pruning_accepted: u64,
    pub q_futility_pruning_rejected: u64,

    pub tt_added: u64,
    pub tt_hits: u64,
    pub tt_misses: u64,

    pub tt_legal_hashmoves: u64,
    pub tt_illegal_hashmoves: u64,
    pub ktable_legal_moves: u64,
    pub ktable_illegal_moves: u64,
    pub cmtable_legal_moves: u64,
    pub cmtable_illegal_moves: u64,

    pub phtable_added: u64,
    pub phtable_hits: u64,
    pub phtable_misses: u64,

    pub movegen_hash_move_stages: u64,
    pub movegen_captures_stages: u64,
    pub movegen_killers_stages: u64,
    pub movegen_counters_stages: u64,
    pub movegen_quiets_stages: u64,

    pub result_hash: u16,
}

/// Runs a benchmark by performing a fixed-depth search for the built-in list of positions.
pub fn run() -> BenchmarkResult {
    const BENCHMARK_POSITIONS: [&str; 30] = [
        // Opening
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
        "rnbqkb1r/pp2pppp/2p2n2/3p4/2PP4/4P3/PP3PPP/RNBQKBNR w KQkq - 1 4",
        "2kr1b1r/ppp1qppp/2n5/3p1b2/2PPn3/5N2/PP2BPPP/RNBQR1K1 w - - 1 10",
        "rnbqk2r/pppp1ppp/8/2b5/2P1P1n1/3pNN2/PP3PPP/R1BQKB1R b KQkq - 1 7",
        "r1bq1rk1/pp1nppbp/2p3p1/8/3PB3/2P2N2/PP2QPPP/R1B2RK1 b - - 6 10",
        "rnbq1rk1/ppn2ppp/4p3/2p5/3PP3/P1P2P2/2Q1N1PP/R1B1KB1R b KQ - 2 10",
        "r1bqkb1r/pp1nnp1p/4p1p1/2ppP2P/3P4/2PB1N2/PP3PP1/RNBQ1RK1 b kq - 0 9",
        // Midgame
        "5rk1/2b1qp1p/1r2p1pB/1ppnn3/3pN3/1P1P2P1/2P1QPBP/R4RK1 b - - 7 22",
        "2k4r/1p3pp1/p2p2n1/2P1p2q/P1P1P3/3PBPP1/2R3Qr/5RK1 b - - 2 22",
        "r6k/p1B4p/Pp3rp1/3p4/2nP4/2PQ1PPq/7P/1R3RK1 b - - 0 32",
        "r3kb2/p4pp1/2q1p3/1pP1n1N1/3B2nr/1QP1P3/PP1N3P/R2R2K1 w q b6 0 2",
        "rn1qkbnr/pp3ppp/4p3/3pPb2/1PpP4/4BN2/P1P1BPPP/RN1QK2R b KQkq b3 0 2",
        "rnbqkbnr/pp1p1ppp/8/2pPp3/8/8/PPP1PPPP/RNBQKBNR w KQkq e6 0 2",
        "r3k2r/1p1n4/1p1b1p2/2pp2p1/P2P2p1/1P1NP2P/R2BKP2/6R1 w kq - 0 21",
        "1k1r4/p1p5/Q2b1pp1/R2P4/b3P2P/P7/1P5P/6K1 w - - 1 34",
        "4rrk1/pppb1p1p/3pq3/4nNp1/2P1P3/3BP3/PP1Q1RPP/5RK1 w - - 4 18",
        "4rrk1/1q3pbp/1p4p1/p3p3/2P1bn2/2B1QN1P/PP3PP1/3RR1K1 w - - 0 26",
        "r5k1/ppq2rpp/2b5/4n3/3QPN2/P5P1/6BP/1R3RK1 b - - 3 22",
        "r3kn1r/1p3pbp/p5n1/q2pPRNQ/3P4/1P6/P3N1P1/R1B3K1 b kq - 0 19",
        "7r/4b1k1/N1nq1rp1/1p1p1p2/4pP1P/P1P1B2R/1P2Q1P1/3R2K1 b - - 2 39",
        "3r4/2q1b1k1/p1np1rp1/1p1Qpp2/5P2/PNP1B1R1/1P4PP/3R2K1 w - - 1 26",
        // Endgame
        "8/8/6Q1/8/6k1/1P2q3/7p/7K b - - 14 75",
        "8/8/4nPk1/8/6pK/8/1R3P1P/2B3r1 b - - 1 54",
        "8/7q/5K2/2q5/6k1/8/8/8 b - - 5 60",
        "8/8/5k2/bp1r4/4R1P1/3pK3/3N1P2/8 b - - 11 59",
        "8/2pkr1Q1/6p1/3P1p2/R6P/P7/1P5P/6K1 w - - 3 42",
        "8/2p2k2/3p4/p1rPn3/6pQ/1K6/8/8 w - - 2 119",
        "8/7N/8/2n5/p3p1Pp/3k1p1P/5P2/6K1 b - - 1 54",
        "8/6p1/1p5p/5PkP/6P1/6K1/p7/N6b b - - 1 48",
        "8/1p6/pP3p2/P2k1P2/6Kp/8/8/8 b - - 1 73",
    ];

    let mut benchmark_result = BenchmarkResult::default();
    let benchmark_time_start = SystemTime::now();

    for (current_position_index, fen) in BENCHMARK_POSITIONS.into_iter().enumerate() {
        println!("{}/{}. {}", current_position_index + 1, BENCHMARK_POSITIONS.len(), fen);

        let ttable = Arc::new(TTable::new(64 * 1024 * 1024));
        let phtable = Arc::new(PHTable::new(2 * 1024 * 1024));
        let abort_flag = Arc::new(AtomicBool::new(false));
        let ponder_flag = Arc::new(AtomicBool::new(false));

        let board = Board::new_from_fen(fen).unwrap();
        let mut context = SearchContext::new(board, ttable.clone(), phtable.clone(), abort_flag.clone(), ponder_flag.clone());
        context.forced_depth = 16;

        context.by_ref().last().unwrap();

        benchmark_result.nodes_count += context.stats.nodes_count;
        benchmark_result.q_nodes_count += context.stats.q_nodes_count;
        benchmark_result.leafs_count += context.stats.leafs_count;
        benchmark_result.q_leafs_count += context.stats.q_leafs_count;

        #[cfg(feature = "dev")]
        {
            benchmark_result.beta_cutoffs += context.stats.beta_cutoffs;
            benchmark_result.q_beta_cutoffs += context.stats.q_beta_cutoffs;

            benchmark_result.perfect_cutoffs += context.stats.perfect_cutoffs;
            benchmark_result.q_perfect_cutoffs += context.stats.q_perfect_cutoffs;
            benchmark_result.non_perfect_cutoffs += context.stats.non_perfect_cutoffs;
            benchmark_result.q_non_perfect_cutoffs += context.stats.q_non_perfect_cutoffs;

            benchmark_result.pvs_full_window_searches += context.stats.pvs_full_window_searches;
            benchmark_result.pvs_zero_window_searches += context.stats.pvs_zero_window_searches;
            benchmark_result.pvs_rejected_searches += context.stats.pvs_rejected_searches;

            benchmark_result.snmp_attempts += context.stats.snmp_attempts;
            benchmark_result.snmp_accepted += context.stats.snmp_accepted;
            benchmark_result.snmp_rejected += context.stats.snmp_rejected;

            benchmark_result.nmp_attempts += context.stats.nmp_attempts;
            benchmark_result.nmp_accepted += context.stats.nmp_accepted;
            benchmark_result.nmp_rejected += context.stats.nmp_rejected;

            benchmark_result.lmp_accepted += context.stats.lmp_accepted;
            benchmark_result.lmp_rejected += context.stats.lmp_rejected;

            benchmark_result.razoring_attempts += context.stats.razoring_attempts;
            benchmark_result.razoring_accepted += context.stats.razoring_accepted;
            benchmark_result.razoring_rejected += context.stats.razoring_rejected;

            benchmark_result.q_score_pruning_accepted += context.stats.q_score_pruning_accepted;
            benchmark_result.q_score_pruning_rejected += context.stats.q_score_pruning_rejected;

            benchmark_result.q_futility_pruning_accepted += context.stats.q_futility_pruning_accepted;
            benchmark_result.q_futility_pruning_rejected += context.stats.q_futility_pruning_rejected;

            benchmark_result.tt_added += context.stats.tt_added;
            benchmark_result.tt_hits += context.stats.tt_hits;
            benchmark_result.tt_misses += context.stats.tt_misses;

            benchmark_result.tt_legal_hashmoves += context.stats.tt_legal_hashmoves;
            benchmark_result.tt_illegal_hashmoves += context.stats.tt_illegal_hashmoves;
            benchmark_result.ktable_legal_moves += context.stats.ktable_legal_moves;
            benchmark_result.ktable_illegal_moves += context.stats.ktable_illegal_moves;
            benchmark_result.cmtable_legal_moves += context.stats.cmtable_legal_moves;
            benchmark_result.cmtable_illegal_moves += context.stats.cmtable_illegal_moves;

            benchmark_result.phtable_added += context.stats.phtable_added;
            benchmark_result.phtable_hits += context.stats.phtable_hits;
            benchmark_result.phtable_misses += context.stats.phtable_misses;

            benchmark_result.movegen_hash_move_stages += context.stats.movegen_hash_move_stages;
            benchmark_result.movegen_captures_stages += context.stats.movegen_captures_stages;
            benchmark_result.movegen_killers_stages += context.stats.movegen_killers_stages;
            benchmark_result.movegen_counters_stages += context.stats.movegen_counters_stages;
            benchmark_result.movegen_quiets_stages += context.stats.movegen_quiets_stages;
        }

        benchmark_result.result_hash ^= context.lines[0].pv_line[0].data;
    }

    benchmark_result.time = (benchmark_time_start.elapsed().unwrap().as_millis() as f32) / 1000.0;
    benchmark_result
}
