use self::context::PerftContext;
use crate::engine;
use crate::state::*;
use crate::utils::panic_fast;
use std::mem::MaybeUninit;

pub mod context;
pub mod divided;
pub mod fast;
pub mod normal;

/// Internal perft function, common for every mode.
pub fn run_internal(context: &mut PerftContext, depth: i32) -> u64 {
    if context.check_integrity {
        let original_hash = context.board.state.hash;
        let original_pawn_hash = context.board.state.pawn_hash;
        let original_eval = context.board.evaluate_without_cache(WHITE);

        context.board.recalculate_hashes();
        context.board.recalculate_incremental_values();

        if original_hash != context.board.state.hash {
            panic_fast!(
                "Integrity check failed, invalid hash: fen={}, original_hash={}, context.board.state.hash={}",
                context.board,
                original_hash,
                context.board.state.hash
            );
        }

        if original_pawn_hash != context.board.state.pawn_hash {
            panic_fast!(
                "Integrity check failed, invalid pawn hash: fen={}, original_pawn_hash={}, context.board.state.pawn_hash={}",
                context.board,
                original_pawn_hash,
                context.board.state.pawn_hash
            );
        }

        let eval = context.board.evaluate_without_cache(WHITE);
        if original_eval != eval {
            panic_fast!("Integrity check failed, invalid evaluation: fen={}, original_eval={}, eval={}", context.board, original_eval, eval)
        }
    }

    if depth <= 0 {
        return 1;
    }

    if context.fast {
        if let Some(entry) = context.hashtable.get(context.board.state.hash, depth as u8) {
            return entry.leafs_count;
        }
    }

    let mut moves = [MaybeUninit::uninit(); engine::MAX_MOVES_COUNT];
    let moves_count = context.board.get_all_moves(&mut moves, u64::MAX);

    let mut count = 0;
    for r#move in &moves[0..moves_count] {
        let r#move = unsafe { r#move.assume_init() };
        if context.check_integrity && !r#move.is_legal(context.board) {
            panic_fast!("Integrity check failed, illegal move: fen={}, r#move.data={}", context.board, r#move.data);
        }

        context.board.make_move(r#move);

        if !context.board.is_king_checked(context.board.stm ^ 1) {
            count += run_internal(context, depth - 1);

            if !context.fast && depth == 1 {
                if r#move.is_capture() {
                    context.stats.captures += 1;
                }

                if r#move.is_en_passant() {
                    context.stats.en_passants += 1;
                }

                if r#move.is_castling() {
                    context.stats.castles += 1;
                }

                if r#move.is_promotion() {
                    context.stats.promotions += 1;
                }

                if context.board.is_king_checked(context.board.stm) {
                    context.stats.checks += 1;
                }
            }
        }

        context.board.undo_move(r#move);
    }

    if context.fast {
        context.hashtable.add(context.board.state.hash, depth as u8, count);
    }

    count
}
