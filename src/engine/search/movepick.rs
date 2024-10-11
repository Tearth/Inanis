use crate::engine::context::SearchContext;
use crate::engine::*;
use crate::state::movescan::Move;
use crate::state::*;
use crate::utils::assert_fast;
use crate::utils::bithelpers::BitHelpers;
use crate::utils::dev;
use crate::utils::panic_fast;
use crate::MoveScores;
use crate::Moves;
use std::mem::MaybeUninit;

pub const MOVEORD_HASH_MOVE: i16 = 10000;
pub const MOVEORD_WINNING_CAPTURES_OFFSET: i16 = 100;
pub const MOVEORD_KILLER_MOVE_1: i16 = 99;
pub const MOVEORD_KILLER_MOVE_2: i16 = 98;
pub const MOVEORD_COUNTERMOVE: i16 = 97;
pub const MOVEORD_QUEEN_PROMOTION: i16 = 95;
pub const MOVEORD_ROOK_PROMOTION: i16 = 94;
pub const MOVEORD_BISHOP_PROMOTION: i16 = 93;
pub const MOVEORD_KNIGHT_PROMOTION: i16 = 92;
pub const MOVEORD_CASTLING: i16 = 91;
pub const MOVEORD_HISTORY_MOVE: u8 = 180;
pub const MOVEORD_HISTORY_MOVE_OFFSET: i16 = -90;
pub const MOVEORD_LOSING_CAPTURES_OFFSET: i16 = -100;

pub struct MoveGenState {
    pub moves: Moves,
    pub move_scores: MoveScores,
    pub stage: MoveGenStage,
    pub quiet_moves_start_index: usize,
    pub killer_moves: [MaybeUninit<Move>; 2],
    pub move_index: usize,
    pub move_number: usize,
    pub moves_count: usize,
    pub evasion_mask: u64,
    pub hash_move: Move,
    pub ply: u16,
    pub friendly_king_checked: bool,
    pub previous_move: Move,
}

#[derive(PartialEq)]
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
pub fn get_next_move(context: &mut SearchContext, state: &mut MoveGenState) -> Option<(Move, i16)> {
    assert_fast!(state.move_index < MAX_MOVES_COUNT);
    assert_fast!(state.move_index <= state.moves_count);
    assert_fast!(state.move_number <= state.moves_count);
    assert_fast!(state.moves_count < MAX_MOVES_COUNT);
    assert_fast!(state.quiet_moves_start_index < MAX_MOVES_COUNT);
    assert_fast!(state.quiet_moves_start_index <= state.moves_count);
    assert_fast!(context.board.stm < 2);

    if matches!(state.stage, MoveGenStage::HashMove | MoveGenStage::Captures | MoveGenStage::Killers | MoveGenStage::Counters | MoveGenStage::AllGenerated) {
        state.move_index += 1;
        state.move_number += 1;
    }

    loop {
        if state.move_index >= state.moves_count {
            match state.stage {
                MoveGenStage::HashMove => state.stage = MoveGenStage::ReadyToGenerateCaptures,
                MoveGenStage::Captures => state.stage = MoveGenStage::ReadyToGenerateKillers,
                MoveGenStage::Killers => state.stage = MoveGenStage::ReadyToGenerateCounters,
                MoveGenStage::Counters => state.stage = MoveGenStage::ReadyToGenerateQuiets,
                MoveGenStage::AllGenerated => return None,
                _ => {}
            }
        }

        match state.stage {
            MoveGenStage::ReadyToCheckHashMove => {
                if state.hash_move.is_some() {
                    state.moves_count = 1;
                    state.stage = MoveGenStage::HashMove;
                } else {
                    state.stage = MoveGenStage::ReadyToGenerateCaptures;
                }

                dev!(context.stats.movegen_hash_move_stages += 1);
            }
            MoveGenStage::HashMove => {
                return Some((state.hash_move, MOVEORD_HASH_MOVE));
            }
            MoveGenStage::ReadyToGenerateCaptures => {
                state.evasion_mask = if state.friendly_king_checked {
                    let king_square = (context.board.pieces[context.board.stm][KING]).bit_scan();
                    let occupancy_bb = context.board.occupancy[WHITE] | context.board.occupancy[BLACK];

                    let queen_moves_bb = movegen::get_queen_moves(occupancy_bb, king_square);
                    let knight_moves_bb = movegen::get_knight_moves(king_square);

                    queen_moves_bb | knight_moves_bb
                } else {
                    u64::MAX
                };

                state.move_index = 0;
                state.moves_count = context.board.get_moves::<true>(&mut state.moves, 0, state.evasion_mask);

                if state.moves_count == 0 {
                    state.stage = MoveGenStage::ReadyToGenerateKillers;
                } else {
                    state.stage = MoveGenStage::Captures;
                    assign_capture_scores(context, state);
                }

                dev!(context.stats.movegen_captures_stages += 1);
            }
            MoveGenStage::Captures => {
                let (r#move, score) = movesort::sort_next_move(&mut state.moves, &mut state.move_scores, state.move_index, state.moves_count);

                if r#move == state.hash_move {
                    state.move_index += 1;
                } else if score < MOVEORD_WINNING_CAPTURES_OFFSET {
                    state.stage = MoveGenStage::ReadyToGenerateKillers;
                } else {
                    return Some((r#move, score));
                }
            }
            MoveGenStage::ReadyToGenerateKillers => {
                let original_moves_count = state.moves_count;
                let killer_moves = context.ktable.get(state.ply);

                for (index, &killer_move) in killer_moves.iter().enumerate() {
                    if killer_move != state.hash_move {
                        if ((1u64 << killer_move.get_to()) & state.evasion_mask) != 0 && killer_move.is_legal(&context.board) {
                            assert_fast!(state.moves_count < MAX_MOVES_COUNT);

                            state.moves[state.moves_count].write(killer_move);
                            state.move_scores[state.moves_count].write(MOVEORD_KILLER_MOVE_1 - (index as i16));
                            state.moves_count += 1;

                            dev!(context.stats.ktable_legal_moves += 1);
                        } else {
                            dev!(context.stats.ktable_illegal_moves += 1);
                        }
                    }

                    state.killer_moves[index].write(killer_move);
                }

                if original_moves_count != state.moves_count {
                    state.stage = MoveGenStage::Killers
                } else {
                    if state.previous_move.is_some() {
                        state.stage = MoveGenStage::ReadyToGenerateCounters
                    } else {
                        state.stage = MoveGenStage::ReadyToGenerateQuiets
                    }
                };

                dev!(context.stats.movegen_killers_stages += 1);
            }
            MoveGenStage::Killers => {
                let (r#move, score) = movesort::sort_next_move(&mut state.moves, &mut state.move_scores, state.move_index, state.moves_count);

                if score < MOVEORD_KILLER_MOVE_2 {
                    if state.previous_move.is_some() {
                        state.stage = MoveGenStage::ReadyToGenerateCounters;
                    } else {
                        state.stage = MoveGenStage::ReadyToGenerateQuiets;
                    }
                } else {
                    return Some((r#move, score));
                }
            }
            MoveGenStage::ReadyToGenerateCounters => {
                let original_moves_count = state.moves_count;
                let countermove = context.cmtable.get(state.previous_move);
                let killer_1 = unsafe { state.killer_moves[0].assume_init() };
                let killer_2 = unsafe { state.killer_moves[1].assume_init() };

                if countermove != state.hash_move && countermove != killer_1 && countermove != killer_2 {
                    if ((1u64 << countermove.get_to()) & state.evasion_mask) != 0 && countermove.is_legal(&context.board) {
                        assert_fast!(state.moves_count < MAX_MOVES_COUNT);

                        state.moves[state.moves_count].write(countermove);
                        state.move_scores[state.moves_count].write(MOVEORD_COUNTERMOVE);
                        state.moves_count += 1;

                        dev!(context.stats.cmtable_legal_moves += 1);
                    } else {
                        dev!(context.stats.cmtable_illegal_moves += 1);
                    }
                }

                if original_moves_count != state.moves_count {
                    state.stage = MoveGenStage::Counters;
                } else {
                    state.stage = MoveGenStage::ReadyToGenerateQuiets;
                };

                dev!(context.stats.movegen_counters_stages += 1);
            }
            MoveGenStage::Counters => {
                let (r#move, score) = movesort::sort_next_move(&mut state.moves, &mut state.move_scores, state.move_index, state.moves_count);

                if score < MOVEORD_COUNTERMOVE {
                    state.stage = MoveGenStage::ReadyToGenerateQuiets;
                } else {
                    return Some((r#move, score));
                }
            }
            MoveGenStage::ReadyToGenerateQuiets => {
                let original_moves_count = state.moves_count;

                state.quiet_moves_start_index = state.move_index;
                state.moves_count = context.board.get_moves::<false>(&mut state.moves, state.moves_count, state.evasion_mask);
                state.stage = MoveGenStage::AllGenerated;

                assign_quiet_scores(context, state, original_moves_count);
                dev!(context.stats.movegen_quiets_stages += 1);
            }
            MoveGenStage::AllGenerated => {
                let (r#move, score) = movesort::sort_next_move(&mut state.moves, &mut state.move_scores, state.move_index, state.moves_count);

                if r#move == state.hash_move || score == MOVEORD_KILLER_MOVE_1 || score == MOVEORD_KILLER_MOVE_2 || score == MOVEORD_COUNTERMOVE {
                    state.move_index += 1;
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
fn assign_capture_scores(context: &SearchContext, state: &mut MoveGenState) {
    assert_fast!(state.moves_count < MAX_MOVES_COUNT);

    let mut attackers_cache = [0; 64];
    let mut defenders_cache = [0; 64];

    for move_index in 0..state.moves_count {
        let r#move = unsafe { state.moves[move_index].assume_init() };

        if r#move == state.hash_move {
            state.move_scores[move_index].write(MOVEORD_HASH_MOVE);
        } else if r#move.is_en_passant() {
            state.move_scores[move_index].write(MOVEORD_WINNING_CAPTURES_OFFSET);
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

            let see = see::get(attacking_piece, captured_piece, attackers, defenders);
            state.move_scores[move_index].write(if see >= 0 { see + MOVEORD_WINNING_CAPTURES_OFFSET } else { see + MOVEORD_LOSING_CAPTURES_OFFSET });
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
fn assign_quiet_scores(context: &SearchContext, state: &mut MoveGenState, start_index: usize) {
    assert_fast!(start_index < MAX_MOVES_COUNT);
    assert_fast!(start_index <= state.moves_count);
    assert_fast!(state.moves_count < MAX_MOVES_COUNT);

    let killer_moves = context.ktable.get(state.ply);
    let countermove = context.cmtable.get(state.previous_move);

    for move_index in start_index..state.moves_count {
        let r#move = unsafe { state.moves[move_index].assume_init() };

        if r#move == state.hash_move {
            state.move_scores[move_index].write(MOVEORD_HASH_MOVE);
        } else if r#move == countermove {
            state.move_scores[move_index].write(MOVEORD_COUNTERMOVE);
        } else if r#move.is_quiet() {
            let mut killer_move_found = false;
            for (index, &killer_move) in killer_moves.iter().enumerate() {
                if killer_move == r#move {
                    state.move_scores[move_index].write(MOVEORD_KILLER_MOVE_1 - (index as i16));
                    killer_move_found = true;
                    break;
                }
            }

            if killer_move_found {
                continue;
            }

            let value = context.htable.get(r#move.get_from(), r#move.get_to(), MOVEORD_HISTORY_MOVE) as i16;
            state.move_scores[move_index].write(value + MOVEORD_HISTORY_MOVE_OFFSET);
        } else if r#move.is_promotion() {
            state.move_scores[move_index].write(match r#move.get_promotion_piece() {
                QUEEN => MOVEORD_QUEEN_PROMOTION,
                ROOK => MOVEORD_ROOK_PROMOTION,
                BISHOP => MOVEORD_BISHOP_PROMOTION,
                KNIGHT => MOVEORD_KNIGHT_PROMOTION,
                _ => panic_fast!("Invalid value: fen={}, r#move.data={}", context.board, r#move.data),
            });
        } else if r#move.is_castling() {
            state.move_scores[move_index].write(MOVEORD_CASTLING);
        } else {
            panic_fast!("Sorting rule missing: fen={}, r#move.data={}", context.board, r#move.data);
        }
    }
}

impl Default for MoveGenState {
    fn default() -> Self {
        Self {
            moves: [MaybeUninit::uninit(); MAX_MOVES_COUNT],
            move_scores: [MaybeUninit::uninit(); MAX_MOVES_COUNT],
            stage: MoveGenStage::ReadyToCheckHashMove,
            quiet_moves_start_index: Default::default(),
            killer_moves: [MaybeUninit::uninit(); 2],
            move_index: Default::default(),
            move_number: Default::default(),
            moves_count: Default::default(),
            evasion_mask: Default::default(),
            hash_move: Default::default(),
            ply: Default::default(),
            friendly_king_checked: Default::default(),
            previous_move: Move::default(),
        }
    }
}
