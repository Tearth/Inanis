use crate::engine::context::SearchContext;
use crate::engine::*;
use crate::evaluation::*;
use crate::state::movescan::MoveFlags;
use crate::state::*;
use crate::utils::assert_fast;
use crate::MoveScores;
use crate::Moves;

/// Assigns scores for `moves` by filling `move_scores` array with `moves_count` length, based on current `context`. Move ordering in
/// quiescence search is mainly based on SEE and works as follows:
///  - for every en passant, assign 0
///  - for every promotion, ignore all of them except queens
///  - for rest of the moves, assign SEE result
pub fn assign_move_scores(context: &SearchContext, moves: &Moves, move_scores: &mut MoveScores, moves_count: usize) {
    assert_fast!(moves_count < MAX_MOVES_COUNT);

    let mut attackers_cache = [0; 64];
    let mut defenders_cache = [0; 64];

    for move_index in 0..moves_count {
        let r#move = unsafe { moves[move_index].assume_init() };

        if r#move.get_flags() == MoveFlags::EN_PASSANT {
            move_scores[move_index].write(0);
        } else if r#move.is_promotion() {
            move_scores[move_index].write(if r#move.get_promotion_piece() == QUEEN { PIECE_VALUE[QUEEN] } else { -9999 });
        } else {
            let square = r#move.get_to();
            let attacking_piece = context.board.get_piece(r#move.get_from());
            let captured_piece = context.board.get_piece(r#move.get_to());

            let attackers = if attackers_cache[square] != 0 {
                attackers_cache[square] as usize
            } else {
                attackers_cache[square] = context.board.get_attacking_pieces(context.board.stm ^ 1, square) as u8;
                attackers_cache[square] as usize
            };

            let defenders = if defenders_cache[square] != 0 {
                defenders_cache[square] as usize
            } else {
                defenders_cache[square] = context.board.get_attacking_pieces(context.board.stm, square) as u8;
                defenders_cache[square] as usize
            };

            move_scores[move_index].write(see::get(attacking_piece, captured_piece, attackers, defenders));
        }
    }
}
