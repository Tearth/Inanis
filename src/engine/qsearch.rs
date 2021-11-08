use super::context::SearchContext;
use super::*;
use crate::state::movescan::Move;
use crate::state::movescan::MoveFlags;
use crate::state::*;
use std::mem::MaybeUninit;

pub fn run(context: &mut SearchContext, depth: i8, ply: u16, mut alpha: i16, beta: i16) -> i16 {
    context.statistics.q_nodes_count += 1;

    if context.board.pieces[context.board.active_color as usize][KING as usize] == 0 {
        context.statistics.q_leafs_count += 1;
        return -CHECKMATE_SCORE + (ply as i16);
    }

    let stand_pat = -((context.board.active_color as i16) * 2 - 1) * context.board.evaluate(&mut context.pawn_hashtable, &mut context.statistics);
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
    let moves_count = context.board.get_moves::<true>(&mut moves);

    assign_move_scores(context, &moves, &mut move_scores, moves_count);

    for move_index in 0..moves_count {
        sort_next_move(&mut moves, &mut move_scores, move_index, moves_count);

        let r#move = moves[move_index];
        context.board.make_move(&r#move);

        let score = -run(context, depth - 1, ply + 1, -beta, -alpha);
        context.board.undo_move(&r#move);

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

    if moves_count == 0 {
        context.statistics.q_leafs_count += 1;
    }

    alpha
}

fn assign_move_scores(context: &SearchContext, moves: &[Move], move_scores: &mut [i16], moves_count: usize) {
    for move_index in 0..moves_count {
        let r#move = moves[move_index];

        if r#move.get_flags() == MoveFlags::EN_PASSANT {
            move_scores[move_index] = 0;
            continue;
        }

        let field = r#move.get_to();
        let attacking_piece = context.board.get_piece(r#move.get_from());
        let captured_piece = context.board.get_piece(r#move.get_to());
        let attackers = context.board.get_attacking_pieces(context.board.active_color ^ 1, field);
        let defenders = context.board.get_attacking_pieces(context.board.active_color, field);

        move_scores[move_index] = (see::get(attacking_piece, captured_piece, attackers, defenders) as i16) * 100;
    }
}
