use crate::cache::pawns::PHTable;
use crate::cache::search::TTable;
use crate::engine::context::SearchContext;
use crate::state::movescan::Move;
use crate::state::representation::Board;
use crate::state::text::fen;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::thread;
use std::time::SystemTime;

pub struct TestPosition {
    id: String,
    board: Board,
    best_move: Move,
}

impl TestPosition {
    /// Constructs a new instance of [TestPosition] with stored `id`, `board` and `best_move`.
    pub fn new(id: String, board: Board, best_move: Move) -> Self {
        Self { id, board, best_move }
    }
}

/// Runs a test by performing a fixed-`depth` search for the positions loaded from the `epd_filename` file, using hashtable with
/// size specified in `ttable_size`. To classify the test as successful, the last iteration has to return the correct best move.
/// Multithreading is supported by `threads_count`.
pub fn run(epd_filename: &str, depth: i8, ttable_size: usize, threads_count: usize) {
    println!("Loading EPD file...");
    let mut positions = match load_positions(epd_filename) {
        Ok(value) => value,
        Err(error) => {
            println!("Invalid PGN: {}", error);
            return;
        }
    };
    println!("Loaded {} positions, starting test", positions.len());

    let index = Arc::new(AtomicU32::new(0));
    let passed_tests = Arc::new(AtomicU32::new(0));
    let failed_tests = Arc::new(AtomicU32::new(0));
    let recognition_depths_sum = Arc::new(AtomicU32::new(0));
    let start_time = SystemTime::now();
    let positions_count = positions.len();

    thread::scope(|scope| {
        for chunk in positions.chunks_mut(positions_count / threads_count) {
            let index_arc = index.clone();
            let passed_tests_arc = passed_tests.clone();
            let failed_tests_arc = failed_tests.clone();
            let recognition_depths_sum_arc = recognition_depths_sum.clone();

            scope.spawn(move || {
                for position in chunk {
                    let ttable = Arc::new(TTable::new(ttable_size));
                    let phtable = Arc::new(PHTable::new(1 * 1024 * 1024));
                    let abort_flag = Arc::new(AtomicBool::new(false));
                    let ponder_flag = Arc::new(AtomicBool::new(false));

                    let board_clone = position.board.clone();
                    let mut context = SearchContext::new(board_clone, ttable, phtable, abort_flag, ponder_flag);
                    context.forced_depth = depth;

                    let mut last_best_move = Move::default();
                    let mut best_moves_count = 0;
                    let mut recognition_depth = 0;

                    while let Some(result) = context.next() {
                        last_best_move = context.lines[0].pv_line[0];
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

/// Loads positions from the `epd_filename` and parses them into a list of [TestPosition].
/// Returns [Err] with a proper error message if the file couldn't be parsed.
fn load_positions(epd_filename: &str) -> Result<Vec<TestPosition>, String> {
    let mut positions = Vec::new();
    let file = match File::open(epd_filename) {
        Ok(value) => value,
        Err(error) => return Err(format!("Invalid EPD file: {}", error)),
    };

    for line in BufReader::new(file).lines() {
        let position = line.unwrap();
        if position.is_empty() {
            continue;
        }

        let mut parsed_epd = fen::epd_to_board(position.as_str())?;
        let parsed_best_move = Move::from_short_notation(&parsed_epd.best_move.unwrap(), &mut parsed_epd.board)?;
        positions.push(TestPosition::new(parsed_epd.id.unwrap(), parsed_epd.board, parsed_best_move));
    }

    Ok(positions)
}
