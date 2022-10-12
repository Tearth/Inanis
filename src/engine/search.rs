use super::context::SearchContext;
use super::context::SearchResultLine;
use super::qsearch;
use super::*;
use crate::cache::search::TranspositionTableScoreType;
use crate::state::*;
use crate::tablebases::syzygy;
use crate::tablebases::WdlResult;
use crate::utils::bithelpers::BitHelpers;
use crate::utils::conditional_expression;
use crate::utils::rand;
use std::cmp;
use std::mem::MaybeUninit;
use std::sync::atomic::Ordering;

pub const RAZORING_MIN_DEPTH: i8 = 1;
pub const RAZORING_MAX_DEPTH: i8 = 4;
pub const RAZORING_DEPTH_MARGIN_BASE: i16 = 300;
pub const RAZORING_DEPTH_MARGIN_MULTIPLIER: i16 = 300;

pub const STATIC_NULL_MOVE_PRUNING_MIN_DEPTH: i8 = 1;
pub const STATIC_NULL_MOVE_PRUNING_MAX_DEPTH: i8 = 8;
pub const STATIC_NULL_MOVE_PRUNING_DEPTH_MARGIN_BASE: i16 = 150;
pub const STATIC_NULL_MOVE_PRUNING_DEPTH_MARGIN_MULTIPLIER: i16 = 150;

pub const NULL_MOVE_PRUNING_MIN_DEPTH: i8 = 2;
pub const NULL_MOVE_PRUNING_R_CHANGE_DEPTH: i8 = 5;
pub const NULL_MOVE_PRUNING_MIN_GAME_PHASE: u8 = 3;
pub const NULL_MOVE_PRUNING_MARGIN: i16 = 50;
pub const NULL_MOVE_PRUNING_SMALL_R: i8 = 2;
pub const NULL_MOVE_PRUNING_BIG_R: i8 = 3;

pub const LATE_MOVE_PRUNING_MIN_DEPTH: i8 = 1;
pub const LATE_MOVE_PRUNING_MAX_DEPTH: i8 = 4;
pub const LATE_MOVE_PRUNING_MOVE_INDEX_MARGIN_BASE: usize = 2;
pub const LATE_MOVE_PRUNING_MOVE_INDEX_MARGIN_MULTIPLIER: usize = 4;
pub const LATE_MOVE_PRUNING_MAX_SCORE: i16 = 0;

pub const LATE_MOVE_REDUCTION_MIN_DEPTH: i8 = 2;
pub const LATE_MOVE_REDUCTION_MAX_SCORE: i16 = 90;
pub const LATE_MOVE_REDUCTION_MIN_MOVE_INDEX: usize = 2;
pub const LATE_MOVE_REDUCTION_REDUCTION_BASE: usize = 1;
pub const LATE_MOVE_REDUCTION_REDUCTION_STEP: usize = 4;
pub const LATE_MOVE_REDUCTION_MAX_REDUCTION: i8 = 3;
pub const LATE_MOVE_REDUCTION_PV_MIN_MOVE_INDEX: usize = 2;
pub const LATE_MOVE_REDUCTION_PV_REDUCTION_BASE: usize = 1;
pub const LATE_MOVE_REDUCTION_PV_REDUCTION_STEP: usize = 8;
pub const LATE_MOVE_REDUCTION_PV_MAX_REDUCTION: i8 = 2;

pub const MOVE_ORDERING_HASH_MOVE: i16 = 10000;
pub const MOVE_ORDERING_WINNING_CAPTURES_OFFSET: i16 = 100;
pub const MOVE_ORDERING_KILLER_MOVE_1: i16 = 99;
pub const MOVE_ORDERING_KILLER_MOVE_2: i16 = 98;
pub const MOVE_ORDERING_QUEEN_PROMOTION: i16 = 95;
pub const MOVE_ORDERING_ROOK_PROMOTION: i16 = 94;
pub const MOVE_ORDERING_BISHOP_PROMOTION: i16 = 93;
pub const MOVE_ORDERING_KNIGHT_PROMOTION: i16 = 92;
pub const MOVE_ORDERING_CASTLING: i16 = 91;
pub const MOVE_ORDERING_HISTORY_MOVE: u8 = 180;
pub const MOVE_ORDERING_HISTORY_MOVE_OFFSET: i16 = -90;
pub const MOVE_ORDERING_LOSING_CAPTURES_OFFSET: i16 = -100;

pub const LAZY_SMP_NOISE: i16 = 10;

#[derive(std::cmp::PartialEq)]
enum MoveGeneratorStage {
    ReadyToCheckHashMove,
    HashMove,
    ReadyToGenerateCaptures,
    Captures,
    ReadyToGenerateKillerMoves,
    KillerMoves,
    ReadyToGenerateQuietMoves,
    AllGenerated,
}

/// Wrapper for the entry point of the regular search, look at `run_internal` for more information.
pub fn run<const DIAG: bool>(context: &mut SearchContext, depth: i8) {
    let king_checked = context.board.is_king_checked(context.board.active_color);
    run_internal::<true, true, DIAG>(context, depth, 0, MIN_ALPHA, MIN_BETA, true, king_checked);
}

/// Entry point of the regular search, with generic `ROOT` parameter indicating if this is the root node where the moves filterigh might happen, and `PV` parameter
/// determining if the current node is a PV (principal variation) node in the PVS framework. The implementation contains a typical alpha-beta approach, together with
/// a bunch of reduction and prunings to optimize search. The most important parameter here, `context` contains the current state of the search, board state,
/// statistics, and is passed by reference to all nodes. Besides obvious parameters like `depth`, `ply`, `alpha` and `beta`, there's also `allow_null_move`
/// which prevents two null move checks in a row, and `friendly_king_checked` which is used to share friendly king check status between nodes (it's always
/// calculated one depth earlier, as it's used as one of the LMR constraints). If `DIAG` is set to true, additional statistics will be gathered (with a small
/// performance penalty).
///
/// Search steps for PV node:
///  - test of abort flag
///  - test of initial constraints: abort flag, forced depth
///  - test if the enemy king is checked
///  - test if there's threefold repetition draw, fifty move rule draw or insufficient material draw
///  - switch to the quiescence search if the depth is equal to zero
///  - read from the transposition table, return score if possible or update alpha/beta (<https://www.chessprogramming.org/Transposition_Table>)
///  - generate evasion mask if the friendly king is checked
///  - main loop:
///     - filter moves (if `ROOT` is set)
///     - late move reduction (<https://www.chessprogramming.org/Late_Move_Reductions>)
///     - PVS framework (<https://www.chessprogramming.org/Principal_Variation_Search>)
///  - test if stalemate draw is detected
///  - update transposition table
///
/// Search steps for non-PV node:
///  - test of abort flag
///  - test of initial constraints: abort flag, forced depth, max nodes count
///  - test if the enemy king is checked
///  - test if there's threefold repetition draw, fifty move rule draw or insufficient material draw
///  - switch to the quiescence search if the depth is equal to zero
///  - read from the transposition table, return score if possible or update alpha/beta (<https://www.chessprogramming.org/Transposition_Table>)
///  - razoring (<https://www.chessprogramming.org/Razoring>)
///  - static null move pruning (<https://www.chessprogramming.org/Reverse_Futility_Pruning>)
///  - null move pruning (<https://www.chessprogramming.org/Null_Move_Pruning>)
///  - generate evasion mask if the friendly king is checked
///  - main loop:
///     - filter moves (if `ROOT` is set)
///     - late move pruning (<https://www.chessprogramming.org/Futility_Pruning#MoveCountBasedPruning>)
///     - late move reduction (<https://www.chessprogramming.org/Late_Move_Reductions>)
///     - PVS framework (<https://www.chessprogramming.org/Principal_Variation_Search>)
///  - test if stalemate draw is detected
///  - update transposition table
fn run_internal<const ROOT: bool, const PV: bool, const DIAG: bool>(
    context: &mut SearchContext,
    depth: i8,
    ply: u16,
    mut alpha: i16,
    mut beta: i16,
    mut allow_null_move: bool,
    friendly_king_checked: bool,
) -> i16 {
    if context.abort_flag.load(Ordering::Relaxed) {
        return INVALID_SCORE;
    }

    if context.forced_depth == 0 && context.max_nodes_count == 0 && (context.statistics.nodes_count & 8191) == 0 {
        if context.search_time_start.elapsed().unwrap().as_millis() > context.deadline as u128 {
            context.abort_flag.store(true, Ordering::Relaxed);
            return INVALID_SCORE;
        }
    }

    if PV && context.max_nodes_count != 0 {
        if context.statistics.nodes_count + context.statistics.q_nodes_count >= context.max_nodes_count {
            context.abort_flag.store(true, Ordering::Relaxed);
            return INVALID_SCORE;
        }
    }

    context.statistics.nodes_count += 1;

    if context.board.is_king_checked(context.board.active_color ^ 1) {
        conditional_expression!(DIAG, context.statistics.leafs_count += 1);
        return CHECKMATE_SCORE - (ply as i16);
    }

    if context.board.is_repetition_draw(if ROOT { 3 } else { 2 }) || context.board.is_fifty_move_rule_draw() || context.board.is_insufficient_material_draw() {
        conditional_expression!(DIAG, context.statistics.leafs_count += 1);
        return DRAW_SCORE;
    }

    if context.syzygy_enabled && depth >= context.syzygy_probe_depth && context.board.get_pieces_count() <= syzygy::probe::get_max_pieces_count() {
        if let Some(wdl) = syzygy::probe::get_wdl(&context.board) {
            context.statistics.tb_hits += 1;
            return match wdl {
                WdlResult::Win => TBMATE_SCORE - (ply as i16),
                WdlResult::Loss => -TBMATE_SCORE + (ply as i16),
                WdlResult::Draw => DRAW_SCORE,
            };
        }
    }

    if depth <= 0 {
        conditional_expression!(DIAG, context.statistics.leafs_count += 1);
        return qsearch::run::<DIAG>(context, depth, ply, alpha, beta);
    }

    let original_alpha = alpha;
    let mut tt_entry_found = false;
    let mut hash_move = Default::default();

    match context.transposition_table.get(context.board.hash, ply) {
        Some(entry) => {
            conditional_expression!(DIAG, context.statistics.tt_hits += 1);

            if entry.best_move != Default::default() {
                if entry.best_move.is_legal(&context.board) {
                    hash_move = entry.best_move;
                    conditional_expression!(DIAG, context.statistics.tt_legal_hashmoves += 1);
                } else {
                    conditional_expression!(DIAG, context.statistics.tt_illegal_hashmoves += 1);
                }
            }

            if !ROOT {
                if entry.depth >= depth as i8 {
                    tt_entry_found = true;
                    match entry.r#type {
                        TranspositionTableScoreType::UPPER_BOUND => {
                            beta = cmp::min(beta, entry.score);
                        }
                        TranspositionTableScoreType::LOWER_BOUND => {
                            alpha = cmp::max(alpha, entry.score);
                        }
                        _ => {
                            if !PV || entry.age == 0 {
                                if !context.board.is_repetition_draw(2) {
                                    conditional_expression!(DIAG, context.statistics.leafs_count += 1);
                                    return entry.score;
                                }
                            }
                        }
                    }

                    if alpha >= beta {
                        conditional_expression!(DIAG, context.statistics.leafs_count += 1);
                        conditional_expression!(DIAG, context.statistics.beta_cutoffs += 1);
                        return entry.score;
                    }
                } else {
                    match entry.r#type {
                        TranspositionTableScoreType::UPPER_BOUND | TranspositionTableScoreType::EXACT_SCORE => {
                            if entry.score < beta {
                                allow_null_move = false;
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
        None => {
            conditional_expression!(DIAG, context.statistics.tt_misses += 1);
        }
    };

    let mut lazy_evaluation = None;

    if razoring_can_be_applied::<PV>(depth, alpha, friendly_king_checked) {
        let margin = razoring_get_margin(depth);
        let lazy_evaluation_value = match lazy_evaluation {
            Some(value) => value,
            None => context.board.evaluate_lazy(context.board.active_color),
        };

        conditional_expression!(DIAG, context.statistics.razoring_attempts += 1);
        if lazy_evaluation_value + margin <= alpha {
            let score = qsearch::run::<DIAG>(context, depth, ply, alpha, beta);
            if score <= alpha {
                conditional_expression!(DIAG, context.statistics.leafs_count += 1);
                conditional_expression!(DIAG, context.statistics.razoring_accepted += 1);
                return score;
            } else {
                conditional_expression!(DIAG, context.statistics.razoring_rejected += 1);
            }
        }

        lazy_evaluation = Some(lazy_evaluation_value);
    }

    if static_null_move_pruning_can_be_applied::<PV>(depth, beta, friendly_king_checked) {
        let margin = static_null_move_pruning_get_margin(depth);
        let lazy_evaluation_value = match lazy_evaluation {
            Some(value) => value,
            None => context.board.evaluate_lazy(context.board.active_color),
        };

        conditional_expression!(DIAG, context.statistics.static_null_move_pruning_attempts += 1);
        if lazy_evaluation_value - margin >= beta {
            conditional_expression!(DIAG, context.statistics.leafs_count += 1);
            conditional_expression!(DIAG, context.statistics.static_null_move_pruning_accepted += 1);
            return lazy_evaluation_value - margin;
        } else {
            conditional_expression!(DIAG, context.statistics.static_null_move_pruning_rejected += 1);
        }

        lazy_evaluation = Some(lazy_evaluation_value);
    }

    if null_move_pruning_can_be_applied::<PV>(context, depth, beta, allow_null_move, friendly_king_checked) {
        let margin = NULL_MOVE_PRUNING_MARGIN;
        let lazy_evaluation_value = match lazy_evaluation {
            Some(value) => value,
            None => context.board.evaluate_lazy(context.board.active_color),
        };

        conditional_expression!(DIAG, context.statistics.null_move_pruning_attempts += 1);
        if lazy_evaluation_value + margin >= beta {
            let r = null_move_pruning_get_r(depth);

            context.board.make_null_move();
            let score = -run_internal::<false, false, DIAG>(context, depth - r - 1, ply + 1, -beta, -beta + 1, false, false);
            context.board.undo_null_move();

            if score >= beta {
                conditional_expression!(DIAG, context.statistics.leafs_count += 1);
                conditional_expression!(DIAG, context.statistics.null_move_pruning_accepted += 1);
                return score;
            } else {
                conditional_expression!(DIAG, context.statistics.null_move_pruning_rejected += 1);
            }
        }

        lazy_evaluation = Some(lazy_evaluation_value);
    }

    let mut best_score = -CHECKMATE_SCORE;
    let mut best_move = Default::default();
    let mut moves = [MaybeUninit::uninit(); MAX_MOVES_COUNT];
    let mut move_scores = [MaybeUninit::uninit(); MAX_MOVES_COUNT];
    let mut move_generator_stage = MoveGeneratorStage::ReadyToCheckHashMove;

    let mut move_index = 0;
    let mut move_number = 0;
    let mut moves_count = 0;
    let mut evasion_mask = 0;

    while let Some((r#move, score)) = get_next_move::<DIAG>(
        context,
        &mut move_generator_stage,
        &mut moves,
        &mut move_scores,
        &mut move_index,
        &mut move_number,
        &mut moves_count,
        &mut evasion_mask,
        hash_move,
        ply,
        friendly_king_checked,
    ) {
        if ROOT && !context.moves_to_search.is_empty() && !context.moves_to_search.contains(&r#move) {
            continue;
        }

        if late_move_pruning_can_be_applied::<PV>(depth, move_number, score, friendly_king_checked) {
            conditional_expression!(DIAG, context.statistics.late_move_pruning_accepted += 1);
            break;
        } else {
            conditional_expression!(DIAG, context.statistics.late_move_pruning_rejected += 1);
        }

        context.board.make_move(r#move);

        let king_checked = context.board.is_king_checked(context.board.active_color);
        let r = if late_move_reduction_can_be_applied(depth, r#move, move_number, score, friendly_king_checked, king_checked) {
            late_move_reduction_get_r::<PV>(move_number)
        } else {
            0
        };

        let score = if PV {
            if move_index == 0 {
                conditional_expression!(DIAG, context.statistics.pvs_full_window_searches += 1);
                -run_internal::<false, true, DIAG>(context, depth - 1, ply + 1, -beta, -alpha, true, king_checked)
            } else {
                let zero_window_score = -run_internal::<false, false, DIAG>(context, depth - r - 1, ply + 1, -alpha - 1, -alpha, true, king_checked);
                conditional_expression!(DIAG, context.statistics.pvs_zero_window_searches += 1);

                if zero_window_score > alpha && (alpha != beta - 1 || r > 0) {
                    conditional_expression!(DIAG, context.statistics.pvs_rejected_searches += 1);
                    -run_internal::<false, true, DIAG>(context, depth - 1, ply + 1, -beta, -alpha, true, king_checked)
                } else {
                    zero_window_score
                }
            }
        } else {
            let zero_window_score = -run_internal::<false, false, DIAG>(context, depth - r - 1, ply + 1, -beta, -alpha, true, king_checked);
            conditional_expression!(DIAG, context.statistics.pvs_zero_window_searches += 1);

            if zero_window_score > alpha && r > 0 {
                conditional_expression!(DIAG, context.statistics.pvs_rejected_searches += 1);
                -run_internal::<false, false, DIAG>(context, depth - 1, ply + 1, -beta, -alpha, true, king_checked)
            } else {
                zero_window_score
            }
        };

        context.board.undo_move(r#move);

        if score > best_score {
            best_score = cmp::max(best_score, score);
            alpha = cmp::max(alpha, best_score);
            best_move = r#move;

            if alpha >= beta {
                if r#move.is_quiet() {
                    context.killers_table.add(ply, r#move);
                    context.history_table.add(r#move.get_from(), r#move.get_to(), depth as u8);
                }

                conditional_expression!(DIAG, context.statistics.beta_cutoffs += 1);
                if move_number == 0 {
                    conditional_expression!(DIAG, context.statistics.perfect_cutoffs += 1);
                } else {
                    conditional_expression!(DIAG, context.statistics.non_perfect_cutoffs += 1);
                }

                break;
            }
        }

        if ROOT && context.multipv {
            context.board.make_move(best_move);
            if !context.board.is_king_checked(context.board.active_color ^ 1) {
                let mut pv_line = context.get_pv_line(&mut context.board.clone(), 0);
                pv_line.insert(0, best_move);

                context.multipv_lines.push(SearchResultLine::new(best_score, pv_line));
            }
            context.board.undo_move(best_move);

            alpha = original_alpha;
            best_score = -CHECKMATE_SCORE;
        }
    }

    // When no legal move is possible, but king is not checked, it's stalemate
    if best_score == -CHECKMATE_SCORE + (ply as i16) + 1 && !friendly_king_checked {
        return DRAW_SCORE;
    }

    if !tt_entry_found || alpha != original_alpha {
        let score_type = if alpha <= original_alpha {
            TranspositionTableScoreType::UPPER_BOUND
        } else if alpha >= beta {
            TranspositionTableScoreType::LOWER_BOUND
        } else {
            TranspositionTableScoreType::EXACT_SCORE
        };

        context.transposition_table.add(context.board.hash, alpha, best_move, depth as i8, ply, score_type, context.search_id);
        conditional_expression!(DIAG, context.statistics.tt_added += 1);
    }

    if ROOT && !context.multipv {
        let pv_line = context.get_pv_line(&mut context.board.clone(), 0);
        context.multipv_lines.push(SearchResultLine::new(best_score, pv_line));
    }

    best_score
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
    let mut attackers_cache = [0; 64];
    let mut defenders_cache = [0; 64];

    for move_index in start_index..moves_count {
        let r#move = unsafe { moves[move_index].assume_init() };

        if r#move == tt_move {
            move_scores[move_index].write(MOVE_ORDERING_HASH_MOVE);
            continue;
        }

        if r#move.is_en_passant() {
            move_scores[move_index].write(MOVE_ORDERING_WINNING_CAPTURES_OFFSET);
            continue;
        }

        let square = r#move.get_to() as usize;
        let attacking_piece = context.board.get_piece(r#move.get_from());
        let captured_piece = context.board.get_piece(r#move.get_to());

        let attackers = if attackers_cache[square] != 0 {
            attackers_cache[square]
        } else {
            attackers_cache[square] = context.board.get_attacking_pieces(context.board.active_color ^ 1, square);
            attackers_cache[square]
        };

        let defenders = if defenders_cache[square] != 0 {
            defenders_cache[square]
        } else {
            defenders_cache[square] = context.board.get_attacking_pieces(context.board.active_color, square);
            defenders_cache[square]
        };

        let see = context.board.see.get(attacking_piece, captured_piece, attackers, defenders, &context.board.evaluation_parameters);
        move_scores[move_index].write(if see >= 0 { see + MOVE_ORDERING_WINNING_CAPTURES_OFFSET } else { see + MOVE_ORDERING_LOSING_CAPTURES_OFFSET });
    }
}

/// Assigns quiet scores for `moves` by filling `move_scores` array with `moves_count` length (starting from `start_index`), based on current `context`.
/// If transposition table move is available, it's passed as `tt_move` too. Moves are prioritized as follows (from most important to the less ones):
///  - for transposition table move, assign [MOVE_ORDERING_HASH_MOVE]
///  - for every promotion (excluding these with capture), assign [MOVE_ORDERING_QUEEN_PROMOTION], [MOVE_ORDERING_ROOK_PROMOTION],
///    [MOVE_ORDERING_BISHOP_PROMOTION] or [MOVE_ORDERING_KNIGHT_PROMOTION]
///  - for every move found in killer table, assign [MOVE_ORDERING_KILLER_MOVE]
///  - for every castling, assign [MOVE_ORDERING_CASTLING]
///  - for every quiet move which wasn't categoried in other categories, assign score from history table + [MOVE_ORDERING_HISTORY_MOVE_OFFSET] + random noise
///    defined by [LAZY_SMP_NOISE] if Lazy SMP is enabled
fn assign_quiet_scores(
    context: &SearchContext,
    moves: &[MaybeUninit<Move>; MAX_MOVES_COUNT],
    move_scores: &mut [MaybeUninit<i16>; MAX_MOVES_COUNT],
    start_index: usize,
    moves_count: usize,
    tt_move: Move,
    ply: u16,
) {
    let killer_moves = context.killers_table.get(ply);
    for move_index in start_index..moves_count {
        let r#move = unsafe { moves[move_index].assume_init() };

        if r#move == tt_move {
            move_scores[move_index].write(MOVE_ORDERING_HASH_MOVE);
            continue;
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

            let mut value = context.history_table.get(r#move.get_from(), r#move.get_to(), MOVE_ORDERING_HISTORY_MOVE) as i16;
            if context.helper_thread && value + LAZY_SMP_NOISE < MOVE_ORDERING_HISTORY_MOVE as i16 {
                value += rand::i16(0..=LAZY_SMP_NOISE);
            }

            value += MOVE_ORDERING_HISTORY_MOVE_OFFSET;
            move_scores[move_index].write(value);

            continue;
        } else if r#move.is_promotion() {
            move_scores[move_index].write(match r#move.get_promotion_piece() {
                QUEEN => MOVE_ORDERING_QUEEN_PROMOTION,
                ROOK => MOVE_ORDERING_ROOK_PROMOTION,
                BISHOP => MOVE_ORDERING_BISHOP_PROMOTION,
                KNIGHT => MOVE_ORDERING_KNIGHT_PROMOTION,
                _ => panic!("Invalid value: fen={}, r#move.data={}", context.board.to_fen(), r#move.data),
            });

            continue;
        } else if r#move.is_castling() {
            move_scores[move_index].write(MOVE_ORDERING_CASTLING);
            continue;
        }

        panic!("Sorting rule missing: fen={}, r#move.data={}", context.board.to_fen(), r#move.data);
    }
}

/// Gets a next move to analyze. This function acts as pseudo-iterator and takes care about managing move generator stages, which is basically
/// a state machine (<https://en.wikipedia.org/wiki/Finite-state_machine>) with following rules:
///  - [MoveGeneratorStage::ReadyToCheckHashMove] - default state, returns transposition table move if possible
///  - [MoveGeneratorStage::ReadyToGenerateCaptures] - generates all captures in the position
///  - [MoveGeneratorStage::Captures] - returns subsequent elements until the end or score is less than [MOVE_ORDERING_WINNING_CAPTURES_OFFSET]
///  - [MoveGeneratorStage::ReadyToGenerateQuietMoves] - generates all quiet moves in the position
///  - [MoveGeneratorStage::AllGenerated] - returns subsequent elements until the end
///
/// Both [MoveGeneratorStage::ReadyToGenerateCaptures] and [MoveGeneratorStage::ReadyToGenerateQuietMoves] are generating moves and assigning scores
/// for move ordering purposes. If the last stage is set and there are no more moves, [None] is returned.
fn get_next_move<const DIAG: bool>(
    context: &mut SearchContext,
    stage: &mut MoveGeneratorStage,
    moves: &mut [MaybeUninit<Move>; MAX_MOVES_COUNT],
    move_scores: &mut [MaybeUninit<i16>; MAX_MOVES_COUNT],
    move_index: &mut usize,
    move_number: &mut usize,
    moves_count: &mut usize,
    evasion_mask: &mut u64,
    hash_move: Move,
    ply: u16,
    friendly_king_checked: bool,
) -> Option<(Move, i16)> {
    if matches!(*stage, MoveGeneratorStage::HashMove | MoveGeneratorStage::Captures | MoveGeneratorStage::KillerMoves | MoveGeneratorStage::AllGenerated) {
        *move_index += 1;
        *move_number += 1;
    }

    loop {
        match stage {
            MoveGeneratorStage::ReadyToCheckHashMove => {
                conditional_expression!(DIAG, context.statistics.move_generator_hash_move_stages += 1);

                if hash_move != Default::default() {
                    *stage = MoveGeneratorStage::HashMove;
                    *moves_count = 1;
                } else {
                    *stage = MoveGeneratorStage::ReadyToGenerateCaptures;
                }
            }
            MoveGeneratorStage::HashMove => {
                if move_index >= moves_count {
                    *stage = MoveGeneratorStage::ReadyToGenerateCaptures;
                    continue;
                }

                return Some((hash_move, MOVE_ORDERING_HASH_MOVE));
            }
            MoveGeneratorStage::ReadyToGenerateCaptures => {
                conditional_expression!(DIAG, context.statistics.move_generator_captures_stages += 1);

                *evasion_mask = if friendly_king_checked {
                    if context.board.pieces[context.board.active_color][KING] == 0 {
                        u64::MAX
                    } else {
                        let king_square_index = (context.board.pieces[context.board.active_color][KING]).bit_scan();
                        let occupancy = context.board.occupancy[WHITE] | context.board.occupancy[BLACK];

                        let queen_moves = context.board.magic.get_queen_moves(occupancy, king_square_index as usize);
                        let knight_moves = context.board.magic.get_knight_moves(king_square_index as usize, &context.board.patterns);

                        queen_moves | knight_moves
                    }
                } else {
                    u64::MAX
                };

                *moves_count = context.board.get_moves::<true>(moves, 0, *evasion_mask);
                *move_index = 0;

                if *moves_count == 0 {
                    *stage = MoveGeneratorStage::ReadyToGenerateKillerMoves;
                    continue;
                }

                assign_capture_scores(context, moves, move_scores, 0, *moves_count, hash_move);
                *stage = MoveGeneratorStage::Captures;
            }
            MoveGeneratorStage::Captures => {
                if move_index >= moves_count {
                    *stage = MoveGeneratorStage::ReadyToGenerateKillerMoves;
                    continue;
                }

                let (r#move, score) = sort_next_move(moves, move_scores, *move_index, *moves_count);

                if r#move == hash_move {
                    *move_index += 1;
                    continue;
                }

                if score < MOVE_ORDERING_WINNING_CAPTURES_OFFSET {
                    *stage = MoveGeneratorStage::ReadyToGenerateKillerMoves;
                    continue;
                }

                return Some((r#move, score));
            }
            MoveGeneratorStage::ReadyToGenerateKillerMoves => {
                conditional_expression!(DIAG, context.statistics.move_generator_killers_stages += 1);

                let original_moves_count = *moves_count;
                let killer_moves = context.killers_table.get(ply);

                for (index, &killer_move) in killer_moves.iter().enumerate() {
                    if killer_move != hash_move {
                        if ((1u64 << killer_move.get_to()) & *evasion_mask) != 0 && killer_move.is_legal(&context.board) {
                            moves[*moves_count].write(killer_move);
                            move_scores[*moves_count].write(MOVE_ORDERING_KILLER_MOVE_1 - (index as i16));
                            *moves_count += 1;

                            conditional_expression!(DIAG, context.statistics.killers_table_legal_moves += 1);
                        } else {
                            conditional_expression!(DIAG, context.statistics.killers_table_illegal_moves += 1);
                        }
                    }
                }

                *stage = if original_moves_count != *moves_count { MoveGeneratorStage::KillerMoves } else { MoveGeneratorStage::ReadyToGenerateQuietMoves }
            }
            MoveGeneratorStage::KillerMoves => {
                if move_index >= moves_count {
                    *stage = MoveGeneratorStage::ReadyToGenerateQuietMoves;
                    continue;
                }

                let (r#move, score) = sort_next_move(moves, move_scores, *move_index, *moves_count);

                if r#move == hash_move {
                    *move_index += 1;
                    continue;
                }

                if score < MOVE_ORDERING_KILLER_MOVE_2 {
                    *stage = MoveGeneratorStage::ReadyToGenerateQuietMoves;
                    continue;
                }

                return Some((r#move, score));
            }
            MoveGeneratorStage::ReadyToGenerateQuietMoves => {
                conditional_expression!(DIAG, context.statistics.move_generator_quiet_moves_stages += 1);
                let original_moves_count = *moves_count;

                *moves_count = context.board.get_moves::<false>(moves, *moves_count, *evasion_mask);
                *stage = MoveGeneratorStage::AllGenerated;

                assign_quiet_scores(context, moves, move_scores, original_moves_count, *moves_count, hash_move, ply);
            }
            MoveGeneratorStage::AllGenerated => {
                if move_index >= moves_count {
                    return None;
                }

                let (r#move, score) = sort_next_move(moves, move_scores, *move_index, *moves_count);

                if r#move == hash_move {
                    *move_index += 1;
                    continue;
                }

                if score == MOVE_ORDERING_KILLER_MOVE_1 || score == MOVE_ORDERING_KILLER_MOVE_2 {
                    *move_index += 1;
                    continue;
                }

                return Some((r#move, score));
            }
        }
    }
}

/// The main idea of the razoring is to detect and prune all nodes, which (based on lazy evaluation) are hopeless compared to the current alpha and
/// the chance to improve the score is too small to spend time here. To make it more safe and not to skip positions where we're somewhere in the
/// middle of capture sequence, there's the quiescence search performed to verify if the final score is still below alpha - margin.
///
/// Conditions:
///  - only non-PV nodes
///  - depth >= [RAZORING_MIN_DEPTH]
///  - depth <= [RAZORING_MAX_DEPTH]
///  - alpha is not a mate score
///  - friendly king is not checked
fn razoring_can_be_applied<const PV: bool>(depth: i8, alpha: i16, friendly_king_checked: bool) -> bool {
    !PV && depth >= RAZORING_MIN_DEPTH && depth <= RAZORING_MAX_DEPTH && !is_score_near_checkmate(alpha) && !friendly_king_checked
}

/// Gets the razoring margin, based on `depth`. The further from the horizon we are, the more margin we should take to determine if node can be pruned.
fn razoring_get_margin(depth: i8) -> i16 {
    RAZORING_DEPTH_MARGIN_BASE + ((depth - RAZORING_MIN_DEPTH) as i16) * RAZORING_DEPTH_MARGIN_MULTIPLIER
}

/// The main idea of the static null move pruning (also called as reverse futility pruning) is to prune all nodes, which (based on lazy evaluation) are too
/// good compared to the current beta, and will very likely be a cut-node. To save time, we skip move loop entirely and return beta + some margin score.
/// The concept is very similar to null move pruning, but without performing any search.
///
/// Conditions:
///  - only non-PV nodes
///  - depth >= [STATIC_NULL_MOVE_PRUNING_MIN_DEPTH]
///  - depth <= [STATIC_NULL_MOVE_PRUNING_MAX_DEPTH]
///  - beta is not a mate score
///  - friendly king is not checked
fn static_null_move_pruning_can_be_applied<const PV: bool>(depth: i8, beta: i16, friendly_king_checked: bool) -> bool {
    !PV && depth >= STATIC_NULL_MOVE_PRUNING_MIN_DEPTH
        && depth <= STATIC_NULL_MOVE_PRUNING_MAX_DEPTH
        && !is_score_near_checkmate(beta)
        && !friendly_king_checked
}

/// Gets the static null move pruning margin, based on `depth`. The further from the horizon we are, the more margin should we take to determine
/// if node can be pruned.
fn static_null_move_pruning_get_margin(depth: i8) -> i16 {
    STATIC_NULL_MOVE_PRUNING_DEPTH_MARGIN_BASE + ((depth - STATIC_NULL_MOVE_PRUNING_MIN_DEPTH) as i16) * STATIC_NULL_MOVE_PRUNING_DEPTH_MARGIN_MULTIPLIER
}

/// The main idea of the null move pruning is to prune all nodes, for which the search gives us score above beta even if we skip a move (which allows
/// the opposite color to make two of them in a row). This is based on the null move observation, which says that there's always a better alternative than
/// doing nothing (except zugzwang).
///
/// Conditions:
///  - only non-PV nodes
///  - depth >= [NULL_MOVE_PRUNING_MIN_DEPTH]
///  - game phase is not indicating endgame
///  - beta score is not a mate score
///  - friendly king is not checked
///  - this is not the second null move in a row
fn null_move_pruning_can_be_applied<const PV: bool>(
    context: &mut SearchContext,
    depth: i8,
    beta: i16,
    allow_null_move: bool,
    friendly_king_checked: bool,
) -> bool {
    !PV && depth >= NULL_MOVE_PRUNING_MIN_DEPTH
        && context.board.game_phase > NULL_MOVE_PRUNING_MIN_GAME_PHASE
        && !is_score_near_checkmate(beta)
        && !friendly_king_checked
        && allow_null_move
}

/// Gets the null move pruning depth reduction, called R, based on `depth`. It returns [NULL_MOVE_PRUNING_SMALL_R] if `depth` is less or equal
/// to [NULL_MOVE_PRUNING_R_CHANGE_DEPTH], otherwise [NULL_MOVE_PRUNING_BIG_R].
fn null_move_pruning_get_r(depth: i8) -> i8 {
    if depth <= NULL_MOVE_PRUNING_R_CHANGE_DEPTH {
        NULL_MOVE_PRUNING_SMALL_R
    } else {
        NULL_MOVE_PRUNING_BIG_R
    }
}

/// The main idea of the late move pruning is to prune all nodes, which are near the horizon and were scored low by the history table.
/// We assume here, that there's a little chance that move being near the end of the list will improve score, so there's no point of spending time here.
///
/// Conditions:
///  - only non-PV nodes
///  - depth >= [LATE_MOVE_PRUNING_MIN_DEPTH]
///  - depth <= [LATE_MOVE_PRUNING_MAX_DEPTH]
///  - move index >= [LATE_MOVE_PRUNING_MOVE_INDEX_MARGIN_BASE] + some margin depending on `depth`
///  - move score <= [LATE_MOVE_PRUNING_MAX_SCORE]
///  - friendly king is not checked
fn late_move_pruning_can_be_applied<const PV: bool>(depth: i8, move_index: usize, move_score: i16, friendly_king_checked: bool) -> bool {
    !PV && depth >= LATE_MOVE_PRUNING_MIN_DEPTH
        && depth <= LATE_MOVE_PRUNING_MAX_DEPTH
        && move_index >= LATE_MOVE_PRUNING_MOVE_INDEX_MARGIN_BASE + (depth as usize - 1) * LATE_MOVE_PRUNING_MOVE_INDEX_MARGIN_MULTIPLIER
        && move_score <= LATE_MOVE_PRUNING_MAX_SCORE
        && !friendly_king_checked
}

/// The main idea of the late move reduction is to reduce search depth of all quiet moves, which aren't promising and with high chance won't improve score.
/// This is the least risky type of pruning (used inside PVS framework which cares about re-search when the move is better than expected),
/// so it's also applied in PV nodes.
///
/// Conditions:
///  - depth >= [LATE_MOVE_REDUCTION_MIN_DEPTH]
///  - move index >= [LATE_MOVE_REDUCTION_MIN_MOVE_INDEX]
///  - move score <= [LATE_MOVE_REDUCTION_MAX_SCORE]
///  - move is quiet
///  - friendly king is not checked
///  - enemy king is not checked
fn late_move_reduction_can_be_applied(
    depth: i8,
    r#move: Move,
    move_index: usize,
    move_score: i16,
    friendly_king_checked: bool,
    enemy_king_checked: bool,
) -> bool {
    depth >= LATE_MOVE_REDUCTION_MIN_DEPTH
        && move_index >= LATE_MOVE_REDUCTION_MIN_MOVE_INDEX
        && move_score <= LATE_MOVE_REDUCTION_MAX_SCORE
        && r#move.is_quiet()
        && !friendly_king_checked
        && !enemy_king_checked
}

/// Gets the late move depth reduction, called R, based on `move_index`. The lower the move was scored, the larger reduction will be returned.
fn late_move_reduction_get_r<const PV: bool>(move_index: usize) -> i8 {
    if PV {
        cmp::min(
            LATE_MOVE_REDUCTION_PV_MAX_REDUCTION,
            (LATE_MOVE_REDUCTION_PV_REDUCTION_BASE + (move_index - LATE_MOVE_REDUCTION_PV_MIN_MOVE_INDEX) / LATE_MOVE_REDUCTION_PV_REDUCTION_STEP) as i8,
        )
    } else {
        cmp::min(
            LATE_MOVE_REDUCTION_MAX_REDUCTION,
            (LATE_MOVE_REDUCTION_REDUCTION_BASE + (move_index - LATE_MOVE_REDUCTION_MIN_MOVE_INDEX) / LATE_MOVE_REDUCTION_REDUCTION_STEP) as i8,
        )
    }
}
