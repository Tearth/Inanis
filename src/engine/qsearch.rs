use super::context::SearchContext;
use super::*;
use crate::state::movescan::Move;
use crate::state::movescan::MoveFlags;
use crate::state::*;
use std::cmp;
use std::mem::MaybeUninit;

pub const SCORE_PRUNING_THRESHOLD: i16 = 0;
pub const FUTILITY_PRUNING_MARGIN: i16 = 100;

/// Entry point of the quiescence search. The main idea here is to reduce the horizon effect by processing capture sequences and eventually
/// make a quiet position suitable for final evaluation. `context`, `depth`, `ply`, `alpha` and `beta` are provided by the leaf of the regular search.
///
/// Search steps:
///  - test if the friendly king was not captured earlier
///  - calculate stand-pat score and process initial pruning/alpha update
///  - main loop:
///     - score pruning
///     - futility pruning (<https://www.chessprogramming.org/Delta_Pruning>)
pub fn run(context: &mut SearchContext, depth: i8, ply: u16, mut alpha: i16, beta: i16) -> i16 {
    context.statistics.q_nodes_count += 1;
    context.statistics.max_ply = cmp::max(ply, context.statistics.max_ply);

    if context.board.pieces[context.board.active_color as usize][KING as usize] == 0 {
        context.statistics.q_leafs_count += 1;
        return -CHECKMATE_SCORE + (ply as i16);
    }

    let stand_pat = -((context.board.active_color as i16) * 2 - 1) * context.board.evaluate(&context.pawn_hashtable, &mut context.statistics);
    if stand_pat >= beta {
        context.statistics.q_leafs_count += 1;
        context.statistics.q_beta_cutoffs += 1;
        return beta;
    }

    if stand_pat > alpha {
        alpha = stand_pat;
    }

    let mut moves: [Move; MAX_MOVES_COUNT] = unsafe { MaybeUninit::uninit().assume_init() };
    let mut move_scores: [i16; MAX_MOVES_COUNT] = unsafe { MaybeUninit::uninit().assume_init() };
    let moves_count = context.board.get_moves::<true>(&mut moves, 0, u64::MAX);

    assign_move_scores(context, &moves, &mut move_scores, moves_count);

    let mut found = false;
    for move_index in 0..moves_count {
        let r#move = sort_next_move(&mut moves, &mut move_scores, move_index, moves_count);
        if score_pruning_can_be_applied(move_scores[move_index]) {
            context.statistics.q_score_pruning_accepted += 1;
            break;
        } else {
            context.statistics.q_score_pruning_rejected += 1;
        }

        if futility_pruning_can_be_applied(move_scores[move_index], stand_pat, alpha) {
            context.statistics.q_futility_pruning_accepted += 1;
            break;
        } else {
            context.statistics.q_futility_pruning_rejected += 1;
        }

        found = true;

        context.board.make_move(r#move);
        let score = -run(context, depth - 1, ply + 1, -beta, -alpha);
        context.board.undo_move(r#move);

        if score > alpha {
            alpha = score;

            if alpha >= beta {
                context.statistics.q_beta_cutoffs += 1;
                if move_index == 0 {
                    context.statistics.q_perfect_cutoffs += 1;
                } else {
                    context.statistics.q_non_perfect_cutoffs += 1;
                }

                break;
            }
        }
    }

    if !found {
        context.statistics.q_leafs_count += 1;
    }

    alpha
}

/// Assigns scores for `moves` by filling `move_scores` array with `moves_count` length, based on current `context`. Move ordering in
/// quiescence search is mainly based on SEE and works as follows:
///  - for every en passant, assign 0
///  - for every capture with promotion, assign value of the promoted piece
///  - for rest of the moves, assign SEE result
fn assign_move_scores(context: &SearchContext, moves: &[Move], move_scores: &mut [i16], moves_count: usize) {
    for move_index in 0..moves_count {
        let r#move = moves[move_index];

        if r#move.get_flags() == MoveFlags::EN_PASSANT {
            move_scores[move_index] = 0;
            continue;
        }

        if r#move.is_promotion() {
            move_scores[move_index] = context.board.evaluation_parameters.piece_value[r#move.get_promotion_piece() as usize];
            continue;
        }

        let field = r#move.get_to();
        let attacking_piece = context.board.get_piece(r#move.get_from());
        let captured_piece = context.board.get_piece(r#move.get_to());
        let attackers = context.board.get_attacking_pieces(context.board.active_color ^ 1, field);
        let defenders = context.board.get_attacking_pieces(context.board.active_color, field);

        move_scores[move_index] = context
            .board
            .see
            .get(attacking_piece, captured_piece, attackers, defenders, &context.board.evaluation_parameters);
    }
}

/// Checks if the score pruning can be applied for `move_score`. The main idea here is to omit all capture sequances, which are clearly
/// loosing material (`move_score` is less than [SCORE_PRUNING_THRESHOLD]) and with high probability won't improve alpha.
fn score_pruning_can_be_applied(move_score: i16) -> bool {
    move_score < SCORE_PRUNING_THRESHOLD
}

/// Checks if the futility pruning can be applied for `move_score`. The main idea here is similar to score pruning, but instead of checking
/// if the specified capture sequence loses some material or not, it checks if the final result added to the `stand_pat` and [FUTILITY_PRUNING_MARGIN]
/// will be below alpha - if yes, then we can safely assume that this move is not enough good to be relevant for the search.
fn futility_pruning_can_be_applied(move_score: i16, stand_pat: i16, alpha: i16) -> bool {
    stand_pat + move_score + FUTILITY_PRUNING_MARGIN < alpha
}
