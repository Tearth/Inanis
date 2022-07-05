use crate::cache::pawns::PawnHashTable;
use crate::cache::search::TranspositionTable;
use crate::engine::context::SearchContext;
use crate::engine::history::HistoryTable;
use crate::engine::killers::KillersTable;
use crate::engine::qsearch;
use crate::engine::see::SEEContainer;
use crate::engine::*;
use crate::evaluation::EvaluationParameters;
use crate::state::board::Bitboard;
use crate::state::movegen::MagicContainer;
use crate::state::patterns::PatternsContainer;
use crate::state::zobrist::ZobristContainer;
use crate::utils::pgn::PGNLoader;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::LineWriter;
use std::io::Write;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

pub fn run(pgn_filename: &str, output_file: &str, min_ply: usize, max_score: i16, max_diff: u16, density: usize) {
    println!("Loading PGN file...");

    let file = match File::open(pgn_filename) {
        Ok(value) => value,
        Err(_) => {
            println!("Can't open PGN file");
            return;
        }
    };

    let pgn_loader = PGNLoader::new(BufReader::new(file).lines());
    let mut output_positions = Vec::new();
    let mut parsed_pgns = 0;

    let evaluation_parameters = Arc::new(EvaluationParameters::default());
    let zobrist_container = Arc::new(ZobristContainer::default());
    let patterns_container = Arc::new(PatternsContainer::default());
    let see_container = Arc::new(SEEContainer::new(Some(evaluation_parameters.clone())));
    let magic_container = Arc::new(MagicContainer::default());

    let transposition_table = Arc::new(TranspositionTable::new(1 * 1024 * 1024));
    let pawn_hashtable = Arc::new(PawnHashTable::new(1 * 1024 * 1024));
    let killers_table = Arc::new(KillersTable::default());
    let history_table = Arc::new(HistoryTable::default());
    let abort_token = Arc::new(AtomicBool::new(false));
    let ponder_token = Arc::new(AtomicBool::new(false));

    for pgn in pgn_loader {
        let pgn = match pgn {
            Ok(value) => value,
            Err(_) => {
                println!("Invalid PGN");
                return;
            }
        };

        let board = Bitboard::new_initial_position(
            Some(evaluation_parameters.clone()),
            Some(zobrist_container.clone()),
            Some(patterns_container.clone()),
            Some(see_container.clone()),
            Some(magic_container.clone()),
        );

        let mut context = SearchContext::new(
            board,
            0,
            0,
            0,
            0,
            0,
            0,
            false,
            false,
            false,
            false,
            Vec::new(),
            None,
            0,
            transposition_table.clone(),
            pawn_hashtable.clone(),
            killers_table.clone(),
            history_table.clone(),
            abort_token.clone(),
            ponder_token.clone(),
        );

        let mut viable_positions = Vec::new();
        for (index, r#move) in pgn.moves.iter().enumerate() {
            context.board.make_move(*r#move);

            if index < min_ply {
                continue;
            }

            if context.board.is_king_checked(context.board.active_color) {
                continue;
            }

            let q_score = qsearch::run::<false>(&mut context, 0, 0, MIN_ALPHA, MIN_BETA);
            if q_score > max_score {
                continue;
            }

            let evaluation = -((context.board.active_color as i16) * 2 - 1) * context.board.evaluate_without_cache();
            if evaluation.abs_diff(q_score) > max_diff {
                continue;
            }

            viable_positions.push(format!("{} c9 \"{}\";", context.board.to_epd(), pgn.result));
        }

        for _ in 0..density {
            if viable_positions.is_empty() {
                break;
            }

            let index = fastrand::usize(0..viable_positions.len());
            output_positions.push(viable_positions[index].to_owned());
            viable_positions.remove(index);
        }

        parsed_pgns += 1;
        if parsed_pgns % 1000 == 0 {
            println!("Parsed PGNS: {} ({} output positions)", parsed_pgns, output_positions.len());
        }
    }

    let output_file = match File::create(output_file) {
        Ok(value) => value,
        Err(_) => {
            println!("Can't create output file");
            return;
        }
    };
    let mut output_file_line_writer = LineWriter::new(output_file);

    for fen in output_positions {
        output_file_line_writer.write_all((fen + "\n").as_bytes()).unwrap();
    }
}
