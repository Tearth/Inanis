use crate::cache::pawns::PawnHashTable;
use crate::cache::search::TranspositionTable;
use crate::engine::context::SearchContext;
use crate::state::board::Bitboard;
use crate::state::fen::*;
use crate::state::movescan::Move;
use chrono::Utc;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

struct TestPosition {
    id: String,
    board: Bitboard,
    best_move: Move,
}

impl TestPosition {
    pub fn new(id: String, board: Bitboard, best_move: Move) -> TestPosition {
        TestPosition { id, board, best_move }
    }
}

pub fn run(epd_filename: &str, depth: i8, tries_to_confirm: i8) {
    println!("Loading EPD file...");
    let positions = match load_positions(epd_filename) {
        Ok(value) => value,
        Err(message) => {
            println!("{}", message);
            return;
        }
    };
    println!("Loaded {} positions, starting test", positions.len());

    let mut passed_tests = 0;
    let mut failed_tests = 0;
    let mut recognition_depths_sum = 0;
    let start_time = Utc::now();

    for position in &positions {
        let mut transposition_table = TranspositionTable::new(64 * 1024 * 1024);
        let mut pawn_hashtable = PawnHashTable::new(1 * 1024 * 1024);
        let mut killers_table = Default::default();
        let mut history_table = Default::default();
        let mut abort_token = Default::default();

        let mut board_clone = position.board.clone();
        let context = SearchContext::new(
            &mut board_clone,
            0,
            0,
            depth,
            0,
            0,
            0,
            &mut transposition_table,
            &mut pawn_hashtable,
            &mut killers_table,
            &mut history_table,
            &mut abort_token,
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

        if last_best_move == position.best_move {
            println!("Test {} PASSED (depth: {})", position.id, recognition_depth);
            recognition_depths_sum += recognition_depth as u32;
            passed_tests += 1;
        } else {
            println!(
                "Test {} FAILED (expected {}, got {})",
                position.id,
                position.best_move.to_long_notation(),
                last_best_move.to_long_notation()
            );
            failed_tests += 1;
        }
    }

    println!("-----------------------------------------------------------------------------");
    println!(
        "Tests done in {:.2} s: {} passed ({:.2}% with average depth {:.2}), {} failed",
        ((Utc::now() - start_time).num_milliseconds() as f32) / 1000.0,
        passed_tests,
        (passed_tests as f32) / (positions.len() as f32) * 100.0,
        (recognition_depths_sum as f32) / (passed_tests as f32),
        failed_tests
    );
}

fn load_positions(epd_filename: &str) -> Result<Vec<TestPosition>, &'static str> {
    let mut positions = Vec::new();
    let file = match File::open(epd_filename) {
        Ok(value) => value,
        Err(_) => return Err("Can't open EPD file"),
    };

    for line in BufReader::new(file).lines() {
        let position = line.unwrap();
        let parsed_epd = epd_to_board(position.as_str())?;

        if parsed_epd.id == None {
            return Err("Not enough data");
        }

        let parsed_best_move = Move::from_short_notation(&parsed_epd.best_move.unwrap(), &parsed_epd.board)?;
        positions.push(TestPosition::new(parsed_epd.id.unwrap(), parsed_epd.board, parsed_best_move));
    }

    Ok(positions)
}
