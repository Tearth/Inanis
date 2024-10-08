use crate::cache::search::TTableScoreType;
use crate::engine::context::SearchContext;
use crate::engine::*;
use crate::state::movescan::Move;
use crate::tablebases::syzygy;
use crate::tablebases::WdlResult;
use crate::utils::assert_fast;
use crate::utils::dev;
use crate::utils::param;
use context::SearchResultLine;
use search::movepick;
use search::movepick::MoveGenStage;
use std::cmp;
use std::mem::MaybeUninit;
use std::sync::atomic::Ordering;

/// Aspiration window wrapper for the entry point of the regular search, look at `run_internal` for more information.
pub fn run(context: &mut SearchContext, depth: i8) {
    let king_checked = context.board.is_king_checked(context.board.stm);
    if depth < param!(context.params.aspwin_min_depth) {
        context.last_score = run_internal::<true, true>(context, depth, 0, MIN_ALPHA, MIN_BETA, true, king_checked, Move::default());
    } else {
        let mut delta = param!(context.params.aspwin_delta);
        let mut alpha = context.last_score - delta;
        let mut beta = context.last_score + delta;

        loop {
            let score = run_internal::<true, true>(context, depth, 0, alpha, beta, true, king_checked, Move::default());
            if score.abs() == INVALID_SCORE.abs() {
                break;
            }

            if score <= alpha {
                alpha -= delta;
            } else if score >= beta {
                beta += delta;
            } else {
                context.last_score = score;
                break;
            }

            delta *= 2;
            if delta >= param!(context.params.aspwin_max_width) {
                alpha = MIN_ALPHA;
                beta = MIN_BETA;
            }

            context.lines.clear();
        }
    }
}

/// Entry point of the regular search, with generic `ROOT` parameter indicating if this is the root node where the moves filterigh might happen, and `PV` parameter
/// determining if the current node is a PV (principal variation) node in the PVS framework. The implementation contains a typical alpha-beta approach, together with
/// a bunch of reductions and prunings to optimize search. The most important parameter here, `context`, contains the current state of the search, board state,
/// statistics, and is passed by reference to all nodes. Besides obvious parameters like `depth`, `ply`, `alpha` and `beta`, there's also `allow_null_move`
/// which prevents two null move checks in a row, and `friendly_king_checked` which is used to share friendly king check status between nodes (it's always
/// calculated one depth earlier, as it's used as one of the LMR constraints).
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
fn run_internal<const ROOT: bool, const PV: bool>(
    context: &mut SearchContext,
    mut depth: i8,
    ply: u16,
    mut alpha: i16,
    mut beta: i16,
    mut allow_null_move: bool,
    friendly_king_checked: bool,
    previous_move: Move,
) -> i16 {
    assert_fast!(alpha <= beta);

    if context.abort_flag.load(Ordering::Relaxed) {
        return INVALID_SCORE;
    }

    if context.forced_depth == 0 && context.max_nodes_count == 0 && (context.stats.nodes_count & 8191) == 0 {
        if unsafe { context.search_time_start.elapsed().unwrap_unchecked().as_millis() } > context.deadline as u128 {
            context.abort_flag.store(true, Ordering::Relaxed);
            return INVALID_SCORE;
        }
    }

    if context.max_nodes_count != 0 {
        if context.stats.nodes_count + context.stats.q_nodes_count >= context.max_nodes_count {
            context.abort_flag.store(true, Ordering::Relaxed);
            return INVALID_SCORE;
        }
    }

    context.stats.nodes_count += 1;

    if context.board.is_king_checked(context.board.stm ^ 1) {
        dev!(context.stats.leafs_count += 1);

        // The position where both kings are checked is illegal, it will be filtered after returning invalid score
        if friendly_king_checked {
            return INVALID_SCORE;
        } else {
            return CHECKMATE_SCORE - (ply as i16);
        }
    }

    if context.board.is_repetition_draw(if ROOT { 3 } else { 2 }) || context.board.is_fifty_move_rule_draw() || context.board.is_insufficient_material_draw() {
        dev!(context.stats.leafs_count += 1);
        return DRAW_SCORE;
    }

    if context.syzygy_enabled && depth >= context.syzygy_probe_depth && context.board.get_pieces_count() <= syzygy::probe::get_max_pieces_count() {
        if let Some(wdl) = syzygy::probe::get_wdl(&context.board) {
            context.stats.tb_hits += 1;
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
        dev!(context.stats.leafs_count += 1);
        return qsearch::run(context, ply, alpha, beta);
    }

    let original_alpha = alpha;
    let mut tt_entry_found = false;
    let mut hash_move = Default::default();

    match context.ttable.get(context.board.state.hash, ply) {
        Some(entry) => {
            dev!(context.stats.tt_hits += 1);

            if entry.best_move.is_some() {
                if entry.best_move.is_legal(&context.board) {
                    hash_move = entry.best_move;
                    dev!(context.stats.tt_legal_hashmoves += 1);
                } else {
                    dev!(context.stats.tt_illegal_hashmoves += 1);
                }
            }

            if !ROOT {
                if entry.depth >= depth {
                    tt_entry_found = true;
                    match entry.r#type {
                        TTableScoreType::UPPER_BOUND => {
                            beta = cmp::min(beta, entry.score);
                        }
                        TTableScoreType::LOWER_BOUND => {
                            alpha = cmp::max(alpha, entry.score);
                        }
                        _ => {
                            if !PV || entry.age == 0 {
                                if !context.board.is_repetition_draw(2) {
                                    dev!(context.stats.leafs_count += 1);
                                    return entry.score;
                                }
                            }
                        }
                    }

                    if alpha >= beta {
                        dev!(context.stats.leafs_count += 1);
                        dev!(context.stats.beta_cutoffs += 1);
                        return entry.score;
                    }
                } else {
                    match entry.r#type {
                        TTableScoreType::UPPER_BOUND | TTableScoreType::EXACT_SCORE => {
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
            dev!(context.stats.tt_misses += 1);
        }
    };

    if iir_can_be_applied(context, depth, hash_move) {
        depth -= iir_get_r(context, depth);
    }

    let mut lazy_eval = None;

    if razoring_can_be_applied::<PV>(context, depth, alpha, friendly_king_checked) {
        let margin = razoring_get_margin(context, depth);
        let lazy_eval_value = match lazy_eval {
            Some(value) => value,
            None => context.board.evaluate_fast(context.board.stm, &context.phtable, &mut context.stats),
        };

        dev!(context.stats.razoring_attempts += 1);
        if lazy_eval_value + margin <= alpha {
            let score = qsearch::run(context, ply, alpha, beta);
            if score <= alpha {
                dev!(context.stats.leafs_count += 1);
                dev!(context.stats.razoring_accepted += 1);
                return score;
            } else {
                dev!(context.stats.razoring_rejected += 1);
            }
        }

        lazy_eval = Some(lazy_eval_value);
    }

    if snmp_can_be_applied::<PV>(context, depth, beta, friendly_king_checked) {
        let margin = snmp_get_margin(context, depth);
        let lazy_eval_value = match lazy_eval {
            Some(value) => value,
            None => context.board.evaluate_fast(context.board.stm, &context.phtable, &mut context.stats),
        };

        dev!(context.stats.snmp_attempts += 1);
        if lazy_eval_value - margin >= beta {
            dev!(context.stats.leafs_count += 1);
            dev!(context.stats.snmp_accepted += 1);
            return lazy_eval_value - margin;
        } else {
            dev!(context.stats.snmp_rejected += 1);
        }

        lazy_eval = Some(lazy_eval_value);
    }

    if nmp_can_be_applied::<PV>(context, depth, beta, allow_null_move, friendly_king_checked) {
        let margin = param!(context.params.nmp_margin);
        let lazy_eval_value = match lazy_eval {
            Some(value) => value,
            None => context.board.evaluate_fast(context.board.stm, &context.phtable, &mut context.stats),
        };

        dev!(context.stats.nmp_attempts += 1);
        if lazy_eval_value + margin >= beta {
            let r = nmp_get_r(context, depth);

            context.board.make_null_move();
            let score = -run_internal::<false, false>(context, depth - r - 1, ply + 1, -beta, -beta + 1, false, false, Move::default());
            context.board.undo_null_move();

            if score >= beta {
                dev!(context.stats.leafs_count += 1);
                dev!(context.stats.nmp_accepted += 1);
                return score;
            } else {
                dev!(context.stats.nmp_rejected += 1);
            }
        }

        lazy_eval = Some(lazy_eval_value);
    }

    let mut best_score = -CHECKMATE_SCORE;
    let mut best_move = Default::default();
    let mut moves = [MaybeUninit::uninit(); MAX_MOVES_COUNT];
    let mut move_scores = [MaybeUninit::uninit(); MAX_MOVES_COUNT];
    let mut movegen_stage = MoveGenStage::ReadyToCheckHashMove;
    let mut quiet_moves_start_index = 0;
    let mut killer_moves = [MaybeUninit::uninit(); 2];

    let mut move_index = 0;
    let mut move_number = 0;
    let mut moves_count = 0;
    let mut evasion_mask = 0;

    while let Some((r#move, score)) = movepick::get_next_move(
        context,
        &mut movegen_stage,
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
            dev!(context.stats.lmp_accepted += 1);
            break;
        } else {
            dev!(context.stats.lmp_rejected += 1);
        }

        context.board.make_move(r#move);
        context.ttable.prefetch(context.board.state.hash);

        let king_checked = context.board.is_king_checked(context.board.stm);
        let r = if lmr_can_be_applied::<PV>(context, depth, r#move, move_number, score, friendly_king_checked, king_checked) {
            lmr_get_r::<PV>(context, move_number)
        } else {
            0
        };

        let score = if PV {
            if move_index == 0 {
                dev!(context.stats.pvs_full_window_searches += 1);
                -run_internal::<false, true>(context, depth - 1, ply + 1, -beta, -alpha, true, king_checked, r#move)
            } else {
                let zero_window_score = -run_internal::<false, false>(context, depth - r - 1, ply + 1, -alpha - 1, -alpha, true, king_checked, r#move);
                dev!(context.stats.pvs_zero_window_searches += 1);

                if zero_window_score > alpha && (alpha != beta - 1 || r > 0) && zero_window_score != -INVALID_SCORE {
                    dev!(context.stats.pvs_rejected_searches += 1);
                    -run_internal::<false, true>(context, depth - 1, ply + 1, -beta, -alpha, true, king_checked, r#move)
                } else {
                    zero_window_score
                }
            }
        } else {
            let zero_window_score = -run_internal::<false, false>(context, depth - r - 1, ply + 1, -beta, -alpha, true, king_checked, r#move);
            dev!(context.stats.pvs_zero_window_searches += 1);

            if zero_window_score > alpha && r > 0 && zero_window_score != -INVALID_SCORE {
                dev!(context.stats.pvs_rejected_searches += 1);
                -run_internal::<false, false>(context, depth - 1, ply + 1, -beta, -alpha, true, king_checked, r#move)
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
                    context.ktable.add(ply, r#move);
                    context.htable.add(r#move.get_from(), r#move.get_to(), depth as u8);

                    if previous_move.is_some() {
                        context.cmtable.add(previous_move, r#move);
                    }

                    if movegen_stage == MoveGenStage::AllGenerated {
                        for i in quiet_moves_start_index..moves_count {
                            assert_fast!(moves_count < MAX_MOVES_COUNT);

                            let move_from_list = unsafe { moves[i].assume_init() };
                            if move_from_list.is_quiet() && move_from_list != best_move {
                                context.htable.punish(move_from_list.get_from(), move_from_list.get_to(), depth as u8);
                            }
                        }
                    }
                }

                dev!(context.stats.beta_cutoffs += 1);
                if move_number == 0 {
                    dev!(context.stats.perfect_cutoffs += 1);
                } else {
                    dev!(context.stats.non_perfect_cutoffs += 1);
                }

                break;
            }
        }

        if ROOT && context.multipv {
            context.board.make_move(best_move);
            if !context.board.is_king_checked(context.board.stm ^ 1) {
                let mut pv_line = context.ttable.get_pv_line(&mut context.board.clone(), 0);
                pv_line.insert(0, best_move);

                context.lines.push(SearchResultLine::new(best_score, pv_line));
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
            TTableScoreType::UPPER_BOUND
        } else if alpha >= beta {
            TTableScoreType::LOWER_BOUND
        } else {
            TTableScoreType::EXACT_SCORE
        };

        context.ttable.add(context.board.state.hash, alpha, best_move, depth, ply, score_type, context.search_id);
        dev!(context.stats.tt_added += 1);
    }

    if ROOT && !context.multipv {
        let pv_line = context.ttable.get_pv_line(&mut context.board.clone(), 0);
        context.lines.push(SearchResultLine::new(best_score, pv_line));
    }

    best_score
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
///  - depth >= `iir_min_depth`
///  - hash move does not exists
fn iir_can_be_applied(context: &mut SearchContext, depth: i8, hash_move: Move) -> bool {
    depth >= param!(context.params.iir_min_depth) && hash_move == Move::default()
}

/// Gets the internal iterative depth reduction, called R, based on `depth`. The further from the horizon we are, the more reduction will be applied.
fn iir_get_r(_context: &mut SearchContext, _depth: i8) -> i8 {
    /*let reduction_base = parameter!(context.params.iir_reduction_base);
    let min_depth = parameter!(context.params.iir_min_depth);
    let reduction_step = parameter!(context.params.iir_reduction_step);
    let max_reduction = parameter!(context.params.iir_max_reduction);

    (reduction_base + (depth - min_depth) / reduction_step).min(max_reduction)*/

    1
}

/// The main idea of the razoring is to detect and prune all nodes, which (based on lazy evaluation) are hopeless compared to the current alpha and
/// the chance to improve the score is too small to spend time here. To make it more safe and not to skip positions where we're somewhere in the
/// middle of capture sequence, there's the quiescence search performed to verify if the final score is still below alpha - margin.
///
/// Conditions:
///  - only non-PV nodes
///  - depth >= `razoring_min_depth`
///  - depth <= `razoring_max_depth`
///  - alpha is not a mate score
///  - friendly king is not checked
fn razoring_can_be_applied<const PV: bool>(context: &mut SearchContext, depth: i8, alpha: i16, friendly_king_checked: bool) -> bool {
    let min_depth = param!(context.params.razoring_min_depth);
    let max_depth = param!(context.params.razoring_max_depth);

    !PV && depth >= min_depth && depth <= max_depth && !is_score_near_checkmate(alpha) && !friendly_king_checked
}

/// Gets the razoring margin, based on `depth`. The further from the horizon we are, the more margin we should take to determine if node can be pruned.
fn razoring_get_margin(context: &mut SearchContext, depth: i8) -> i16 {
    let depth_margin_base = param!(context.params.razoring_depth_margin_base);
    let min_depth = param!(context.params.razoring_min_depth);
    let depth_margin_multiplier = param!(context.params.razoring_depth_margin_multiplier);

    depth_margin_base + ((depth - min_depth) as i16) * depth_margin_multiplier
}

/// The main idea of the static null move pruning (also called as reverse futility pruning) is to prune all nodes, which (based on lazy evaluation) are too
/// good compared to the current beta, and will very likely be a cut-node. To save time, we skip move loop entirely and return beta + some margin score.
/// The concept is very similar to null move pruning, but without performing any search.
///
/// Conditions:
///  - only non-PV nodes
///  - depth >= `snmp_min_depth`
///  - depth <= `snmp_max_depth`
///  - beta is not a mate score
///  - friendly king is not checked
fn snmp_can_be_applied<const PV: bool>(context: &mut SearchContext, depth: i8, beta: i16, friendly_king_checked: bool) -> bool {
    let min_depth = param!(context.params.snmp_min_depth);
    let max_depth = param!(context.params.snmp_max_depth);

    !PV && depth >= min_depth && depth <= max_depth && !is_score_near_checkmate(beta) && !friendly_king_checked
}

/// Gets the static null move pruning margin, based on `depth`. The further from the horizon we are, the more margin should we take to determine
/// if node can be pruned.
fn snmp_get_margin(context: &mut SearchContext, depth: i8) -> i16 {
    let depth_margin_base = param!(context.params.snmp_depth_margin_base);
    let min_depth = param!(context.params.snmp_min_depth);
    let depth_margin_multiplier = param!(context.params.snmp_depth_margin_multiplier);

    depth_margin_base + ((depth - min_depth) as i16) * depth_margin_multiplier
}

/// The main idea of the null move pruning is to prune all nodes, for which the search gives us score above beta even if we skip a move (which allows
/// the opposite color to make two of them in a row). This is based on the null move observation, which says that there's always a better alternative than
/// doing nothing (except zugzwang).
///
/// Conditions:
///  - only non-PV nodes
///  - depth >= `nmp_min_depth`
///  - game phase is not indicating endgame
///  - beta score is not a mate score
///  - friendly king is not checked
///  - this is not the second null move in a row
fn nmp_can_be_applied<const PV: bool>(context: &mut SearchContext, depth: i8, beta: i16, allow_null_move: bool, friendly_king_checked: bool) -> bool {
    let min_depth = param!(context.params.nmp_min_depth);
    let min_game_phase = param!(context.params.nmp_min_game_phase);

    !PV && depth >= min_depth && context.board.game_phase > min_game_phase && !is_score_near_checkmate(beta) && !friendly_king_checked && allow_null_move
}

/// Gets the null move pruning depth reduction, called R, based on `depth`. The further from the horizon we are, the more reduction will be applied.
fn nmp_get_r(context: &mut SearchContext, depth: i8) -> i8 {
    let depth_base = param!(context.params.nmp_depth_base);
    let depth_divider = param!(context.params.nmp_depth_divider);

    depth_base + depth / depth_divider
}

/// The main idea of the late move pruning is to prune all nodes, which are near the horizon and were scored low by the history table.
/// We assume here, that there's a little chance that move being near the end of the list will improve score, so there's no point of spending time here.
///
/// Conditions:
///  - only non-PV nodes
///  - depth >= `lmp_min_depth`
///  - depth <= `lmp_max_depth`
///  - move index >= `lmp_move_index_margin_multiplier` + (`depth` - 1) * `lmp_move_index_margin_multiplier`
///  - move score <= `lmp_max_score`
///  - friendly king is not checked
fn lmp_can_be_applied<const PV: bool>(context: &mut SearchContext, depth: i8, move_index: usize, move_score: i16, friendly_king_checked: bool) -> bool {
    let min_depth = param!(context.params.lmp_min_depth);
    let max_depth = param!(context.params.lmp_max_depth);
    let move_index_margin_base = param!(context.params.lmp_move_index_margin_base);
    let move_index_margin_multiplier = param!(context.params.lmp_move_index_margin_multiplier);
    let max_score = param!(context.params.lmp_max_score);

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
///  - depth >= `lmr_min_depth`
///  - move index >= `lmr_pv_min_move_index` or move index >= `lmr_min_move_index`
///  - move score <= `lmr_max_score`
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
    let min_depth = param!(context.params.lmr_min_depth);
    let min_move_index = if PV { param!(context.params.lmr_pv_min_move_index) } else { param!(context.params.lmr_min_move_index) };
    let max_score = param!(context.params.lmr_max_score);

    depth >= min_depth && move_index >= min_move_index && move_score <= max_score && r#move.is_quiet() && !friendly_king_checked && !enemy_king_checked
}

/// Gets the late move depth reduction, called R, based on `move_index`. The lower the move was scored, the larger reduction will be returned.
fn lmr_get_r<const PV: bool>(context: &mut SearchContext, move_index: usize) -> i8 {
    let (max, r) = if PV {
        let max_reduction = param!(context.params.lmr_pv_max_reduction);
        let reduction_base = param!(context.params.lmr_pv_reduction_base);
        let min_move_index = param!(context.params.lmr_pv_min_move_index);
        let reduction_step = param!(context.params.lmr_pv_reduction_step);

        (max_reduction, (reduction_base + (move_index - min_move_index) / reduction_step))
    } else {
        let max_reduction = param!(context.params.lmr_max_reduction);
        let reduction_base = param!(context.params.lmr_reduction_base);
        let min_move_index = param!(context.params.lmr_min_move_index);
        let reduction_step = param!(context.params.lmr_reduction_step);

        (max_reduction, (reduction_base + (move_index - min_move_index) / reduction_step))
    };

    cmp::min(max, r as i8)
}
