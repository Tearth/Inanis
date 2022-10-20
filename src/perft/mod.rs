use self::context::PerftContext;
use crate::engine;
use crate::state::*;
use std::mem::MaybeUninit;
use std::u64;

pub mod context;
pub mod divided;
pub mod fast;
pub mod normal;

/// Internal perft function, common for every mode.
pub fn run_internal(context: &mut PerftContext, depth: i32) -> u64 {
    if context.check_integrity {
        let original_hash = context.board.hash;
        let original_pawn_hash = context.board.pawn_hash;
        let original_evaluation = context.board.evaluate_without_cache(WHITE);

        context.board.recalculate_hashes();
        context.board.recalculate_incremental_values();

        if original_hash != context.board.hash {
            panic!("Integrity check failed, invalid hash: fen={}, original_hash={}, context.board.hash={}", context.board, original_hash, context.board.hash);
        }

        if original_pawn_hash != context.board.pawn_hash {
            panic!(
                "Integrity check failed, invalid pawn hash: fen={}, original_pawn_hash={}, context.board.pawn_hash={}",
                context.board, original_pawn_hash, context.board.pawn_hash
            );
        }

        let evaluation = context.board.evaluate_without_cache(WHITE);
        if original_evaluation != evaluation {
            panic!("Integrity check failed, invalid evaluation: fen={}, original_evaluation={}, evaluation={}", context.board, original_evaluation, evaluation)
        }
    }

    if depth <= 0 {
        return 1;
    }

    if context.fast {
        if let Some(entry) = context.hashtable.get(context.board.hash, depth as u8) {
            return entry.leafs_count;
        }
    }

    let mut moves = [MaybeUninit::uninit(); engine::MAX_MOVES_COUNT];
    let moves_count = context.board.get_all_moves(&mut moves, u64::MAX);

    let mut count = 0;
    for r#move in &moves[0..moves_count] {
        let r#move = unsafe { r#move.assume_init() };
        if context.check_integrity && !r#move.is_legal(context.board) {
            panic!("Integrity check failed, illegal move: fen={}, r#move.data={}", context.board, r#move.data);
        }

        context.board.make_move(r#move);

        if !context.board.is_king_checked(context.board.active_color ^ 1) {
            count += run_internal(context, depth - 1);

            if !context.fast && depth == 1 {
                if r#move.is_capture() {
                    context.statistics.captures += 1;
                }

                if r#move.is_en_passant() {
                    context.statistics.en_passants += 1;
                }

                if r#move.is_castling() {
                    context.statistics.castles += 1;
                }

                if r#move.is_promotion() {
                    context.statistics.promotions += 1;
                }

                if context.board.is_king_checked(context.board.active_color) {
                    context.statistics.checks += 1;
                }
            }
        }

        context.board.undo_move(r#move);
    }

    if context.fast {
        context.hashtable.add(context.board.hash, depth as u8, count);
    }

    count
}
