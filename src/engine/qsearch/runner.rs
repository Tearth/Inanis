use crate::engine::context::SearchContext;
use crate::engine::*;
use crate::state::*;
use crate::utils::dev;
use crate::utils::param;
use qsearch::movepick;
use std::cmp;
use std::mem::MaybeUninit;

/// Entry point of the quiescence search. The main idea here is to reduce the horizon effect by processing capture sequences and eventually
/// make a quiet position suitable for final evaluation. `context`, `ply`, `alpha` and `beta` are provided by the leaf of the regular search.
/// If `DIAG` is set to true, additional statistics will be gathered (with a small performance penalty).
///
/// Search steps:
///  - test if the friendly king was not captured earlier
///  - calculate stand-pat score and process initial pruning/alpha update
///  - main loop:
///     - score pruning
///     - futility pruning (<https://www.chessprogramming.org/Delta_Pruning>)
pub fn run(context: &mut SearchContext, ply: u16, mut alpha: i16, beta: i16) -> i16 {
    debug_assert!(alpha <= beta);

    context.statistics.q_nodes_count += 1;
    context.statistics.max_ply = cmp::max(ply, context.statistics.max_ply);

    if context.board.pieces[context.board.active_color][KING] == 0 {
        dev!(context.statistics.q_leafs_count += 1);
        return -CHECKMATE_SCORE + (ply as i16);
    }

    let stand_pat = context.board.evaluate(context.board.active_color, &context.pawn_hashtable, &mut context.statistics);
    if stand_pat >= beta {
        dev!(context.statistics.q_leafs_count += 1);
        dev!(context.statistics.q_beta_cutoffs += 1);
        return stand_pat;
    }

    alpha = cmp::max(alpha, stand_pat);

    let mut moves = [MaybeUninit::uninit(); MAX_MOVES_COUNT];
    let mut move_scores = [MaybeUninit::uninit(); MAX_MOVES_COUNT];
    let moves_count = context.board.get_moves::<true>(&mut moves, 0, u64::MAX);

    movepick::assign_move_scores(context, &moves, &mut move_scores, moves_count);

    let mut found = false;
    for move_index in 0..moves_count {
        let (r#move, score) = movesort::sort_next_move(&mut moves, &mut move_scores, move_index, moves_count);

        if score_pruning_can_be_applied(context, score) {
            dev!(context.statistics.q_score_pruning_accepted += 1);
            break;
        } else {
            dev!(context.statistics.q_score_pruning_rejected += 1);
        }

        if futility_pruning_can_be_applied(context, score, stand_pat, alpha) {
            dev!(context.statistics.q_futility_pruning_accepted += 1);
            break;
        } else {
            dev!(context.statistics.q_futility_pruning_rejected += 1);
        }

        found = true;

        context.board.make_move(r#move);
        let score = -run(context, ply + 1, -beta, -alpha);
        context.board.undo_move(r#move);

        alpha = cmp::max(alpha, score);
        if alpha >= beta {
            dev!(context.statistics.q_beta_cutoffs += 1);
            if move_index == 0 {
                dev!(context.statistics.q_perfect_cutoffs += 1);
            } else {
                dev!(context.statistics.q_non_perfect_cutoffs += 1);
            }

            break;
        }
    }

    if !found {
        dev!(context.statistics.q_leafs_count += 1);
    }

    alpha
}

/// Checks if the score pruning can be applied for `move_score`. The main idea here is to omit all capture sequances, which are clearly
/// loosing material (`move_score` is less than [q_score_pruning_treshold]) and with high probability won't improve alpha.
fn score_pruning_can_be_applied(context: &SearchContext, move_score: i16) -> bool {
    move_score < param!(context.parameters.q_score_pruning_treshold)
}

/// Checks if the futility pruning can be applied for `move_score`. The main idea here is similar to score pruning, but instead of checking
/// if the specified capture sequence loses some material or not, it checks if the final result added to the `stand_pat` and [q_futility_pruning_margin]
/// will be below alpha - if yes, then we can safely assume that this move is not enough good to be relevant for the search.
fn futility_pruning_can_be_applied(context: &SearchContext, move_score: i16, stand_pat: i16, alpha: i16) -> bool {
    stand_pat + move_score + param!(context.parameters.q_futility_pruning_margin) < alpha
}
