use crate::cache::pawns::PawnHashTable;
use crate::cache::search::TranspositionTable;
use crate::engine::context::SearchContext;
use crate::state::board::Bitboard;
use crate::state::fen;
use crate::state::movescan::Move;
use chrono::Utc;
use std::cell::UnsafeCell;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::thread;

struct TestContext {
    positions: UnsafeCell<Vec<TestPosition>>,
}

struct TestPosition {
    id: String,
    board: Bitboard,
    best_move: Move,
}

impl TestContext {
    /// Constructs a new instance of [TestContext] with stored `positions`.
    pub fn new(positions: UnsafeCell<Vec<TestPosition>>) -> Self {
        Self { positions }
    }
}

unsafe impl Sync for TestContext {}

impl TestPosition {
    /// Constructs a new instance of [TestPosition] with stored `id`, `board` and `best_move`.
    pub fn new(id: String, board: Bitboard, best_move: Move) -> Self {
        Self { id, board, best_move }
    }
}

/// Runs a test by performing a fixed-`depth` search for the positions loaded from the `epd_filename` file. To classify the test as successful,
/// the last iteration has to return the correct move, or there must be at least `tries_to_confirm` search iterations in a row which returned
/// the best move same as the expected one in the position.
pub fn run(epd_filename: &str, depth: i8, tries_to_confirm: i8, threads_count: usize) {
    println!("Loading EPD file...");
    let positions = match load_positions(epd_filename) {
        Ok(value) => value,
        Err(message) => {
            println!("{}", message);
            return;
        }
    };
    println!("Loaded {} positions, starting test", unsafe { (*positions.get()).len() });

    let context = Arc::new(TestContext::new(positions));
    run_internal(&context, depth, tries_to_confirm, threads_count);
}

/// Internal test function, called by [run].
fn run_internal(context: &Arc<TestContext>, depth: i8, tries_to_confirm: i8, threads_count: usize) {
    unsafe {
        let index = Arc::new(AtomicU32::new(0));
        let passed_tests = Arc::new(AtomicU32::new(0));
        let failed_tests = Arc::new(AtomicU32::new(0));
        let recognition_depths_sum = Arc::new(AtomicU32::new(0));
        let start_time = Utc::now();

        let mut threads = Vec::new();
        let positions_count = (*context.positions.get()).len();

        for thread_index in 0..threads_count {
            let index_arc = index.clone();
            let context_arc = context.clone();
            let passed_tests_arc = passed_tests.clone();
            let failed_tests_arc = failed_tests.clone();
            let recognition_depths_sum_arc = recognition_depths_sum.clone();

            threads.push(thread::spawn(move || {
                let from = thread_index * (positions_count / threads_count);
                let mut to = (thread_index + 1) * (positions_count / threads_count);

                // Add rest of the positions which didn't fit in the last thread
                if to + (positions_count % threads_count) == positions_count {
                    to = positions_count;
                }

                for position in &mut (*context_arc.positions.get())[from..to] {
                    let mut transposition_table = TranspositionTable::new(64 * 1024 * 1024);
                    let mut pawn_hashtable = PawnHashTable::new(1 * 1024 * 1024);
                    let mut killers_table = Default::default();
                    let mut history_table = Default::default();
                    let mut abort_token = Default::default();
                    let mut ponder_token = Default::default();

                    let mut board_clone = position.board.clone();
                    let context = SearchContext::new(
                        &mut board_clone,
                        0,
                        0,
                        depth,
                        0,
                        0,
                        0,
                        false,
                        false,
                        &mut transposition_table,
                        &mut pawn_hashtable,
                        &mut killers_table,
                        &mut history_table,
                        &mut abort_token,
                        &mut ponder_token,
                    );

                    let mut last_best_move = Default::default();
                    let mut best_moves_count = 0;
                    let mut recognition_depth = 0;
                    for result in context {
                        last_best_move = result.pv_line[0];
                        if last_best_move == position.best_move {
                            if best_moves_count == 0 {
                                recognition_depth = result.depth;
                            }

                            best_moves_count += 1;
                        } else {
                            best_moves_count = 0;
                        }

                        if best_moves_count >= tries_to_confirm {
                            break;
                        }
                    }

                    let index = index_arc.fetch_add(1, Ordering::Relaxed);
                    if last_best_move == position.best_move {
                        println!("{}/{}. Test {} PASSED (depth: {})", index + 1, positions_count, position.id, recognition_depth);
                        recognition_depths_sum_arc.fetch_add(recognition_depth as u32, Ordering::Relaxed);
                        passed_tests_arc.fetch_add(1, Ordering::Relaxed);
                    } else {
                        println!(
                            "{}/{}. Test {} FAILED (expected {}, got {})",
                            index + 1,
                            positions_count,
                            position.id,
                            position.best_move.to_long_notation(),
                            last_best_move.to_long_notation()
                        );
                        failed_tests_arc.fetch_add(1, Ordering::Relaxed);
                    }
                }
            }));
        }

        for thread in threads {
            thread.join().unwrap();
        }

        println!("-----------------------------------------------------------------------------");
        println!(
            "Tests done in {:.2} s: {} passed ({:.2}% with average depth {:.2}), {} failed",
            ((Utc::now() - start_time).num_milliseconds() as f32) / 1000.0,
            passed_tests.load(Ordering::Relaxed),
            (passed_tests.load(Ordering::Relaxed) as f32) / (positions_count as f32) * 100.0,
            (recognition_depths_sum.load(Ordering::Relaxed) as f32) / (passed_tests.load(Ordering::Relaxed) as f32),
            failed_tests.load(Ordering::Relaxed)
        );
    }
}

/// Loads positions from the `epd_filename` and parses them into a list of [TestPosition]. Returns [Err] with a proper error message if the
/// file couldn't be parsed.
fn load_positions(epd_filename: &str) -> Result<UnsafeCell<Vec<TestPosition>>, &'static str> {
    let mut positions = Vec::new();
    let file = match File::open(epd_filename) {
        Ok(value) => value,
        Err(_) => return Err("Can't open EPD file"),
    };

    for line in BufReader::new(file).lines() {
        let position = line.unwrap();
        if position.is_empty() {
            continue;
        }

        let parsed_epd = fen::epd_to_board(position.as_str())?;
        if parsed_epd.id == None {
            return Err("Not enough data");
        }

        let parsed_best_move = Move::from_short_notation(&parsed_epd.best_move.unwrap(), &parsed_epd.board)?;
        positions.push(TestPosition::new(parsed_epd.id.unwrap(), parsed_epd.board, parsed_best_move));
    }

    Ok(UnsafeCell::new(positions))
}
