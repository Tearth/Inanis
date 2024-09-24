use crate::engine::context::SearchContext;
use crate::engine::*;
use crate::state::movescan::Move;
use crate::state::*;
use crate::utils::bithelpers::BitHelpers;
use crate::utils::dev;
use crate::utils::panic_fast;
use std::mem::MaybeUninit;

pub const MOVE_ORDERING_HASH_MOVE: i16 = 10000;
pub const MOVE_ORDERING_WINNING_CAPTURES_OFFSET: i16 = 100;
pub const MOVE_ORDERING_KILLER_MOVE_1: i16 = 99;
pub const MOVE_ORDERING_KILLER_MOVE_2: i16 = 98;
pub const MOVE_ORDERING_COUNTERMOVE: i16 = 97;
pub const MOVE_ORDERING_QUEEN_PROMOTION: i16 = 95;
pub const MOVE_ORDERING_ROOK_PROMOTION: i16 = 94;
pub const MOVE_ORDERING_BISHOP_PROMOTION: i16 = 93;
pub const MOVE_ORDERING_KNIGHT_PROMOTION: i16 = 92;
pub const MOVE_ORDERING_CASTLING: i16 = 91;
pub const MOVE_ORDERING_HISTORY_MOVE: u8 = 180;
pub const MOVE_ORDERING_HISTORY_MOVE_OFFSET: i16 = -90;
pub const MOVE_ORDERING_LOSING_CAPTURES_OFFSET: i16 = -100;

#[derive(std::cmp::PartialEq)]
pub enum MoveGenStage {
    ReadyToCheckHashMove,
    HashMove,
    ReadyToGenerateCaptures,
    Captures,
    ReadyToGenerateKillers,
    Killers,
    ReadyToGenerateCounters,
    Counters,
    ReadyToGenerateQuiets,
    AllGenerated,
}

/// Gets a next move to analyze. This function acts as pseudo-iterator and takes care about managing move generator stages, which is basically
/// a state machine (<https://en.wikipedia.org/wiki/Finite-state_machine>) with following rules:
///  - [MoveGenStage::ReadyToCheckHashMove] - default state, prepares hash move if possible
///  - [MoveGenStage::HashMove] - returns hashmove if possible
///  - [MoveGenStage::ReadyToGenerateCaptures] - generates all captures in the position
///  - [MoveGenStage::Captures] - returns subsequent elements until the end or score is less than [MOVE_ORDERING_WINNING_CAPTURES_OFFSET]
///  - [MoveGenStage::ReadyToGenerateKillers] - generates all killer moves in the position
///  - [MoveGenStage::Killers] - returns subsequent elements until all killer moves are processed
///  - [MoveGenStage::ReadyToGenerateCounters] - generates all countermoves in the position
///  - [MoveGenStage::Counters] - returns subsequent elements until all countermoves are processed
///  - [MoveGenStage::ReadyToGenerateQuiets] - generates all quiet moves in the position
///  - [MoveGenStage::AllGenerated] - returns all subsequent elements until the end
///
/// If the last stage is set and there are no more moves, [None] is returned.
pub fn get_next_move(
    context: &mut SearchContext,
    stage: &mut MoveGenStage,
    moves: &mut [MaybeUninit<Move>; MAX_MOVES_COUNT],
    move_scores: &mut [MaybeUninit<i16>; MAX_MOVES_COUNT],
    move_index: &mut usize,
    move_number: &mut usize,
    moves_count: &mut usize,
    evasion_mask: &mut u64,
    hash_move: Move,
    ply: u16,
    friendly_king_checked: bool,
    previous_move: Move,
    quiet_moves_start_index: &mut usize,
    killer_moves_cache: &mut [MaybeUninit<Move>; 2],
) -> Option<(Move, i16)> {
    debug_assert!(*move_index < MAX_MOVES_COUNT);
    debug_assert!(move_index <= moves_count);
    debug_assert!(move_number <= moves_count);
    debug_assert!(*moves_count < MAX_MOVES_COUNT);
    debug_assert!(*quiet_moves_start_index < MAX_MOVES_COUNT);
    debug_assert!(quiet_moves_start_index <= moves_count);

    if matches!(*stage, MoveGenStage::HashMove | MoveGenStage::Captures | MoveGenStage::Killers | MoveGenStage::Counters | MoveGenStage::AllGenerated) {
        *move_index += 1;
        *move_number += 1;
    }

    loop {
        if move_index >= moves_count {
            match *stage {
                MoveGenStage::HashMove => *stage = MoveGenStage::ReadyToGenerateCaptures,
                MoveGenStage::Captures => *stage = MoveGenStage::ReadyToGenerateKillers,
                MoveGenStage::Killers => *stage = MoveGenStage::ReadyToGenerateCounters,
                MoveGenStage::Counters => *stage = MoveGenStage::ReadyToGenerateQuiets,
                MoveGenStage::AllGenerated => return None,
                _ => {}
            }
        }

        match stage {
            MoveGenStage::ReadyToCheckHashMove => {
                if hash_move.is_some() {
                    *moves_count = 1;
                    *stage = MoveGenStage::HashMove;
                } else {
                    *stage = MoveGenStage::ReadyToGenerateCaptures;
                }

                dev!(context.statistics.move_generator_hash_move_stages += 1);
            }
            MoveGenStage::HashMove => {
                return Some((hash_move, MOVE_ORDERING_HASH_MOVE));
            }
            MoveGenStage::ReadyToGenerateCaptures => {
                *evasion_mask = if friendly_king_checked {
                    let king_square = (context.board.pieces[context.board.active_color][KING]).bit_scan();
                    let occupancy_bb = context.board.occupancy[WHITE] | context.board.occupancy[BLACK];

                    let queen_moves_bb = movegen::get_queen_moves(occupancy_bb, king_square);
                    let knight_moves_bb = movegen::get_knight_moves(king_square);

                    queen_moves_bb | knight_moves_bb
                } else {
                    u64::MAX
                };

                *move_index = 0;
                *moves_count = context.board.get_moves::<true>(moves, 0, *evasion_mask);

                if *moves_count == 0 {
                    *stage = MoveGenStage::ReadyToGenerateKillers;
                } else {
                    *stage = MoveGenStage::Captures;
                    assign_capture_scores(context, moves, move_scores, 0, *moves_count, hash_move);
                }

                dev!(context.statistics.move_generator_captures_stages += 1);
            }
            MoveGenStage::Captures => {
                let (r#move, score) = movesort::sort_next_move(moves, move_scores, *move_index, *moves_count);

                if r#move == hash_move {
                    *move_index += 1;
                } else if score < MOVE_ORDERING_WINNING_CAPTURES_OFFSET {
                    *stage = MoveGenStage::ReadyToGenerateKillers;
                } else {
                    return Some((r#move, score));
                }
            }
            MoveGenStage::ReadyToGenerateKillers => {
                let original_moves_count = *moves_count;
                let killer_moves = context.killers_table.get(ply);

                for (index, &killer_move) in killer_moves.iter().enumerate() {
                    if killer_move != hash_move {
                        if ((1u64 << killer_move.get_to()) & *evasion_mask) != 0 && killer_move.is_legal(&context.board) {
                            moves[*moves_count].write(killer_move);
                            move_scores[*moves_count].write(MOVE_ORDERING_KILLER_MOVE_1 - (index as i16));
                            *moves_count += 1;

                            dev!(context.statistics.killers_table_legal_moves += 1);
                        } else {
                            dev!(context.statistics.killers_table_illegal_moves += 1);
                        }
                    }

                    killer_moves_cache[index].write(killer_move);
                }

                if original_moves_count != *moves_count {
                    *stage = MoveGenStage::Killers
                } else {
                    if previous_move.is_some() {
                        *stage = MoveGenStage::ReadyToGenerateCounters
                    } else {
                        *stage = MoveGenStage::ReadyToGenerateQuiets
                    }
                };

                dev!(context.statistics.move_generator_killers_stages += 1);
            }
            MoveGenStage::Killers => {
                let (r#move, score) = movesort::sort_next_move(moves, move_scores, *move_index, *moves_count);

                if score < MOVE_ORDERING_KILLER_MOVE_2 {
                    if previous_move.is_some() {
                        *stage = MoveGenStage::ReadyToGenerateCounters;
                    } else {
                        *stage = MoveGenStage::ReadyToGenerateQuiets;
                    }
                } else {
                    return Some((r#move, score));
                }
            }
            MoveGenStage::ReadyToGenerateCounters => {
                let original_moves_count = *moves_count;
                let countermove = context.countermoves_table.get(previous_move);
                let killer_1 = unsafe { killer_moves_cache[0].assume_init() };
                let killer_2 = unsafe { killer_moves_cache[1].assume_init() };

                if countermove != hash_move && countermove != killer_1 && countermove != killer_2 {
                    if ((1u64 << countermove.get_to()) & *evasion_mask) != 0 && countermove.is_legal(&context.board) {
                        moves[*moves_count].write(countermove);
                        move_scores[*moves_count].write(MOVE_ORDERING_COUNTERMOVE);
                        *moves_count += 1;

                        dev!(context.statistics.countermoves_table_legal_moves += 1);
                    } else {
                        dev!(context.statistics.countermoves_table_illegal_moves += 1);
                    }
                }

                if original_moves_count != *moves_count {
                    *stage = MoveGenStage::Counters;
                } else {
                    *stage = MoveGenStage::ReadyToGenerateQuiets;
                };

                dev!(context.statistics.move_generator_counters_stages += 1);
            }
            MoveGenStage::Counters => {
                let (r#move, score) = movesort::sort_next_move(moves, move_scores, *move_index, *moves_count);

                if score < MOVE_ORDERING_COUNTERMOVE {
                    *stage = MoveGenStage::ReadyToGenerateQuiets;
                } else {
                    return Some((r#move, score));
                }
            }
            MoveGenStage::ReadyToGenerateQuiets => {
                let original_moves_count = *moves_count;

                *quiet_moves_start_index = *move_index;
                *moves_count = context.board.get_moves::<false>(moves, *moves_count, *evasion_mask);
                *stage = MoveGenStage::AllGenerated;

                assign_quiet_scores(context, moves, move_scores, original_moves_count, *moves_count, hash_move, previous_move, ply);
                dev!(context.statistics.move_generator_quiets_stages += 1);
            }
            MoveGenStage::AllGenerated => {
                let (r#move, score) = movesort::sort_next_move(moves, move_scores, *move_index, *moves_count);

                if r#move == hash_move || score == MOVE_ORDERING_KILLER_MOVE_1 || score == MOVE_ORDERING_KILLER_MOVE_2 || score == MOVE_ORDERING_COUNTERMOVE {
                    *move_index += 1;
                } else {
                    return Some((r#move, score));
                }
            }
        }
    }
}

/// Assigns capture scores for `moves` by filling `move_scores` array with `moves_count` length (starting from `start_index`), based on current `context`.
/// If transposition table move is available, it's passed as `tt_move` too. Moves are prioritized as follows (from most important to the less ones):
///  - for transposition table move, assign [MOVE_ORDERING_HASH_MOVE]
///  - for every positive capture, assign SEE score + [MOVE_ORDERING_WINNING_CAPTURES_OFFSET]
///  - for every negative capture, assign SEE score + [MOVE_ORDERING_LOSING_CAPTURES_OFFSET]
fn assign_capture_scores(
    context: &SearchContext,
    moves: &[MaybeUninit<Move>; MAX_MOVES_COUNT],
    move_scores: &mut [MaybeUninit<i16>; MAX_MOVES_COUNT],
    start_index: usize,
    moves_count: usize,
    tt_move: Move,
) {
    debug_assert!(start_index < MAX_MOVES_COUNT);
    debug_assert!(start_index <= moves_count);
    debug_assert!(start_index + moves_count < MAX_MOVES_COUNT);

    let mut attackers_cache = [0; 64];
    let mut defenders_cache = [0; 64];

    for move_index in start_index..moves_count {
        let r#move = unsafe { moves[move_index].assume_init() };

        if r#move == tt_move {
            move_scores[move_index].write(MOVE_ORDERING_HASH_MOVE);
        } else if r#move.is_en_passant() {
            move_scores[move_index].write(MOVE_ORDERING_WINNING_CAPTURES_OFFSET);
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

            let see = context.board.see.get(attacking_piece, captured_piece, attackers, defenders);
            move_scores[move_index].write(if see >= 0 { see + MOVE_ORDERING_WINNING_CAPTURES_OFFSET } else { see + MOVE_ORDERING_LOSING_CAPTURES_OFFSET });
        }
    }
}

/// Assigns quiet scores for `moves` by filling `move_scores` array with `moves_count` length (starting from `start_index`), based on current `context`.
/// If transposition table move is available, it's passed as `tt_move` too. Moves are prioritized as follows (from most important to the less ones):
///  - for transposition table move, assign [MOVE_ORDERING_HASH_MOVE]
///  - for every promotion (excluding these with capture), assign [MOVE_ORDERING_QUEEN_PROMOTION], [MOVE_ORDERING_ROOK_PROMOTION],
///    [MOVE_ORDERING_BISHOP_PROMOTION] or [MOVE_ORDERING_KNIGHT_PROMOTION]
///  - for every move found in killer table, assign [MOVE_ORDERING_KILLER_MOVE_1] or [MOVE_ORDERING_KILLER_MOVE_2]
///  - for every countermove, assign [MOVE_ORDERING_COUNTERMOVE]
///  - for every castling, assign [MOVE_ORDERING_CASTLING]
///  - for every quiet move which didn't fit in other categories, assign score from history table
fn assign_quiet_scores(
    context: &SearchContext,
    moves: &[MaybeUninit<Move>; MAX_MOVES_COUNT],
    move_scores: &mut [MaybeUninit<i16>; MAX_MOVES_COUNT],
    start_index: usize,
    moves_count: usize,
    tt_move: Move,
    previous_move: Move,
    ply: u16,
) {
    debug_assert!(start_index < MAX_MOVES_COUNT);
    debug_assert!(start_index <= moves_count);
    debug_assert!(start_index + moves_count < MAX_MOVES_COUNT);

    let killer_moves = context.killers_table.get(ply);
    let countermove = context.countermoves_table.get(previous_move);

    for move_index in start_index..moves_count {
        let r#move = unsafe { moves[move_index].assume_init() };

        if r#move == tt_move {
            move_scores[move_index].write(MOVE_ORDERING_HASH_MOVE);
        } else if r#move == countermove {
            move_scores[move_index].write(MOVE_ORDERING_COUNTERMOVE);
        } else if r#move.is_quiet() {
            let mut killer_move_found = false;
            for (index, &killer_move) in killer_moves.iter().enumerate() {
                if killer_move == r#move {
                    move_scores[move_index].write(MOVE_ORDERING_KILLER_MOVE_1 - (index as i16));
                    killer_move_found = true;
                    break;
                }
            }

            if killer_move_found {
                continue;
            }

            let value = context.history_table.get(r#move.get_from(), r#move.get_to(), MOVE_ORDERING_HISTORY_MOVE) as i16;
            move_scores[move_index].write(value + MOVE_ORDERING_HISTORY_MOVE_OFFSET);
        } else if r#move.is_promotion() {
            move_scores[move_index].write(match r#move.get_promotion_piece() {
                QUEEN => MOVE_ORDERING_QUEEN_PROMOTION,
                ROOK => MOVE_ORDERING_ROOK_PROMOTION,
                BISHOP => MOVE_ORDERING_BISHOP_PROMOTION,
                KNIGHT => MOVE_ORDERING_KNIGHT_PROMOTION,
                _ => panic_fast!("Invalid value: fen={}, r#move.data={}", context.board, r#move.data),
            });
        } else if r#move.is_castling() {
            move_scores[move_index].write(MOVE_ORDERING_CASTLING);
        } else {
            panic_fast!("Sorting rule missing: fen={}, r#move.data={}", context.board, r#move.data);
        }
    }
}
