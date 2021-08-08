use crate::board::representation::Bitboard;
use crate::engine::search;
use chrono::Utc;

pub struct BenchmarkResult {
    pub time: f32,

    pub nodes_count: u64,
    pub q_nodes_count: u64,
    pub leafs_count: u64,
    pub q_leafs_count: u64,
    pub beta_cutoffs: u64,
    pub q_beta_cutoffs: u64,

    pub perfect_cutoffs: u64,
    pub q_perfect_cutoffs: u64,
    pub non_perfect_cutoffs: u64,
    pub q_non_perfect_cutoffs: u64,
}

impl BenchmarkResult {
    pub fn new() -> BenchmarkResult {
        BenchmarkResult {
            time: 0.0,

            nodes_count: 0,
            q_nodes_count: 0,
            leafs_count: 0,
            q_leafs_count: 0,
            beta_cutoffs: 0,
            q_beta_cutoffs: 0,

            perfect_cutoffs: 0,
            q_perfect_cutoffs: 0,
            non_perfect_cutoffs: 0,
            q_non_perfect_cutoffs: 0,
        }
    }
}

impl Default for BenchmarkResult {
    fn default() -> Self {
        Self::new()
    }
}

pub fn run() -> BenchmarkResult {
    let benchmark_positions = [
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
        "5rk1/2b1qp1p/1r2p1pB/1ppnn3/3pN3/1P1P2P1/2P1QPBP/R4RK1 b - - 7 22",
        "2k4r/1p3pp1/p2p2n1/2P1p2q/P1P1P3/3PBPP1/2R3Qr/5RK1 b - - 2 22",
        "r6k/p1B4p/Pp3rp1/3p4/2nP4/2PQ1PPq/7P/1R3RK1 b - - 0 32",
        "r3kb2/p4pp1/2q1p3/1pP1n1N1/3B2nr/1QP1P3/PP1N3P/R2R2K1 w q b6 0 2",
        "rn1qkbnr/pp3ppp/4p3/3pPb2/1PpP4/4BN2/P1P1BPPP/RN1QK2R b KQkq b3 0 2",
        "rnbqkbnr/pp1p1ppp/8/2pPp3/8/8/PPP1PPPP/RNBQKBNR w KQkq e6 0 2",
        "8/8/6Q1/8/6k1/1P2q3/7p/7K b - - 14 75",
        "8/8/4nPk1/8/6pK/8/1R3P1P/2B3r1 b - - 1 54",
        "8/7q/5K2/2q5/6k1/8/8/8 b - - 5 60",
    ];

    let mut benchmark_result = BenchmarkResult::new();
    let benchmark_time_start = Utc::now();

    for fen in benchmark_positions {
        let mut board = Bitboard::new_from_fen(fen).unwrap();
        let result = search::run_fixed_depth(&mut board, 6);

        benchmark_result.nodes_count += result.statistics.nodes_count;
        benchmark_result.q_nodes_count += result.statistics.q_nodes_count;
        benchmark_result.leafs_count += result.statistics.leafs_count;
        benchmark_result.q_leafs_count += result.statistics.q_leafs_count;
        benchmark_result.beta_cutoffs += result.statistics.beta_cutoffs;
        benchmark_result.q_beta_cutoffs += result.statistics.q_beta_cutoffs;

        benchmark_result.perfect_cutoffs += result.statistics.perfect_cutoffs;
        benchmark_result.q_perfect_cutoffs += result.statistics.q_perfect_cutoffs;
        benchmark_result.non_perfect_cutoffs += result.statistics.non_perfect_cutoffs;
        benchmark_result.q_non_perfect_cutoffs += result.statistics.q_non_perfect_cutoffs;
    }

    benchmark_result.time = ((Utc::now() - benchmark_time_start).num_milliseconds() as f32) / 1000.0;
    benchmark_result
}