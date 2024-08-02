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
use crate::utils::parameter;
use crate::utils::rand;
use std::cmp;
use std::mem::MaybeUninit;
use std::sync::atomic::Ordering;

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

pub const LAZY_SMP_NOISE: i16 = 10;

#[derive(std::cmp::PartialEq)]
enum MoveGeneratorStage {
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

/// Wrapper for the entry point of the regular search, look at `run_internal` for more information.
pub fn run<const DIAG: bool>(context: &mut SearchContext, depth: i8) {
    let king_checked = context.board.is_king_checked(context.board.active_color);
    run_internal::<true, true, DIAG>(context, depth, 0, MIN_ALPHA, MIN_BETA, true, king_checked, Move::default());
}

/// Entry point of the regular search, with generic `ROOT` parameter indicating if this is the root node where the moves filterigh might happen, and `PV` parameter
/// determining if the current node is a PV (principal variation) node in the PVS framework. The implementation contains a typical alpha-beta approach, together with
/// a bunch of reductions and prunings to optimize search. The most important parameter here, `context`, contains the current state of the search, board state,
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
///  - check extensions (<https://www.chessprogramming.org/Check_Extensions>)
///  - switch to the quiescence search if the depth is equal to zero
///  - read from the transposition table, return score if possible or update alpha/beta (<https://www.chessprogramming.org/Transposition_Table>)
///  - internal iterative reduction (<https://chessprogrammingwiki.netlify.app/internal_iterative_reductions/>)
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
///  - check extensions (<https://www.chessprogramming.org/Check_Extensions>)
///  - switch to the quiescence search if the depth is equal to zero
///  - read from the transposition table, return score if possible or update alpha/beta (<https://www.chessprogramming.org/Transposition_Table>)
///  - internal iterative reduction (<https://chessprogrammingwiki.netlify.app/internal_iterative_reductions/>)
///  - razoring (<https://www.chessprogramming.org/Razoring>)
///  - static null move pruning (<https://www.chessprogramming.org/Reverse_Futility_Pruning>)
///  - null move pruning (<https://www.chessprogramming.org/Null_Move_Pruning>)
///  - main loop:
///     - filter moves (if `ROOT` is set)
///     - late move pruning (<https://www.chessprogramming.org/Futility_Pruning#MoveCountBasedPruning>)
///     - late move reduction (<https://www.chessprogramming.org/Late_Move_Reductions>)
///     - PVS framework (<https://www.chessprogramming.org/Principal_Variation_Search>)
///  - test if stalemate draw is detected
///  - update transposition table
fn run_internal<const ROOT: bool, const PV: bool, const DIAG: bool>(
    context: &mut SearchContext,
    mut depth: i8,
    ply: u16,
    mut alpha: i16,
    mut beta: i16,
    mut allow_null_move: bool,
    friendly_king_checked: bool,
    previous_move: Move,
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

        // The position where both kings are checked is illegal, it will be filtered after returning invalid score
        if friendly_king_checked {
            return INVALID_SCORE;
        } else {
            return CHECKMATE_SCORE - (ply as i16);
        }
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

    if check_extensions_can_be_applied(friendly_king_checked) {
        depth += check_extensions_get_e();
    }

    if depth <= 0 {
        conditional_expression!(DIAG, context.statistics.leafs_count += 1);
        return qsearch::run::<DIAG>(context, ply, alpha, beta);
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
                if entry.depth >= depth {
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

    if iir_can_be_applied(context, depth, hash_move) {
        depth -= iir_get_r(context, depth);
    }

    let mut lazy_evaluation = None;

    if razoring_can_be_applied::<PV>(context, depth, alpha, friendly_king_checked) {
        let margin = razoring_get_margin(context, depth);
        let lazy_evaluation_value = match lazy_evaluation {
            Some(value) => value,
            None => context.board.evaluate_lazy(context.board.active_color),
        };

        conditional_expression!(DIAG, context.statistics.razoring_attempts += 1);
        if lazy_evaluation_value + margin <= alpha {
            let score = qsearch::run::<DIAG>(context, ply, alpha, beta);
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

    if snmp_can_be_applied::<PV>(context, depth, beta, friendly_king_checked) {
        let margin = snmp_get_margin(context, depth);
        let lazy_evaluation_value = match lazy_evaluation {
            Some(value) => value,
            None => context.board.evaluate_lazy(context.board.active_color),
        };

        conditional_expression!(DIAG, context.statistics.snmp_attempts += 1);
        if lazy_evaluation_value - margin >= beta {
            conditional_expression!(DIAG, context.statistics.leafs_count += 1);
            conditional_expression!(DIAG, context.statistics.snmp_accepted += 1);
            return lazy_evaluation_value - margin;
        } else {
            conditional_expression!(DIAG, context.statistics.snmp_rejected += 1);
        }

        lazy_evaluation = Some(lazy_evaluation_value);
    }

    if nmp_can_be_applied::<PV>(context, depth, beta, allow_null_move, friendly_king_checked) {
        let margin = parameter!(context.parameters.nmp_margin);
        let lazy_evaluation_value = match lazy_evaluation {
            Some(value) => value,
            None => context.board.evaluate_lazy(context.board.active_color),
        };

        conditional_expression!(DIAG, context.statistics.nmp_attempts += 1);
        if lazy_evaluation_value + margin >= beta {
            let r = nmp_get_r(context, depth);

            context.board.make_null_move();
            let score = -run_internal::<false, false, DIAG>(context, depth - r - 1, ply + 1, -beta, -beta + 1, false, false, Move::default());
            context.board.undo_null_move();

            if score >= beta {
                conditional_expression!(DIAG, context.statistics.leafs_count += 1);
                conditional_expression!(DIAG, context.statistics.nmp_accepted += 1);
                return score;
            } else {
                conditional_expression!(DIAG, context.statistics.nmp_rejected += 1);
            }
        }

        lazy_evaluation = Some(lazy_evaluation_value);
    }

    let mut best_score = -CHECKMATE_SCORE;
    let mut best_move = Default::default();
    let mut moves = [MaybeUninit::uninit(); MAX_MOVES_COUNT];
    let mut move_scores = [MaybeUninit::uninit(); MAX_MOVES_COUNT];
    let mut move_generator_stage = MoveGeneratorStage::ReadyToCheckHashMove;
    let mut quiet_moves_start_index = 0;
    let mut killer_moves = [MaybeUninit::uninit(); 2];

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
        previous_move,
        &mut quiet_moves_start_index,
        &mut killer_moves,
    ) {
        if ROOT && !context.moves_to_search.is_empty() && !context.moves_to_search.contains(&r#move) {
            continue;
        }

        if lmp_can_be_applied::<PV>(context, depth, move_number, score, friendly_king_checked) {
            conditional_expression!(DIAG, context.statistics.lmp_accepted += 1);
            break;
        } else {
            conditional_expression!(DIAG, context.statistics.lmp_rejected += 1);
        }

        context.board.make_move(r#move);

        let king_checked = context.board.is_king_checked(context.board.active_color);
        let r = if lmr_can_be_applied::<PV>(context, depth, r#move, move_number, score, friendly_king_checked, king_checked) {
            lmr_get_r::<PV>(context, move_number)
        } else {
            0
        };

        let score = if PV {
            if move_index == 0 {
                conditional_expression!(DIAG, context.statistics.pvs_full_window_searches += 1);
                -run_internal::<false, true, DIAG>(context, depth - 1, ply + 1, -beta, -alpha, true, king_checked, r#move)
            } else {
                let zero_window_score = -run_internal::<false, false, DIAG>(context, depth - r - 1, ply + 1, -alpha - 1, -alpha, true, king_checked, r#move);
                conditional_expression!(DIAG, context.statistics.pvs_zero_window_searches += 1);

                if zero_window_score > alpha && (alpha != beta - 1 || r > 0) && zero_window_score != -INVALID_SCORE {
                    conditional_expression!(DIAG, context.statistics.pvs_rejected_searches += 1);
                    -run_internal::<false, true, DIAG>(context, depth - 1, ply + 1, -beta, -alpha, true, king_checked, r#move)
                } else {
                    zero_window_score
                }
            }
        } else {
            let zero_window_score = -run_internal::<false, false, DIAG>(context, depth - r - 1, ply + 1, -beta, -alpha, true, king_checked, r#move);
            conditional_expression!(DIAG, context.statistics.pvs_zero_window_searches += 1);

            if zero_window_score > alpha && r > 0 && zero_window_score != -INVALID_SCORE {
                conditional_expression!(DIAG, context.statistics.pvs_rejected_searches += 1);
                -run_internal::<false, false, DIAG>(context, depth - 1, ply + 1, -beta, -alpha, true, king_checked, r#move)
            } else {
                zero_window_score
            }
        };

        context.board.undo_move(r#move);

        if score == -INVALID_SCORE {
            continue;
        }

        if score > best_score {
            best_score = cmp::max(best_score, score);
            alpha = cmp::max(alpha, best_score);
            best_move = r#move;

            if alpha >= beta {
                if r#move.is_quiet() {
                    context.killers_table.add(ply, r#move);
                    context.history_table.add(r#move.get_from(), r#move.get_to(), depth as u8);
                    context.countermoves_table.add(previous_move, r#move);

                    if move_generator_stage == MoveGeneratorStage::AllGenerated {
                        for i in quiet_moves_start_index..moves_count {
                            let move_from_list = unsafe { moves[i].assume_init() };
                            if move_from_list.is_quiet() && move_from_list != best_move {
                                context.history_table.punish(move_from_list.get_from(), move_from_list.get_to(), depth as u8);
                            }
                        }
                    }
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
                let mut pv_line = context.transposition_table.get_pv_line(&mut context.board.clone(), 0);
                pv_line.insert(0, best_move);

                context.multipv_lines.push(SearchResultLine::new(best_score, pv_line));
            }
            context.board.undo_move(best_move);

            alpha = original_alpha;
            best_score = -CHECKMATE_SCORE;
        }
    }

    // When no legal move is possible, but king is not checked, it's a stalemate
    if best_score == -CHECKMATE_SCORE + (ply as i16) + 1 && !friendly_king_checked {
        return DRAW_SCORE;
    }

    if (!tt_entry_found || alpha != original_alpha) && !context.abort_flag.load(Ordering::Relaxed) {
        let score_type = if alpha <= original_alpha {
            TranspositionTableScoreType::UPPER_BOUND
        } else if alpha >= beta {
            TranspositionTableScoreType::LOWER_BOUND
        } else {
            TranspositionTableScoreType::EXACT_SCORE
        };

        context.transposition_table.add(context.board.hash, alpha, best_move, depth, ply, score_type, context.search_id);
        conditional_expression!(DIAG, context.statistics.tt_added += 1);
    }

    if ROOT && !context.multipv {
        let pv_line = context.transposition_table.get_pv_line(&mut context.board.clone(), 0);
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

/// Assigns quiet scores for `moves` by filling `move_scores` array with `moves_count` length (starting from `start_index`), based on current `context`.
/// If transposition table move is available, it's passed as `tt_move` too. Moves are prioritized as follows (from most important to the less ones):
///  - for transposition table move, assign [MOVE_ORDERING_HASH_MOVE]
///  - for every promotion (excluding these with capture), assign [MOVE_ORDERING_QUEEN_PROMOTION], [MOVE_ORDERING_ROOK_PROMOTION],
///    [MOVE_ORDERING_BISHOP_PROMOTION] or [MOVE_ORDERING_KNIGHT_PROMOTION]
///  - for every move found in killer table, assign [MOVE_ORDERING_KILLER_MOVE_1] or [MOVE_ORDERING_KILLER_MOVE_2]
///  - for every castling, assign [MOVE_ORDERING_CASTLING]
///  - for every quiet move which didn't fit in other categories, assign score from history table + [MOVE_ORDERING_HISTORY_MOVE_OFFSET] + random noise
///    defined by [LAZY_SMP_NOISE] if Lazy SMP is enabled
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
    let killer_moves = context.killers_table.get(ply);
    let countermove = context.countermoves_table.get(previous_move);

    for move_index in start_index..moves_count {
        let r#move = unsafe { moves[move_index].assume_init() };

        if r#move == tt_move {
            move_scores[move_index].write(MOVE_ORDERING_HASH_MOVE);
            continue;
        } else if r#move == countermove {
            move_scores[move_index].write(MOVE_ORDERING_COUNTERMOVE);
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
                _ => panic!("Invalid value: fen={}, r#move.data={}", context.board, r#move.data),
            });

            continue;
        } else if r#move.is_castling() {
            move_scores[move_index].write(MOVE_ORDERING_CASTLING);
            continue;
        }

        panic!("Sorting rule missing: fen={}, r#move.data={}", context.board, r#move.data);
    }
}

/// Gets a next move to analyze. This function acts as pseudo-iterator and takes care about managing move generator stages, which is basically
/// a state machine (<https://en.wikipedia.org/wiki/Finite-state_machine>) with following rules:
///  - [MoveGeneratorStage::ReadyToCheckHashMove] - default state, prepares hash move if possible
///  - [MoveGeneratorStage::HashMove] - returns hashmove if possible
///  - [MoveGeneratorStage::ReadyToGenerateCaptures] - generates all captures in the position
///  - [MoveGeneratorStage::Captures] - returns subsequent elements until the end or score is less than [MOVE_ORDERING_WINNING_CAPTURES_OFFSET]
///  - [MoveGeneratorStage::ReadyToGenerateKillers] -
///  - [MoveGeneratorStage::Killers] -
///  - [MoveGeneratorStage::ReadyToGenerateCounters] -
///  - [MoveGeneratorStage::Counters] -
///  - [MoveGeneratorStage::ReadyToGenerateQuiets] - generates all quiet moves in the position
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
    previous_move: Move,
    quiet_moves_start_index: &mut usize,
    killer_moves_cache: &mut [MaybeUninit<Move>; 2],
) -> Option<(Move, i16)> {
    if matches!(
        *stage,
        MoveGeneratorStage::HashMove
            | MoveGeneratorStage::Captures
            | MoveGeneratorStage::Killers
            | MoveGeneratorStage::Counters
            | MoveGeneratorStage::AllGenerated
    ) {
        *move_index += 1;
        *move_number += 1;
    }

    loop {
        if move_index >= moves_count {
            match *stage {
                MoveGeneratorStage::HashMove => *stage = MoveGeneratorStage::ReadyToGenerateCaptures,
                MoveGeneratorStage::Captures => *stage = MoveGeneratorStage::ReadyToGenerateKillers,
                MoveGeneratorStage::Killers => *stage = MoveGeneratorStage::ReadyToGenerateCounters,
                MoveGeneratorStage::Counters => *stage = MoveGeneratorStage::ReadyToGenerateQuiets,
                MoveGeneratorStage::AllGenerated => return None,
                _ => {}
            }
        }

        match stage {
            MoveGeneratorStage::ReadyToCheckHashMove => {
                if hash_move != Default::default() {
                    *moves_count = 1;
                    *stage = MoveGeneratorStage::HashMove;
                } else {
                    *stage = MoveGeneratorStage::ReadyToGenerateCaptures;
                }

                conditional_expression!(DIAG, context.statistics.move_generator_hash_move_stages += 1);
            }
            MoveGeneratorStage::HashMove => {
                return Some((hash_move, MOVE_ORDERING_HASH_MOVE));
            }
            MoveGeneratorStage::ReadyToGenerateCaptures => {
                *evasion_mask = if friendly_king_checked {
                    let king_square = (context.board.pieces[context.board.active_color][KING]).bit_scan();
                    let occupancy_bb = context.board.occupancy[WHITE] | context.board.occupancy[BLACK];

                    let queen_moves_bb = context.board.magic.get_queen_moves(occupancy_bb, king_square);
                    let knight_moves_bb = context.board.magic.get_knight_moves(king_square, &context.board.patterns);

                    queen_moves_bb | knight_moves_bb
                } else {
                    u64::MAX
                };

                *move_index = 0;
                *moves_count = context.board.get_moves::<true>(moves, 0, *evasion_mask);

                if *moves_count == 0 {
                    *stage = MoveGeneratorStage::ReadyToGenerateKillers;
                } else {
                    *stage = MoveGeneratorStage::Captures;
                    assign_capture_scores(context, moves, move_scores, 0, *moves_count, hash_move);
                }

                conditional_expression!(DIAG, context.statistics.move_generator_captures_stages += 1);
            }
            MoveGeneratorStage::Captures => {
                let (r#move, score) = sort_next_move(moves, move_scores, *move_index, *moves_count);

                if r#move == hash_move {
                    *move_index += 1;
                } else if score < MOVE_ORDERING_WINNING_CAPTURES_OFFSET {
                    *stage = MoveGeneratorStage::ReadyToGenerateKillers;
                } else {
                    return Some((r#move, score));
                }
            }
            MoveGeneratorStage::ReadyToGenerateKillers => {
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

                    killer_moves_cache[index].write(killer_move);
                }

                *stage = if original_moves_count != *moves_count { MoveGeneratorStage::Killers } else { MoveGeneratorStage::ReadyToGenerateCounters };
                conditional_expression!(DIAG, context.statistics.move_generator_killers_stages += 1);
            }
            MoveGeneratorStage::Killers => {
                let (r#move, score) = sort_next_move(moves, move_scores, *move_index, *moves_count);

                if score < MOVE_ORDERING_KILLER_MOVE_2 {
                    *stage = MoveGeneratorStage::ReadyToGenerateCounters;
                } else {
                    return Some((r#move, score));
                }
            }
            MoveGeneratorStage::ReadyToGenerateCounters => {
                let original_moves_count = *moves_count;
                let countermove = context.countermoves_table.get(previous_move);

                if countermove != hash_move
                    && countermove != unsafe { killer_moves_cache[0].assume_init() }
                    && countermove != unsafe { killer_moves_cache[1].assume_init() }
                {
                    if ((1u64 << countermove.get_to()) & *evasion_mask) != 0 && countermove.is_legal(&context.board) {
                        moves[*moves_count].write(countermove);
                        move_scores[*moves_count].write(MOVE_ORDERING_COUNTERMOVE);
                        *moves_count += 1;

                        conditional_expression!(DIAG, context.statistics.countermoves_table_legal_moves += 1);
                    } else {
                        conditional_expression!(DIAG, context.statistics.countermoves_table_illegal_moves += 1);
                    }
                }

                *stage = if original_moves_count != *moves_count { MoveGeneratorStage::Counters } else { MoveGeneratorStage::ReadyToGenerateQuiets };
                conditional_expression!(DIAG, context.statistics.move_generator_counters_stages += 1);
            }
            MoveGeneratorStage::Counters => {
                let (r#move, score) = sort_next_move(moves, move_scores, *move_index, *moves_count);

                if score < MOVE_ORDERING_COUNTERMOVE {
                    *stage = MoveGeneratorStage::ReadyToGenerateQuiets;
                } else {
                    return Some((r#move, score));
                }
            }
            MoveGeneratorStage::ReadyToGenerateQuiets => {
                let original_moves_count = *moves_count;

                *quiet_moves_start_index = *move_index;
                *moves_count = context.board.get_moves::<false>(moves, *moves_count, *evasion_mask);
                *stage = MoveGeneratorStage::AllGenerated;

                assign_quiet_scores(context, moves, move_scores, original_moves_count, *moves_count, hash_move, previous_move, ply);
                conditional_expression!(DIAG, context.statistics.move_generator_quiets_stages += 1);
            }
            MoveGeneratorStage::AllGenerated => {
                let (r#move, score) = sort_next_move(moves, move_scores, *move_index, *moves_count);

                if r#move == hash_move || score == MOVE_ORDERING_KILLER_MOVE_1 || score == MOVE_ORDERING_KILLER_MOVE_2 || score == MOVE_ORDERING_COUNTERMOVE {
                    *move_index += 1;
                } else {
                    return Some((r#move, score));
                }
            }
        }
    }
}

/// The main idea of the check extensions is to extend search when there's a check. Because it's a forced move, we assume that a lot is going on
/// in that branch and it's a good idea to search it deeper so we avoid horizon effects.
///
/// Conditions:
///  - friendly king is checked
fn check_extensions_can_be_applied(friendly_king_checked: bool) -> bool {
    friendly_king_checked
}

/// Gets the check extensions, for now it's constant 1.
fn check_extensions_get_e() -> i8 {
    1
}

/// The main idea of the internal iterative reduction is that nodes without hash moves are potentially less important, so we
/// try to save time here by reducing depth a little bit. This is not always true, but some inaccuracies should be recompensated by deeper search.
///
/// Conditions:
///  - depth >= context.parameters.iir_min_depth
///  - hash move does not exists
fn iir_can_be_applied(context: &mut SearchContext, depth: i8, hash_move: Move) -> bool {
    depth >= parameter!(context.parameters.iir_min_depth) && hash_move == Move::default()
}

/// Gets the internal iterative depth reduction, called R, based on `depth`. The further from the horizon we are, the more reduction will be applied.
fn iir_get_r(_context: &mut SearchContext, _depth: i8) -> i8 {
    /*let reduction_base = parameter!(context.parameters.iir_reduction_base);
    let min_depth = parameter!(context.parameters.iir_min_depth);
    let reduction_step = parameter!(context.parameters.iir_reduction_step);
    let max_reduction = parameter!(context.parameters.iir_max_reduction);

    (reduction_base + (depth - min_depth) / reduction_step).min(max_reduction)*/

    1
}

/// The main idea of the razoring is to detect and prune all nodes, which (based on lazy evaluation) are hopeless compared to the current alpha and
/// the chance to improve the score is too small to spend time here. To make it more safe and not to skip positions where we're somewhere in the
/// middle of capture sequence, there's the quiescence search performed to verify if the final score is still below alpha - margin.
///
/// Conditions:
///  - only non-PV nodes
///  - depth >= [razoring_min_depth]
///  - depth <= [razoring_max_depth]
///  - alpha is not a mate score
///  - friendly king is not checked
fn razoring_can_be_applied<const PV: bool>(context: &mut SearchContext, depth: i8, alpha: i16, friendly_king_checked: bool) -> bool {
    let min_depth = parameter!(context.parameters.razoring_min_depth);
    let max_depth = parameter!(context.parameters.razoring_max_depth);

    !PV && depth >= min_depth && depth <= max_depth && !is_score_near_checkmate(alpha) && !friendly_king_checked
}

/// Gets the razoring margin, based on `depth`. The further from the horizon we are, the more margin we should take to determine if node can be pruned.
fn razoring_get_margin(context: &mut SearchContext, depth: i8) -> i16 {
    let depth_margin_base = parameter!(context.parameters.razoring_depth_margin_base);
    let min_depth = parameter!(context.parameters.razoring_min_depth);
    let depth_margin_multiplier = parameter!(context.parameters.razoring_depth_margin_multiplier);

    depth_margin_base + ((depth - min_depth) as i16) * depth_margin_multiplier
}

/// The main idea of the static null move pruning (also called as reverse futility pruning) is to prune all nodes, which (based on lazy evaluation) are too
/// good compared to the current beta, and will very likely be a cut-node. To save time, we skip move loop entirely and return beta + some margin score.
/// The concept is very similar to null move pruning, but without performing any search.
///
/// Conditions:
///  - only non-PV nodes
///  - depth >= [snmp_min_depth]
///  - depth <= [snmp_max_depth]
///  - beta is not a mate score
///  - friendly king is not checked
fn snmp_can_be_applied<const PV: bool>(context: &mut SearchContext, depth: i8, beta: i16, friendly_king_checked: bool) -> bool {
    let min_depth = parameter!(context.parameters.snmp_min_depth);
    let max_depth = parameter!(context.parameters.snmp_max_depth);

    !PV && depth >= min_depth && depth <= max_depth && !is_score_near_checkmate(beta) && !friendly_king_checked
}

/// Gets the static null move pruning margin, based on `depth`. The further from the horizon we are, the more margin should we take to determine
/// if node can be pruned.
fn snmp_get_margin(context: &mut SearchContext, depth: i8) -> i16 {
    let depth_margin_base = parameter!(context.parameters.snmp_depth_margin_base);
    let min_depth = parameter!(context.parameters.snmp_min_depth);
    let depth_margin_multiplier = parameter!(context.parameters.snmp_depth_margin_multiplier);

    depth_margin_base + ((depth - min_depth) as i16) * depth_margin_multiplier
}

/// The main idea of the null move pruning is to prune all nodes, for which the search gives us score above beta even if we skip a move (which allows
/// the opposite color to make two of them in a row). This is based on the null move observation, which says that there's always a better alternative than
/// doing nothing (except zugzwang).
///
/// Conditions:
///  - only non-PV nodes
///  - depth >= [nmp_min_depth]
///  - game phase is not indicating endgame
///  - beta score is not a mate score
///  - friendly king is not checked
///  - this is not the second null move in a row
fn nmp_can_be_applied<const PV: bool>(context: &mut SearchContext, depth: i8, beta: i16, allow_null_move: bool, friendly_king_checked: bool) -> bool {
    let min_depth = parameter!(context.parameters.nmp_min_depth);
    let min_game_phase = parameter!(context.parameters.nmp_min_game_phase);

    !PV && depth >= min_depth && context.board.game_phase > min_game_phase && !is_score_near_checkmate(beta) && !friendly_king_checked && allow_null_move
}

/// Gets the null move pruning depth reduction, called R, based on `depth`. The further from the horizon we are, the more reduction will be applied.
fn nmp_get_r(context: &mut SearchContext, depth: i8) -> i8 {
    let depth_base = parameter!(context.parameters.nmp_depth_base);
    let depth_divider = parameter!(context.parameters.nmp_depth_divider);

    depth_base + depth / depth_divider
}

/// The main idea of the late move pruning is to prune all nodes, which are near the horizon and were scored low by the history table.
/// We assume here, that there's a little chance that move being near the end of the list will improve score, so there's no point of spending time here.
///
/// Conditions:
///  - only non-PV nodes
///  - depth >= [lmp_min_depth]
///  - depth <= [lmp_max_depth]
///  - move index >= [lmp_move_index_margin_multiplier] + (`depth` - 1) * [lmp_move_index_margin_multiplier]
///  - move score <= [lmp_max_score]
///  - friendly king is not checked
fn lmp_can_be_applied<const PV: bool>(context: &mut SearchContext, depth: i8, move_index: usize, move_score: i16, friendly_king_checked: bool) -> bool {
    let min_depth = parameter!(context.parameters.lmp_min_depth);
    let max_depth = parameter!(context.parameters.lmp_max_depth);
    let move_index_margin_base = parameter!(context.parameters.lmp_move_index_margin_base);
    let move_index_margin_multiplier = parameter!(context.parameters.lmp_move_index_margin_multiplier);
    let max_score = parameter!(context.parameters.lmp_max_score);

    !PV && depth >= min_depth
        && depth <= max_depth
        && move_index >= move_index_margin_base + (depth as usize - 1) * move_index_margin_multiplier
        && move_score <= max_score
        && !friendly_king_checked
}

/// The main idea of the late move reduction is to reduce search depth of all quiet moves, which aren't promising and with high chance won't improve score.
/// This is the least risky type of pruning (used inside PVS framework which cares about re-search when the move is better than expected),
/// so it's also applied in PV nodes.
///
/// Conditions:
///  - depth >= [lmr_min_depth]
///  - move index >= [lmr_pv_min_move_index] or move index >= [lmr_min_move_index]
///  - move score <= [lmr_max_score]
///  - move is quiet
///  - friendly king is not checked
///  - enemy king is not checked
fn lmr_can_be_applied<const PV: bool>(
    context: &mut SearchContext,
    depth: i8,
    r#move: Move,
    move_index: usize,
    move_score: i16,
    friendly_king_checked: bool,
    enemy_king_checked: bool,
) -> bool {
    let min_depth = parameter!(context.parameters.lmr_min_depth);
    let min_move_index = if PV { parameter!(context.parameters.lmr_pv_min_move_index) } else { parameter!(context.parameters.lmr_min_move_index) };
    let max_score = parameter!(context.parameters.lmr_max_score);

    depth >= min_depth && move_index >= min_move_index && move_score <= max_score && r#move.is_quiet() && !friendly_king_checked && !enemy_king_checked
}

/// Gets the late move depth reduction, called R, based on `move_index`. The lower the move was scored, the larger reduction will be returned.
fn lmr_get_r<const PV: bool>(context: &mut SearchContext, move_index: usize) -> i8 {
    let (max, r) = if PV {
        let max_reduction = parameter!(context.parameters.lmr_pv_max_reduction);
        let reduction_base = parameter!(context.parameters.lmr_pv_reduction_base);
        let min_move_index = parameter!(context.parameters.lmr_pv_min_move_index);
        let reduction_step = parameter!(context.parameters.lmr_pv_reduction_step);

        (max_reduction, (reduction_base + (move_index - min_move_index) / reduction_step))
    } else {
        let max_reduction = parameter!(context.parameters.lmr_max_reduction);
        let reduction_base = parameter!(context.parameters.lmr_reduction_base);
        let min_move_index = parameter!(context.parameters.lmr_min_move_index);
        let reduction_step = parameter!(context.parameters.lmr_reduction_step);

        (max_reduction, (reduction_base + (move_index - min_move_index) / reduction_step))
    };

    cmp::min(max, r as i8)
}
