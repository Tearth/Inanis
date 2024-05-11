use crate::cache::history::HistoryTable;
use crate::cache::killers::KillersTable;
use crate::cache::pawns::PawnHashTable;
use crate::cache::search::TranspositionTable;
use crate::engine::context::SearchContext;
use crate::engine::see::SEEContainer;
use crate::evaluation::EvaluationParameters;
use crate::state::movegen::MagicContainer;
use crate::state::movescan::Move;
use crate::state::patterns::PatternsContainer;
use crate::state::representation::Board;
use crate::state::text::fen;
use crate::state::zobrist::ZobristContainer;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::thread;
use std::time::SystemTime;

pub struct TestContext {
    positions: Vec<TestPosition>,
}

pub struct TestPosition {
    id: String,
    board: Board,
    best_move: Move,
}

impl TestContext {
    /// Constructs a new instance of [TestContext] with stored `positions`.
    pub fn new(positions: Vec<TestPosition>) -> Self {
        Self { positions }
    }
}

impl TestPosition {
    /// Constructs a new instance of [TestPosition] with stored `id`, `board` and `best_move`.
    pub fn new(id: String, board: Board, best_move: Move) -> Self {
        Self { id, board, best_move }
    }
}

/// Runs a test by performing a fixed-`depth` search for the positions loaded from the `epd_filename` file, using hashtable with
/// size specified in `transposition_table_size`. To classify the test as successful, the last iteration has to return the correct best move.
pub fn run(epd_filename: &str, depth: i8, transposition_table_size: usize, threads_count: usize) {
    println!("Loading EPD file...");
    let positions = match load_positions(epd_filename) {
        Ok(value) => value,
        Err(error) => {
            println!("Invalid PGN: {}", error);
            return;
        }
    };
    println!("Loaded {} positions, starting test", positions.len());

    let mut context = TestContext::new(positions);
    run_internal(&mut context, depth, transposition_table_size, threads_count);
}

/// Internal test function, called by [run].
fn run_internal(context: &mut TestContext, depth: i8, transposition_table_size: usize, threads_count: usize) {
    let index = Arc::new(AtomicU32::new(0));
    let passed_tests = Arc::new(AtomicU32::new(0));
    let failed_tests = Arc::new(AtomicU32::new(0));
    let recognition_depths_sum = Arc::new(AtomicU32::new(0));
    let start_time = SystemTime::now();

    let positions_count = context.positions.len();

    thread::scope(|scope| {
        for chunk in context.positions.chunks_mut(positions_count / threads_count) {
            let index_arc = index.clone();
            let passed_tests_arc = passed_tests.clone();
            let failed_tests_arc = failed_tests.clone();
            let recognition_depths_sum_arc = recognition_depths_sum.clone();

            scope.spawn(move || {
                for position in chunk {
                    let transposition_table = Arc::new(TranspositionTable::new(transposition_table_size));
                    let pawn_hashtable = Arc::new(PawnHashTable::new(1 * 1024 * 1024));
                    let killers_table = Arc::new(KillersTable::default());
                    let history_table = Arc::new(HistoryTable::default());
                    let abort_flag = Arc::new(AtomicBool::new(false));
                    let ponder_flag = Arc::new(AtomicBool::new(false));

                    let board_clone = position.board.clone();
                    let context = SearchContext::new(
                        board_clone,
                        Default::default(),
                        0,
                        0,
                        0,
                        depth,
                        0,
                        0,
                        0,
                        Vec::new(),
                        false,
                        false,
                        false,
                        false,
                        false,
                        false,
                        0,
                        0,
                        transposition_table.clone(),
                        pawn_hashtable.clone(),
                        killers_table.clone(),
                        history_table.clone(),
                        abort_flag.clone(),
                        ponder_flag.clone(),
                    );

                    let mut last_best_move = Default::default();
                    let mut best_moves_count = 0;
                    let mut recognition_depth = 0;

                    for result in context {
                        last_best_move = result.lines[0].pv_line[0];
                        if last_best_move == position.best_move {
                            if best_moves_count == 0 {
                                recognition_depth = result.depth;
                            }

                            best_moves_count += 1;
                        } else {
                            best_moves_count = 0;
                        }
                    }

                    let index_to_display = index_arc.fetch_add(1, Ordering::Relaxed);
                    if last_best_move == position.best_move {
                        println!("{}/{}. Test {} PASSED (depth: {})", index_to_display + 1, positions_count, position.id, recognition_depth);
                        recognition_depths_sum_arc.fetch_add(recognition_depth as u32, Ordering::Relaxed);
                        passed_tests_arc.fetch_add(1, Ordering::Relaxed);
                    } else {
                        println!(
                            "{}/{}. Test {} FAILED (expected {}, got {})",
                            index_to_display + 1,
                            positions_count,
                            position.id,
                            position.best_move,
                            last_best_move
                        );
                        failed_tests_arc.fetch_add(1, Ordering::Relaxed);
                    }
                }
            });
        }
    });

    println!("-----------------------------------------------------------------------------");
    println!(
        "Tests done in {:.2} s: {} passed ({:.2}% with average depth {:.2}), {} failed",
        (start_time.elapsed().unwrap().as_millis() as f32) / 1000.0,
        passed_tests.load(Ordering::Relaxed),
        (passed_tests.load(Ordering::Relaxed) as f32) / (positions_count as f32) * 100.0,
        (recognition_depths_sum.load(Ordering::Relaxed) as f32) / (passed_tests.load(Ordering::Relaxed) as f32),
        failed_tests.load(Ordering::Relaxed)
    );
}

/// Loads positions from the `epd_filename` and parses them into a list of [TestPosition]. Returns [Err] with a proper error message if the
/// file couldn't be parsed.
fn load_positions(epd_filename: &str) -> Result<Vec<TestPosition>, String> {
    let mut positions = Vec::new();
    let file = match File::open(epd_filename) {
        Ok(value) => value,
        Err(error) => return Err(format!("Invalid EPD file: {}", error)),
    };

    let evaluation_parameters = Arc::new(EvaluationParameters::default());
    let zobrist_container = Arc::new(ZobristContainer::default());
    let patterns_container = Arc::new(PatternsContainer::default());
    let see_container = Arc::new(SEEContainer::new(Some(evaluation_parameters.clone())));
    let magic_container = Arc::new(MagicContainer::default());

    for line in BufReader::new(file).lines() {
        let position = line.unwrap();
        if position.is_empty() {
            continue;
        }

        let mut parsed_epd = fen::epd_to_board(
            position.as_str(),
            Some(evaluation_parameters.clone()),
            Some(zobrist_container.clone()),
            Some(patterns_container.clone()),
            Some(see_container.clone()),
            Some(magic_container.clone()),
        )?;

        let parsed_best_move = Move::from_short_notation(&parsed_epd.best_move.unwrap(), &mut parsed_epd.board)?;
        positions.push(TestPosition::new(parsed_epd.id.unwrap(), parsed_epd.board, parsed_best_move));
    }

    Ok(positions)
}
