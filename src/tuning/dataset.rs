use crate::cache::counters::CountermovesTable;
use crate::cache::history::HistoryTable;
use crate::cache::killers::KillersTable;
use crate::cache::pawns::PawnHashTable;
use crate::cache::search::TranspositionTable;
use crate::engine::context::SearchContext;
use crate::engine::qsearch;
use crate::engine::see::SEEContainer;
use crate::engine::*;
use crate::evaluation::material;
use crate::evaluation::*;
use crate::state::movegen::MagicContainer;
use crate::state::patterns::PatternsContainer;
use crate::state::representation::Board;
use crate::state::text::pgn::PGNLoader;
use crate::state::zobrist::ZobristContainer;
use crate::utils::rand;
use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::LineWriter;
use std::io::Write;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::time::SystemTime;

/// Runs generator of the dataset for the tuner. It works by parsing `pgn_filename`, and then picking random positions based on the
/// provided restrictions like `min_ply`, `max_score`, `max_differ` and `density`. Output positions are then stored in the `output_file`.
pub fn run(pgn_filename: &str, output_file: &str, min_ply: usize, max_score: i16, max_diff: u16, density: usize) {
    println!("Loading PGN file...");

    let start_time = SystemTime::now();
    let file = match File::open(pgn_filename) {
        Ok(value) => value,
        Err(error) => {
            println!("Invalid PGN file: {}", error);
            return;
        }
    };

    let pgn_loader = PGNLoader::new(BufReader::new(file).lines());
    let mut output_positions = HashSet::new();
    let mut parsed_pgns = 0;

    let zobrist_container = Arc::new(ZobristContainer::default());
    let patterns_container = Arc::new(PatternsContainer::default());
    let see_container = Arc::new(SEEContainer::default());
    let magic_container = Arc::new(MagicContainer::default());

    let transposition_table = Arc::new(TranspositionTable::new(1 * 1024 * 1024));
    let pawn_hashtable = Arc::new(PawnHashTable::new(1 * 1024 * 1024));
    let abort_flag = Arc::new(AtomicBool::new(false));
    let ponder_flag = Arc::new(AtomicBool::new(false));

    let mut total_viable_positions = 0;
    let mut ignored_positions = 0;
    let mut duplicates = 0;
    let mut sum_of_game_phases = 0.0;

    for pgn in pgn_loader {
        let pgn = match pgn {
            Ok(value) => value,
            Err(error) => {
                println!("Invalid PGN file: {}", error);
                return;
            }
        };

        if pgn.result == "*" {
            continue;
        }

        let board = match pgn.fen {
            Some(fen) => {
                let fen_result = Board::new_from_fen(
                    &fen,
                    Some(zobrist_container.clone()),
                    Some(patterns_container.clone()),
                    Some(see_container.clone()),
                    Some(magic_container.clone()),
                );

                match fen_result {
                    Ok(board) => board,
                    Err(error) => {
                        println!("Invalid PGN file: {}", error);
                        return;
                    }
                }
            }

            None => Board::new_initial_position(
                Some(zobrist_container.clone()),
                Some(patterns_container.clone()),
                Some(see_container.clone()),
                Some(magic_container.clone()),
            ),
        };

        let mut context = SearchContext::new(
            board,
            Default::default(),
            transposition_table.clone(),
            pawn_hashtable.clone(),
            KillersTable::default(),
            HistoryTable::default(),
            CountermovesTable::default(),
            abort_flag.clone(),
            ponder_flag.clone(),
        );

        let mut viable_positions = Vec::new();

        for (index, data) in pgn.data.iter().enumerate() {
            context.board.make_move(data.r#move);

            if index < min_ply {
                ignored_positions += 1;
                continue;
            }

            if data.r#move.is_capture() || data.r#move.is_castling() || data.r#move.is_promotion() {
                ignored_positions += 1;
                continue;
            }

            if context.board.is_king_checked(context.board.active_color) {
                ignored_positions += 1;
                continue;
            }

            let material_evaluation = material::evaluate(&context.board);
            if material_evaluation.taper_score(context.board.game_phase).abs() > max_score {
                ignored_positions += 1;
                continue;
            }

            let score = context.board.evaluate_without_cache(context.board.active_color);
            let q_score = qsearch::run(&mut context, 0, MIN_ALPHA, MIN_BETA);

            if score.abs_diff(q_score) > max_diff {
                ignored_positions += 1;
                continue;
            }

            let epd = format!("{} c9 \"{:.2}|{}\";", context.board.to_epd(), data.evaluation, pgn.result);
            let game_phase = (context.board.game_phase as f32) / (INITIAL_GAME_PHASE as f32);

            viable_positions.push((epd, game_phase));
            total_viable_positions += 1;
        }

        let mut picked_positions = 0;

        while picked_positions < density {
            if viable_positions.is_empty() {
                break;
            }

            let index = rand::usize(0..viable_positions.len());
            let (position, game_phase) = viable_positions[index].to_owned();

            if output_positions.contains(&position) {
                viable_positions.remove(index);
                duplicates += 1;

                continue;
            }

            output_positions.insert(position);
            viable_positions.remove(index);
            picked_positions += 1;
            sum_of_game_phases += game_phase;
        }

        parsed_pgns += 1;

        if parsed_pgns % 1000 == 0 {
            println!(
                "Parsed PGNs: {} ({} viable positions, {} ignored positions, {} output positions, {} duplicates)",
                parsed_pgns,
                total_viable_positions,
                ignored_positions,
                output_positions.len(),
                duplicates
            );
        }
    }

    println!("-----------------------------------------------------------------------------");
    println!("Saving output...");

    let output_file = match File::create(output_file) {
        Ok(value) => value,
        Err(error) => {
            println!("Error while saving output: {}", error);
            return;
        }
    };
    let mut output_file_line_writer = LineWriter::new(output_file);
    let positions_count = output_positions.len();

    for fen in output_positions {
        output_file_line_writer.write_all((fen + "\n").as_bytes()).unwrap();
    }

    println!(
        "Tuner dataset generation done in {:.2} s, average game phase: {:.2}",
        (start_time.elapsed().unwrap().as_millis() as f32) / 1000.0,
        sum_of_game_phases / (positions_count as f32)
    );
}
