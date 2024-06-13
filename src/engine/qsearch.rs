use super::context::SearchContext;
use super::*;
use crate::state::movescan::Move;
use crate::state::movescan::MoveFlags;
use crate::state::*;
use crate::utils::conditional_expression;
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
pub fn run<const DIAG: bool>(context: &mut SearchContext, ply: u16, mut alpha: i16, beta: i16) -> i16 {
    context.statistics.q_nodes_count += 1;
    context.statistics.max_ply = cmp::max(ply, context.statistics.max_ply);

    if context.board.pieces[context.board.active_color][KING] == 0 {
        conditional_expression!(DIAG, context.statistics.q_leafs_count += 1);
        return -CHECKMATE_SCORE + (ply as i16);
    }

    let stand_pat = context.board.evaluate::<DIAG>(context.board.active_color, &context.pawn_hashtable, &mut context.statistics);
    if stand_pat >= beta {
        conditional_expression!(DIAG, context.statistics.q_leafs_count += 1);
        conditional_expression!(DIAG, context.statistics.q_beta_cutoffs += 1);
        return stand_pat;
    }

    alpha = cmp::max(alpha, stand_pat);

    let mut moves = [MaybeUninit::uninit(); MAX_MOVES_COUNT];
    let mut move_scores = [MaybeUninit::uninit(); MAX_MOVES_COUNT];
    let moves_count = context.board.get_moves::<true>(&mut moves, 0, u64::MAX);

    assign_move_scores(context, &moves, &mut move_scores, moves_count);

    let mut found = false;
    for move_index in 0..moves_count {
        let (r#move, score) = sort_next_move(&mut moves, &mut move_scores, move_index, moves_count);

        if score_pruning_can_be_applied(context, score) {
            conditional_expression!(DIAG, context.statistics.q_score_pruning_accepted += 1);
            break;
        } else {
            conditional_expression!(DIAG, context.statistics.q_score_pruning_rejected += 1);
        }

        if futility_pruning_can_be_applied(context, score, stand_pat, alpha) {
            conditional_expression!(DIAG, context.statistics.q_futility_pruning_accepted += 1);
            break;
        } else {
            conditional_expression!(DIAG, context.statistics.q_futility_pruning_rejected += 1);
        }

        found = true;

        context.board.make_move(r#move);
        let score = -run::<DIAG>(context, ply + 1, -beta, -alpha);
        context.board.undo_move(r#move);

        alpha = cmp::max(alpha, score);
        if alpha >= beta {
            conditional_expression!(DIAG, context.statistics.q_beta_cutoffs += 1);
            if move_index == 0 {
                conditional_expression!(DIAG, context.statistics.q_perfect_cutoffs += 1);
            } else {
                conditional_expression!(DIAG, context.statistics.q_non_perfect_cutoffs += 1);
            }

            break;
        }
    }

    if !found {
        conditional_expression!(DIAG, context.statistics.q_leafs_count += 1);
    }

    alpha
}

/// Assigns scores for `moves` by filling `move_scores` array with `moves_count` length, based on current `context`. Move ordering in
/// quiescence search is mainly based on SEE and works as follows:
///  - for every en passant, assign 0
///  - for every capture with promotion (excluding underpromotions), assign value of the promoted piece
///  - for rest of the moves, assign SEE result
fn assign_move_scores(
    context: &SearchContext,
    moves: &[MaybeUninit<Move>; MAX_MOVES_COUNT],
    move_scores: &mut [MaybeUninit<i16>; MAX_MOVES_COUNT],
    moves_count: usize,
) {
    let mut attackers_cache = [0; 64];
    let mut defenders_cache = [0; 64];

    for move_index in 0..moves_count {
        let r#move = unsafe { moves[move_index].assume_init() };

        if r#move.get_flags() == MoveFlags::EN_PASSANT {
            move_scores[move_index].write(0);
        } else if r#move.is_promotion() {
            move_scores[move_index].write(if r#move.get_promotion_piece() == QUEEN {
                context.board.evaluation_parameters.piece_value[r#move.get_promotion_piece()]
            } else {
                -9999
            });
        } else {
            let square = r#move.get_to();
            let attacking_piece = context.board.get_piece(r#move.get_from());
            let captured_piece = context.board.get_piece(r#move.get_to());

            let attackers = if attackers_cache[square] != 0 {
                attackers_cache[square] as usize
            } else {
                attackers_cache[square] = context.board.get_attacking_pieces(context.board.active_color ^ 1, square) as u8;
                attackers_cache[square] as usize
            };

            let defenders = if defenders_cache[square] != 0 {
                defenders_cache[square] as usize
            } else {
                defenders_cache[square] = context.board.get_attacking_pieces(context.board.active_color, square) as u8;
                defenders_cache[square] as usize
            };

            move_scores[move_index].write(context.board.see.get(attacking_piece, captured_piece, attackers, defenders));
        }
    }
}

/// Checks if the score pruning can be applied for `move_score`. The main idea here is to omit all capture sequances, which are clearly
/// loosing material (`move_score` is less than [q_score_pruning_treshold]) and with high probability won't improve alpha.
fn score_pruning_can_be_applied(context: &SearchContext, move_score: i16) -> bool {
    move_score < context.parameters.q_score_pruning_treshold
}

/// Checks if the futility pruning can be applied for `move_score`. The main idea here is similar to score pruning, but instead of checking
/// if the specified capture sequence loses some material or not, it checks if the final result added to the `stand_pat` and [q_futility_pruning_margin]
/// will be below alpha - if yes, then we can safely assume that this move is not enough good to be relevant for the search.
fn futility_pruning_can_be_applied(context: &SearchContext, move_score: i16, stand_pat: i16, alpha: i16) -> bool {
    stand_pat + move_score + context.parameters.q_futility_pruning_margin < alpha
}
