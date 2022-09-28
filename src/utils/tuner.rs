use super::fen;
use super::rand;
use crate::engine::see::SEEContainer;
use crate::evaluation::EvaluationParameters;
use crate::state::board::Bitboard;
use crate::state::movegen::MagicContainer;
use crate::state::patterns::PatternsContainer;
use crate::state::zobrist::ZobristContainer;
use crate::state::*;
use std::fmt::Display;
use std::fs;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;
use std::path::Path;
use std::sync::Arc;
use std::thread;
use std::time::SystemTime;

struct TunerContext {
    positions: Vec<TunerPosition>,
    parameters: EvaluationParameters,
}

struct TunerPosition {
    board: Bitboard,
    result: f64,
}

#[derive(Clone, Copy)]
struct TunerParameter {
    value: i16,
    min: i16,
    min_init: i16,
    max_init: i16,
    max: i16,
}

impl TunerContext {
    /// Constructs a new instance of [TunerContext] with stored `positions`.
    pub fn new(positions: Vec<TunerPosition>) -> Self {
        Self { positions, parameters: Default::default() }
    }
}

impl TunerPosition {
    /// Constructs a new instance of [TunerPosition] with stored `board` and `result`.
    pub fn new(board: Bitboard, result: f64) -> Self {
        Self { board, result }
    }
}

impl TunerParameter {
    /// Constructs a new instance of [TunerParameter] with stored `value`, `min`, `min_init`, `max_init` and `max`.
    pub fn new(value: i16, min: i16, min_init: i16, max_init: i16, max: i16) -> Self {
        Self { value, min, min_init, max_init, max }
    }
}

/// Runs tuner of evaluation parameters. The input file is specified by `epd_filename` with a list of positions and their expected results, and the `output_directory`
/// directory is used to store generated Rust sources with the optimized values. Use `lock_material` to disable tuner for piece values, and `random_values` to initialize
/// evaluation parameters with random values. Multithreading is supported by `threads_count`.
///
/// The tuner is implemented using Texel's tuning method (<https://www.chessprogramming.org/Texel%27s_Tuning_Method>), with addition of cache to reduce time needed
/// to get the best result. The loaded positions must be quiet, since the tuner doesn't run quiescence search to make sure that the position is not in the middle
/// of capture sequence. The cache itself is a list of trends corresponding to the evaluation parameters - it's used to save the information if the value in the previous
/// iterations was increasing or decreasing, so the tuner can try this direction first. The more times the direction was right, the bigger increasion or decreasion will
/// be performed as next.
///
/// The result (Rust sources with the calculated values) are saved every iteration, and can be put directly into the code.
pub fn run(epd_filename: &str, output_directory: &str, lock_material: bool, random_values: bool, threads_count: usize) {
    println!("Loading EPD file...");
    let positions = match load_positions(epd_filename) {
        Ok(value) => value,
        Err(error) => {
            println!("Invalid EPD file: {}", error);
            return;
        }
    };
    println!("Loaded {} positions, starting tuner", positions.len());

    let mut context = TunerContext::new(positions);
    let mut tendence = Vec::new();
    tendence.resize(context.positions.len(), 1i8);

    let mut best_values = load_values(&context, lock_material, random_values);
    save_values(&mut context, &mut best_values, lock_material);

    let mut best_error = calculate_error(&mut context, 1.13, threads_count);
    let mut improved = true;
    let mut iterations_count = 0;

    while improved {
        improved = false;
        iterations_count += 1;

        let mut changes = 0;
        let last_best_error = best_error;
        let start_time = SystemTime::now();

        for value_index in 0..best_values.len() {
            // Ignore pawn and king value (no point to tune it)
            if !lock_material && (value_index == 0 || value_index == 5) {
                continue;
            }

            let step = tendence[value_index] as i16;
            let mut values = best_values.to_vec();
            let mut value_changed = false;

            let original_value = values[value_index].value;
            let error = if original_value == values[value_index].max {
                f64::MAX
            } else {
                let min = values[value_index].min;
                let max = values[value_index].max;

                values[value_index].value += step;
                values[value_index].value = values[value_index].value.clamp(min, max);

                save_values(&mut context, &mut values, lock_material);
                calculate_error(&mut context, 1.13, threads_count)
            };

            if error < best_error {
                tendence[value_index] *= 2;
                best_error = error;
                best_values = values;
                improved = true;
                value_changed = true;
                changes += 1;

                println!("Value {} changed by {} (new error: {:.6})", value_index, step, best_error);
            } else if error > best_error {
                let mut values = best_values.to_vec();
                let error = if original_value == values[value_index].min {
                    f64::MAX
                } else {
                    let min = values[value_index].min;
                    let max = values[value_index].max;

                    values[value_index].value -= step.signum();
                    values[value_index].value = values[value_index].value.clamp(min, max);

                    save_values(&mut context, &mut values, lock_material);
                    calculate_error(&mut context, 1.13, threads_count)
                };

                if error < best_error {
                    tendence[value_index] = 2 * -step.signum() as i8;
                    best_error = error;
                    best_values = values;
                    improved = true;
                    value_changed = true;
                    changes += 1;

                    println!("Value {} changed by {} (tendence change, new error: {:.6})", value_index, -step.signum(), best_error);
                }
            }

            if !value_changed {
                println!("Value {} skipped", value_index);

                // Step may be too big, reset it
                tendence[value_index] = step.signum() as i8;
            }
        }

        write_evaluation_parameters(&mut context, output_directory, best_error);
        write_piece_square_table(output_directory, best_error, "pawn", &context.parameters.pst_patterns[PAWN as usize]);
        write_piece_square_table(output_directory, best_error, "knight", &context.parameters.pst_patterns[KNIGHT as usize]);
        write_piece_square_table(output_directory, best_error, "bishop", &context.parameters.pst_patterns[BISHOP as usize]);
        write_piece_square_table(output_directory, best_error, "rook", &context.parameters.pst_patterns[ROOK as usize]);
        write_piece_square_table(output_directory, best_error, "queen", &context.parameters.pst_patterns[QUEEN as usize]);
        write_piece_square_table(output_directory, best_error, "king", &context.parameters.pst_patterns[KING as usize]);

        println!(
            "Iteration {} done in {} seconds, {} changes made, error reduced from {:.6} to {:.6} ({:.6})",
            iterations_count,
            (start_time.elapsed().unwrap().as_millis() as f32) / 1000.0,
            changes,
            last_best_error,
            best_error,
            last_best_error - best_error
        );
    }
}

/// Tests the correctness of [load_values] and [save_values] methods.
pub fn validate() -> bool {
    let mut context = TunerContext::new(Vec::new());
    let mut values = load_values(&context, false, false);
    save_values(&mut context, &mut values, false);

    let values_after_save = load_values(&context, false, false);
    values.iter().zip(&values_after_save).all(|(a, b)| a.value == b.value)
}

/// Loads positions from the `epd_filename` and parses them into a list of [TunerPosition]. Returns [Err] with a proper error message if the
/// file couldn't be parsed.
fn load_positions(epd_filename: &str) -> Result<Vec<TunerPosition>, String> {
    let mut positions = Vec::new();
    let file = match File::open(epd_filename) {
        Ok(value) => value,
        Err(error) => return Err(format!("Invalid EPD file: {}", error)),
    };

    let evaluation_parameters = Arc::new(EvaluationParameters::default());
    let zobrist_container = Arc::new(ZobristContainer::default());
    let patterns_container = Arc::new(PatternsContainer::default());
    let see_container = Arc::new(SEEContainer::new(Some(evaluation_parameters.clone())));
    let magic_container = Arc::new(MagicContainer::default());

    for line in BufReader::new(file).lines() {
        let position = line.unwrap();
        let parsed_epd = fen::epd_to_board(
            position.as_str(),
            Some(evaluation_parameters.clone()),
            Some(zobrist_container.clone()),
            Some(patterns_container.clone()),
            Some(see_container.clone()),
            Some(magic_container.clone()),
        )?;

        if parsed_epd.comment == None {
            return Err("Game result not found".to_string());
        }

        let comment = parsed_epd.comment.unwrap();
        let result = match comment.as_str() {
            "1-0" => 1.0,
            "1/2-1/2" => 0.5,
            "0-1" => 0.0,
            _ => return Err(format!("Invalid game result: comment={}", comment)),
        };

        positions.push(TunerPosition::new(parsed_epd.board, result));
    }

    Ok(positions)
}

/// Calculates an error by evaluating all loaded positions with the currently set evaluation parameters. Multithreading is supported by `threads_count`.
fn calculate_error(context: &mut TunerContext, scaling_constant: f64, threads_count: usize) -> f64 {
    let mut sum_of_errors = 0.0;
    let positions_count = context.positions.len();

    let evaluation_parameters = Arc::new(context.parameters.clone());
    thread::scope(|scope| {
        let mut threads = Vec::new();
        for chunk in context.positions.chunks_mut(positions_count / threads_count) {
            let evaluation_parameters_arc = evaluation_parameters.clone();
            threads.push(scope.spawn(move || {
                for position in chunk {
                    let evaluation_parameters_arc = evaluation_parameters_arc.clone();
                    position.board.evaluation_parameters = evaluation_parameters_arc;
                    position.board.recalculate_incremental_values();

                    let evaluation = position.board.evaluate_without_cache(WHITE) as f64;
                    let sigmoid = 1.0 / (1.0 + 10.0f64.powf(-scaling_constant * evaluation / 400.0));
                    sum_of_errors += (position.result - sigmoid).powi(2);
                }

                sum_of_errors
            }));
        }

        for thread in threads {
            sum_of_errors += thread.join().unwrap();
        }
    });

    sum_of_errors / (positions_count as f64)
}

/// Transforms the current evaluation values into a list of [TunerParameter]. Use `lock_material` if the parameters related to piece values should
/// be skipped, and `random_values` if the parameters should have random values (useful when initializing tuner).
fn load_values(context: &TunerContext, lock_material: bool, random_values: bool) -> Vec<TunerParameter> {
    let mut parameters = Vec::new();

    if !lock_material {
        parameters.push(TunerParameter::new(context.parameters.piece_value[PAWN as usize], 100, 100, 100, 100));
        parameters.push(TunerParameter::new(context.parameters.piece_value[KNIGHT as usize], 0, 300, 400, 9999));
        parameters.push(TunerParameter::new(context.parameters.piece_value[BISHOP as usize], 0, 300, 400, 9999));
        parameters.push(TunerParameter::new(context.parameters.piece_value[ROOK as usize], 0, 400, 600, 9999));
        parameters.push(TunerParameter::new(context.parameters.piece_value[QUEEN as usize], 0, 900, 1200, 9999));
        parameters.push(TunerParameter::new(context.parameters.piece_value[KING as usize], 10000, 10000, 10000, 10000));
    }

    parameters.push(TunerParameter::new(context.parameters.mobility_opening[PAWN as usize], 0, 3, 6, 8));
    parameters.push(TunerParameter::new(context.parameters.mobility_opening[KNIGHT as usize], 0, 3, 6, 8));
    parameters.push(TunerParameter::new(context.parameters.mobility_opening[BISHOP as usize], 0, 3, 6, 8));
    parameters.push(TunerParameter::new(context.parameters.mobility_opening[ROOK as usize], 0, 3, 6, 8));
    parameters.push(TunerParameter::new(context.parameters.mobility_opening[QUEEN as usize], 0, 3, 6, 8));
    parameters.push(TunerParameter::new(context.parameters.mobility_opening[KING as usize], 0, 3, 6, 8));

    parameters.push(TunerParameter::new(context.parameters.mobility_ending[PAWN as usize], 0, 2, 6, 10));
    parameters.push(TunerParameter::new(context.parameters.mobility_ending[KNIGHT as usize], 0, 2, 6, 10));
    parameters.push(TunerParameter::new(context.parameters.mobility_ending[BISHOP as usize], 0, 2, 6, 10));
    parameters.push(TunerParameter::new(context.parameters.mobility_ending[ROOK as usize], 0, 2, 6, 10));
    parameters.push(TunerParameter::new(context.parameters.mobility_ending[QUEEN as usize], 0, 2, 6, 10));
    parameters.push(TunerParameter::new(context.parameters.mobility_ending[KING as usize], 0, 2, 6, 10));

    parameters.push(TunerParameter::new(context.parameters.mobility_center_multiplier[PAWN as usize], 0, 2, 6, 10));
    parameters.push(TunerParameter::new(context.parameters.mobility_center_multiplier[KNIGHT as usize], 0, 2, 6, 10));
    parameters.push(TunerParameter::new(context.parameters.mobility_center_multiplier[BISHOP as usize], 0, 2, 6, 10));
    parameters.push(TunerParameter::new(context.parameters.mobility_center_multiplier[ROOK as usize], 0, 2, 6, 10));
    parameters.push(TunerParameter::new(context.parameters.mobility_center_multiplier[QUEEN as usize], 0, 2, 6, 10));
    parameters.push(TunerParameter::new(context.parameters.mobility_center_multiplier[KING as usize], 0, 2, 6, 10));

    parameters.push(TunerParameter::new(context.parameters.doubled_pawn_opening, -999, -40, -10, 999));
    parameters.push(TunerParameter::new(context.parameters.doubled_pawn_ending, -999, -40, -10, 999));

    parameters.push(TunerParameter::new(context.parameters.isolated_pawn_opening, -999, -40, -10, 999));
    parameters.push(TunerParameter::new(context.parameters.isolated_pawn_ending, -999, -40, -10, 999));

    parameters.push(TunerParameter::new(context.parameters.chained_pawn_opening, -999, 10, 40, 999));
    parameters.push(TunerParameter::new(context.parameters.chained_pawn_ending, -999, 10, 40, 999));

    parameters.push(TunerParameter::new(context.parameters.passed_pawn_opening, -999, 10, 40, 999));
    parameters.push(TunerParameter::new(context.parameters.passed_pawn_ending, -999, 10, 40, 999));

    parameters.push(TunerParameter::new(context.parameters.pawn_shield_opening, -999, 10, 40, 999));
    parameters.push(TunerParameter::new(context.parameters.pawn_shield_ending, -999, 10, 40, 999));

    parameters.push(TunerParameter::new(context.parameters.pawn_shield_open_file_opening, -999, -40, -10, 999));
    parameters.push(TunerParameter::new(context.parameters.pawn_shield_open_file_ending, -999, -40, -10, 999));

    parameters.push(TunerParameter::new(context.parameters.king_attacked_squares_opening, -999, -40, -10, 999));
    parameters.push(TunerParameter::new(context.parameters.king_attacked_squares_ending, -999, -40, -10, 999));

    let pawn_pst = &context.parameters.pst_patterns[PAWN as usize];
    parameters.append(&mut pawn_pst[0].iter().map(|v| TunerParameter::new(*v as i16, -999, -40, 40, 999)).collect());
    parameters.append(&mut pawn_pst[1].iter().map(|v| TunerParameter::new(*v as i16, -999, -40, 40, 999)).collect());

    let knight_pst = &context.parameters.pst_patterns[KNIGHT as usize];
    parameters.append(&mut knight_pst[0].iter().map(|v| TunerParameter::new(*v as i16, -999, -40, 40, 999)).collect());
    parameters.append(&mut knight_pst[1].iter().map(|v| TunerParameter::new(*v as i16, -999, -40, 40, 999)).collect());

    let bishop_pst = &context.parameters.pst_patterns[BISHOP as usize];
    parameters.append(&mut bishop_pst[0].iter().map(|v| TunerParameter::new(*v as i16, -999, -40, 40, 999)).collect());
    parameters.append(&mut bishop_pst[1].iter().map(|v| TunerParameter::new(*v as i16, -999, -40, 40, 999)).collect());

    let rook_pst = &context.parameters.pst_patterns[ROOK as usize];
    parameters.append(&mut rook_pst[0].iter().map(|v| TunerParameter::new(*v as i16, -999, -40, 40, 999)).collect());
    parameters.append(&mut rook_pst[1].iter().map(|v| TunerParameter::new(*v as i16, -999, -40, 40, 999)).collect());

    let queen_pst = &context.parameters.pst_patterns[QUEEN as usize];
    parameters.append(&mut queen_pst[0].iter().map(|v| TunerParameter::new(*v as i16, -999, -40, 40, 999)).collect());
    parameters.append(&mut queen_pst[1].iter().map(|v| TunerParameter::new(*v as i16, -999, -40, 40, 999)).collect());

    let king_pst = &context.parameters.pst_patterns[KING as usize];
    parameters.append(&mut king_pst[0].iter().map(|v| TunerParameter::new(*v as i16, -999, -40, 40, 999)).collect());
    parameters.append(&mut king_pst[1].iter().map(|v| TunerParameter::new(*v as i16, -999, -40, 40, 999)).collect());

    if random_values {
        rand::seed(common::time::get_unix_timestamp());
        for parameter in &mut parameters {
            (*parameter).value = rand::i16(parameter.min_init..=parameter.max_init);
        }
    }

    for parameter in &mut parameters {
        (*parameter).value = (*parameter).value.clamp(parameter.min, parameter.max);
    }

    parameters
}

/// Transforms `values` into the evaluation parameters, which can be used during real evaluation. Use `lock_material` if the parameters
/// related to piece values should be skipped.
fn save_values(context: &mut TunerContext, values: &mut [TunerParameter], lock_material: bool) {
    let mut index = 0;

    if !lock_material {
        save_values_to_i16_array_internal(values, &mut context.parameters.piece_value, &mut index);
    }

    save_values_to_i16_array_internal(values, &mut context.parameters.mobility_opening, &mut index);
    save_values_to_i16_array_internal(values, &mut context.parameters.mobility_ending, &mut index);
    save_values_to_i16_array_internal(values, &mut context.parameters.mobility_center_multiplier, &mut index);

    save_values_internal(values, &mut context.parameters.doubled_pawn_opening, &mut index);
    save_values_internal(values, &mut context.parameters.doubled_pawn_ending, &mut index);

    save_values_internal(values, &mut context.parameters.isolated_pawn_opening, &mut index);
    save_values_internal(values, &mut context.parameters.isolated_pawn_ending, &mut index);

    save_values_internal(values, &mut context.parameters.chained_pawn_opening, &mut index);
    save_values_internal(values, &mut context.parameters.chained_pawn_ending, &mut index);

    save_values_internal(values, &mut context.parameters.passed_pawn_opening, &mut index);
    save_values_internal(values, &mut context.parameters.passed_pawn_ending, &mut index);

    save_values_internal(values, &mut context.parameters.pawn_shield_opening, &mut index);
    save_values_internal(values, &mut context.parameters.pawn_shield_ending, &mut index);

    save_values_internal(values, &mut context.parameters.pawn_shield_open_file_opening, &mut index);
    save_values_internal(values, &mut context.parameters.pawn_shield_open_file_ending, &mut index);

    save_values_internal(values, &mut context.parameters.king_attacked_squares_opening, &mut index);
    save_values_internal(values, &mut context.parameters.king_attacked_squares_ending, &mut index);

    let pawn_pst = &mut context.parameters.pst_patterns[PAWN as usize];
    save_values_to_i8_array_internal(values, &mut pawn_pst[0], &mut index);
    save_values_to_i8_array_internal(values, &mut pawn_pst[1], &mut index);

    let knight_pst = &mut context.parameters.pst_patterns[KNIGHT as usize];
    save_values_to_i8_array_internal(values, &mut knight_pst[0], &mut index);
    save_values_to_i8_array_internal(values, &mut knight_pst[1], &mut index);

    let bishop_pst = &mut context.parameters.pst_patterns[BISHOP as usize];
    save_values_to_i8_array_internal(values, &mut bishop_pst[0], &mut index);
    save_values_to_i8_array_internal(values, &mut bishop_pst[1], &mut index);

    let rook_pst = &mut context.parameters.pst_patterns[ROOK as usize];
    save_values_to_i8_array_internal(values, &mut rook_pst[0], &mut index);
    save_values_to_i8_array_internal(values, &mut rook_pst[1], &mut index);

    let queen_pst = &mut context.parameters.pst_patterns[QUEEN as usize];
    save_values_to_i8_array_internal(values, &mut queen_pst[0], &mut index);
    save_values_to_i8_array_internal(values, &mut queen_pst[1], &mut index);

    let king_pst = &mut context.parameters.pst_patterns[KING as usize];
    save_values_to_i8_array_internal(values, &mut king_pst[0], &mut index);
    save_values_to_i8_array_internal(values, &mut king_pst[1], &mut index);

    context.parameters.recalculate();
}

/// Saves `index`-th evaluation parameter stored in `values` in the `destination`.
fn save_values_internal(values: &mut [TunerParameter], destination: &mut i16, index: &mut usize) {
    *destination = values[*index].value;
    *index += 1;
}

/// Saves [i8] array starting at the `index` of `values` in the `array`.
fn save_values_to_i8_array_internal(values: &mut [TunerParameter], array: &mut [i16], index: &mut usize) {
    array.copy_from_slice(&values[*index..(*index + array.len())].iter().map(|v| (*v).value).collect::<Vec<i16>>());
    *index += array.len();
}

/// Saves [i16] array starting at the `index` of `values` in the `array`.
fn save_values_to_i16_array_internal(values: &mut [TunerParameter], array: &mut [i16], index: &mut usize) {
    array.copy_from_slice(&values[*index..(*index + array.len())].iter().map(|v| (*v).value).collect::<Vec<i16>>());
    *index += array.len();
}

/// Generates `parameters.rs` file with current evaluation parameters, and saves it into the `output_directory`.
fn write_evaluation_parameters(context: &mut TunerContext, output_directory: &str, best_error: f64) {
    let mut output = String::new();

    output.push_str(get_header(best_error).as_str());
    output.push('\n');
    output.push_str("use super::*;\n");
    output.push('\n');
    output.push_str("impl Default for EvaluationParameters {\n");
    output.push_str("    fn default() -> Self {\n");
    output.push_str("        let mut evaluation_parameters = Self {\n");
    output.push_str(get_array("piece_value", &context.parameters.piece_value).as_str());
    output.push_str(get_array("piece_phase_value", &context.parameters.piece_phase_value).as_str());
    output.push_str(get_parameter("initial_game_phase", context.parameters.initial_game_phase).as_str());
    output.push('\n');
    output.push_str(get_array("mobility_opening", &context.parameters.mobility_opening).as_str());
    output.push_str(get_array("mobility_ending", &context.parameters.mobility_ending).as_str());
    output.push_str(get_array("mobility_center_multiplier", &context.parameters.mobility_center_multiplier).as_str());
    output.push('\n');
    output.push_str(get_parameter("doubled_pawn_opening", context.parameters.doubled_pawn_opening).as_str());
    output.push_str(get_parameter("doubled_pawn_ending", context.parameters.doubled_pawn_ending).as_str());
    output.push('\n');
    output.push_str(get_parameter("isolated_pawn_opening", context.parameters.isolated_pawn_opening).as_str());
    output.push_str(get_parameter("isolated_pawn_ending", context.parameters.isolated_pawn_ending).as_str());
    output.push('\n');
    output.push_str(get_parameter("chained_pawn_opening", context.parameters.chained_pawn_opening).as_str());
    output.push_str(get_parameter("chained_pawn_ending", context.parameters.chained_pawn_ending).as_str());
    output.push('\n');
    output.push_str(get_parameter("passed_pawn_opening", context.parameters.passed_pawn_opening).as_str());
    output.push_str(get_parameter("passed_pawn_ending", context.parameters.passed_pawn_ending).as_str());
    output.push('\n');
    output.push_str(get_parameter("pawn_shield_opening", context.parameters.pawn_shield_opening).as_str());
    output.push_str(get_parameter("pawn_shield_ending", context.parameters.pawn_shield_ending).as_str());
    output.push('\n');
    output.push_str(get_parameter("pawn_shield_open_file_opening", context.parameters.pawn_shield_open_file_opening).as_str());
    output.push_str(get_parameter("pawn_shield_open_file_ending", context.parameters.pawn_shield_open_file_ending).as_str());
    output.push('\n');
    output.push_str(get_parameter("king_attacked_squares_opening", context.parameters.king_attacked_squares_opening).as_str());
    output.push_str(get_parameter("king_attacked_squares_ending", context.parameters.king_attacked_squares_ending).as_str());
    output.push('\n');
    output.push_str("            pst: [[[[0; 64]; 2]; 6]; 2],\n");
    output.push_str("            pst_patterns: [[[0; 64]; 2]; 6],\n");
    output.push_str("        };\n");
    output.push('\n');
    output.push_str("        evaluation_parameters.set_default_pst_patterns();\n");
    output.push_str("        evaluation_parameters.recalculate();\n");
    output.push_str("        evaluation_parameters\n");
    output.push_str("    }\n");
    output.push_str("}\n");

    let path = Path::new(output_directory);
    fs::create_dir_all(path).unwrap();

    let path = path.join("parameters.rs");
    write!(&mut File::create(path).unwrap(), "{}", output).unwrap();
}

/// Generates piece-square tables (Rust source file with current evaluation parameters), and saves it into the `output_directory`.
fn write_piece_square_table(output_directory: &str, best_error: f64, name: &str, patterns: &[[i16; 64]; 2]) {
    let mut output = String::new();
    let function_signature = format!("    pub fn get_{}_pst_pattern(&self) -> [[i16; 64]; 2] {{\n", name);

    output.push_str(get_header(best_error).as_str());
    output.push('\n');
    output.push_str("use super::*;\n");
    output.push('\n');
    output.push_str("impl EvaluationParameters {\n");
    output.push_str("    #[rustfmt::skip]\n");
    output.push_str(&function_signature);
    output.push_str("        [\n");
    output.push_str("            [\n");
    output.push_str(get_piece_square_table(&patterns[0]).as_str());
    output.push_str("            ],\n");
    output.push_str("            [\n");
    output.push_str(get_piece_square_table(&patterns[1]).as_str());
    output.push_str("            ],\n");
    output.push_str("        ]\n");
    output.push_str("    }\n");
    output.push_str("}\n");

    let path = Path::new(output_directory).join("pst");
    fs::create_dir_all(path).unwrap();

    let path = Path::new(output_directory).join("pst").join(format!("{}.rs", name));
    write!(&mut File::create(path).unwrap(), "{}", output).unwrap();
}

/// Gets a generated Rust source file header with timestamp and `best_error`.
fn get_header(best_error: f64) -> String {
    let mut output = String::new();

    let timestamp = common::time::get_unix_timestamp();
    let datetime = common::time::unix_timestamp_to_datetime(timestamp);
    let datetime_formatted =
        format!("{:0>2}-{:0>2}-{} {:0>2}:{:0>2}:{:0>2}", datetime.day, datetime.month, datetime.year, datetime.hour, datetime.minute, datetime.day);

    output.push_str("// --------------------------------------------------- //\n");
    output.push_str(format!("// Generated at {} UTC (e = {:.6}) //\n", datetime_formatted, best_error).as_str());
    output.push_str("// --------------------------------------------------- //\n");
    output
}

/// Gets a Rust representation of the piece `values` array.
fn get_array<T>(name: &str, values: &[T]) -> String
where
    T: Display,
{
    format!("            {}: [{}, {}, {}, {}, {}, {}],\n", name, values[0], values[1], values[2], values[3], values[4], values[5])
}

/// Gets a Rust representation of the parameter with the specified `name` and `value`.
fn get_parameter<T>(name: &str, value: T) -> String
where
    T: Display,
{
    format!("            {}: {},\n", name, value)
}

/// Gets a Rust representation of the piece-square tables with the specified `values`.
fn get_piece_square_table(values: &[i16]) -> String {
    let mut output = String::new();

    output.push_str("                ");
    for index in 0..64 {
        output.push_str(format!("{:4}", values[index]).as_str());
        if index % 8 == 7 {
            output.push_str(",\n");
            if index != 63 {
                output.push_str("                ");
            }
        } else {
            output.push_str(", ");
        }
    }

    output
}
