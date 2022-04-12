use crate::evaluation;
use crate::evaluation::parameters::*;
use crate::evaluation::pst;
use crate::evaluation::pst::bishop;
use crate::evaluation::pst::king;
use crate::evaluation::pst::knight;
use crate::evaluation::pst::pawn;
use crate::evaluation::pst::queen;
use crate::evaluation::pst::rook;
use crate::state::board::Bitboard;
use crate::state::fen;
use crate::state::*;
use chrono::Utc;
use nameof::name_of;
use std::cell::UnsafeCell;
use std::fs;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;
use std::path::Path;
use std::sync::Arc;
use std::thread;

struct TunerContext {
    positions: UnsafeCell<Vec<TunerPosition>>,
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
    pub fn new(positions: UnsafeCell<Vec<TunerPosition>>) -> Self {
        Self { positions }
    }
}

unsafe impl Sync for TunerContext {}

impl TunerPosition {
    /// Constructs a new instance of [TunerPosition] with stored `board` and `result`.
    pub fn new(board: Bitboard, result: f64) -> Self {
        Self { board, result }
    }
}

impl TunerParameter {
    /// Constructs a new instance of [TunerParameter] with stored `value`, `min`, `min_init`, `max_init` and `max`.
    pub fn new(value: i16, min: i16, min_init: i16, max_init: i16, max: i16) -> Self {
        Self {
            value,
            min,
            min_init,
            max_init,
            max,
        }
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
        Err(message) => {
            println!("{}", message);
            return;
        }
    };
    unsafe { println!("Loaded {} positions, starting tuner", (*positions.get()).len()) };

    let context = Arc::new(TunerContext::new(positions));
    let mut tendence = Vec::new();
    unsafe { tendence.resize((*context.positions.get()).len(), 1i8) };

    let mut best_values = load_values(lock_material, random_values);
    save_values(&mut best_values, lock_material);

    let mut best_error = calculate_error(&context, 1.13, threads_count);
    let mut improved = true;
    let mut iterations_count = 0;

    while improved {
        improved = false;
        iterations_count += 1;

        let mut changes = 0;
        let last_best_error = best_error;
        let start_time = Utc::now();

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

                save_values(&mut values, lock_material);
                calculate_error(&context, 1.13, threads_count)
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

                    save_values(&mut values, lock_material);
                    calculate_error(&context, 1.13, threads_count)
                };

                if error < best_error {
                    tendence[value_index] = 2 * -step.signum() as i8;
                    best_error = error;
                    best_values = values;
                    improved = true;
                    value_changed = true;
                    changes += 1;

                    println!(
                        "Value {} changed by {} (tendence change, new error: {:.6})",
                        value_index,
                        -step.signum(),
                        best_error
                    );
                }
            }

            if !value_changed {
                println!("Value {} skipped", value_index);

                // Step may be too big, reset it
                tendence[value_index] = step.signum() as i8;
            }
        }

        unsafe {
            write_evaluation_parameters(output_directory, best_error);
            write_piece_square_table(output_directory, best_error, "pawn", &pawn::PATTERN[0], &pawn::PATTERN[1]);
            write_piece_square_table(output_directory, best_error, "knight", &knight::PATTERN[0], &knight::PATTERN[1]);
            write_piece_square_table(output_directory, best_error, "bishop", &bishop::PATTERN[0], &bishop::PATTERN[1]);
            write_piece_square_table(output_directory, best_error, "rook", &rook::PATTERN[0], &rook::PATTERN[1]);
            write_piece_square_table(output_directory, best_error, "queen", &queen::PATTERN[0], &queen::PATTERN[1]);
            write_piece_square_table(output_directory, best_error, "king", &king::PATTERN[0], &king::PATTERN[1]);
        }

        println!(
            "Iteration {} done in {} seconds, {} changes made, error reduced from {:.6} to {:.6} ({:.6})",
            iterations_count,
            (Utc::now() - start_time).num_seconds(),
            changes,
            last_best_error,
            best_error,
            last_best_error - best_error
        );
    }
}

/// Tests the correctness of [load_values] and [save_values] methods.
pub fn validate() -> bool {
    let mut values = load_values(false, false);
    save_values(&mut values, false);

    let values_after_save = load_values(false, false);
    values.iter().zip(&values_after_save).all(|(a, b)| a.value == b.value)
}

/// Loads positions from the `epd_filename` and parses them into a list of [TunerPosition]. Returns [Err] with a proper error message if the
/// file couldn't be parsed.
fn load_positions(epd_filename: &str) -> Result<UnsafeCell<Vec<TunerPosition>>, &'static str> {
    let mut positions = Vec::new();
    let file = match File::open(epd_filename) {
        Ok(value) => value,
        Err(_) => return Err("Can't open EPD file"),
    };

    for line in BufReader::new(file).lines() {
        let position = line.unwrap();
        let parsed_epd = fen::epd_to_board(position.as_str())?;

        if parsed_epd.comment == None {
            return Err("Invalid game result");
        }

        let result = match parsed_epd.comment.unwrap().as_str() {
            "1-0" => 1.0,
            "0-1" => 0.0,
            "1/2-1/2" => 0.5,
            _ => return Err("Invalid game result"),
        };

        positions.push(TunerPosition::new(parsed_epd.board, result));
    }

    Ok(UnsafeCell::new(positions))
}

/// Calculates an error by evaluating all loaded positions with the currently set evaluation parameters. Multithreading is supported by `threads_count`.
fn calculate_error(context: &Arc<TunerContext>, scaling_constant: f64, threads_count: usize) -> f64 {
    unsafe {
        let mut threads = Vec::new();
        let positions_count = (*context.positions.get()).len();

        for thread_index in 0..threads_count {
            let context_arc = context.clone();
            threads.push(thread::spawn(move || {
                let mut sum_of_errors = 0.0;
                let from = thread_index * (positions_count / threads_count);
                let mut to = (thread_index + 1) * (positions_count / threads_count);

                // Add rest of the positions which didn't fit in the last thread
                if to + (positions_count % threads_count) == positions_count {
                    to = positions_count;
                }

                for position in &mut (*context_arc.positions.get())[from..to] {
                    position.board.recalculate_incremental_values();

                    let evaluation = position.board.evaluate_without_cache() as f64;
                    let sigmoid = 1.0 / (1.0 + 10.0f64.powf(-scaling_constant * evaluation / 400.0));
                    sum_of_errors += (position.result - sigmoid).powi(2);
                }

                sum_of_errors
            }));
        }

        let mut sum_of_errors = 0.0;

        for thread in threads {
            sum_of_errors += thread.join().unwrap();
        }

        sum_of_errors / (positions_count as f64)
    }
}

/// Transforms the current evaluation values into a list of [TunerParameter]. Use `lock_material` if the parameters related to piece values should
/// be skipped, and `random_values` if the parameters should have random values (useful when initializing tuner).
fn load_values(lock_material: bool, random_values: bool) -> Vec<TunerParameter> {
    let mut parameters = Vec::new();
    unsafe {
        if !lock_material {
            parameters.push(TunerParameter::new(PIECE_VALUE[PAWN as usize], 100, 100, 100, 100));
            parameters.push(TunerParameter::new(PIECE_VALUE[KNIGHT as usize], 0, 300, 400, 9999));
            parameters.push(TunerParameter::new(PIECE_VALUE[BISHOP as usize], 0, 300, 400, 9999));
            parameters.push(TunerParameter::new(PIECE_VALUE[ROOK as usize], 0, 400, 600, 9999));
            parameters.push(TunerParameter::new(PIECE_VALUE[QUEEN as usize], 0, 900, 1200, 9999));
            parameters.push(TunerParameter::new(PIECE_VALUE[KING as usize], 10000, 10000, 10000, 10000));
        }

        parameters.push(TunerParameter::new(PIECE_MOBILITY_OPENING[PAWN as usize], 0, 2, 6, 10));
        parameters.push(TunerParameter::new(PIECE_MOBILITY_OPENING[KNIGHT as usize], 0, 2, 6, 10));
        parameters.push(TunerParameter::new(PIECE_MOBILITY_OPENING[BISHOP as usize], 0, 2, 6, 10));
        parameters.push(TunerParameter::new(PIECE_MOBILITY_OPENING[ROOK as usize], 0, 2, 6, 10));
        parameters.push(TunerParameter::new(PIECE_MOBILITY_OPENING[QUEEN as usize], 0, 2, 6, 10));
        parameters.push(TunerParameter::new(PIECE_MOBILITY_OPENING[KING as usize], 0, 2, 6, 10));

        parameters.push(TunerParameter::new(PIECE_MOBILITY_ENDING[PAWN as usize], 0, 2, 6, 10));
        parameters.push(TunerParameter::new(PIECE_MOBILITY_ENDING[KNIGHT as usize], 0, 2, 6, 10));
        parameters.push(TunerParameter::new(PIECE_MOBILITY_ENDING[BISHOP as usize], 0, 2, 6, 10));
        parameters.push(TunerParameter::new(PIECE_MOBILITY_ENDING[ROOK as usize], 0, 2, 6, 10));
        parameters.push(TunerParameter::new(PIECE_MOBILITY_ENDING[QUEEN as usize], 0, 2, 6, 10));
        parameters.push(TunerParameter::new(PIECE_MOBILITY_ENDING[KING as usize], 0, 2, 6, 10));

        parameters.push(TunerParameter::new(PIECE_MOBILITY_CENTER_MULTIPLIER[PAWN as usize], 0, 2, 6, 10));
        parameters.push(TunerParameter::new(PIECE_MOBILITY_CENTER_MULTIPLIER[KNIGHT as usize], 0, 2, 6, 10));
        parameters.push(TunerParameter::new(PIECE_MOBILITY_CENTER_MULTIPLIER[BISHOP as usize], 0, 2, 6, 10));
        parameters.push(TunerParameter::new(PIECE_MOBILITY_CENTER_MULTIPLIER[ROOK as usize], 0, 2, 6, 10));
        parameters.push(TunerParameter::new(PIECE_MOBILITY_CENTER_MULTIPLIER[QUEEN as usize], 0, 2, 6, 10));
        parameters.push(TunerParameter::new(PIECE_MOBILITY_CENTER_MULTIPLIER[KING as usize], 0, 2, 6, 10));

        parameters.push(TunerParameter::new(DOUBLED_PAWN_OPENING, -999, -40, -10, 999));
        parameters.push(TunerParameter::new(DOUBLED_PAWN_ENDING, -999, -40, -10, 999));

        parameters.push(TunerParameter::new(ISOLATED_PAWN_OPENING, -999, -40, -10, 999));
        parameters.push(TunerParameter::new(ISOLATED_PAWN_ENDING, -999, -40, -10, 999));

        parameters.push(TunerParameter::new(CHAINED_PAWN_OPENING, -999, 10, 40, 999));
        parameters.push(TunerParameter::new(CHAINED_PAWN_ENDING, -999, 10, 40, 999));

        parameters.push(TunerParameter::new(PASSING_PAWN_OPENING, -999, 10, 40, 999));
        parameters.push(TunerParameter::new(PASSING_PAWN_ENDING, -999, 10, 40, 999));

        parameters.push(TunerParameter::new(PAWN_SHIELD_OPENING, -999, 10, 40, 999));
        parameters.push(TunerParameter::new(PAWN_SHIELD_ENDING, -999, 10, 40, 999));

        parameters.push(TunerParameter::new(PAWN_SHIELD_OPEN_FILE_OPENING, -999, -40, -10, 999));
        parameters.push(TunerParameter::new(PAWN_SHIELD_OPEN_FILE_ENDING, -999, -40, -10, 999));

        parameters.push(TunerParameter::new(KING_ATTACKED_FIELDS_OPENING, -999, -40, -10, 999));
        parameters.push(TunerParameter::new(KING_ATTACKED_FIELDS_ENDING, -999, -40, -10, 999));

        parameters.append(&mut pawn::PATTERN[0].iter().map(|v| TunerParameter::new(*v as i16, -999, -40, 40, 999)).collect());
        parameters.append(&mut pawn::PATTERN[1].iter().map(|v| TunerParameter::new(*v as i16, -999, -40, 40, 999)).collect());

        parameters.append(&mut knight::PATTERN[0].iter().map(|v| TunerParameter::new(*v as i16, -999, -40, 40, 999)).collect());
        parameters.append(&mut knight::PATTERN[1].iter().map(|v| TunerParameter::new(*v as i16, -999, -40, 40, 999)).collect());

        parameters.append(&mut bishop::PATTERN[0].iter().map(|v| TunerParameter::new(*v as i16, -999, -40, 40, 999)).collect());
        parameters.append(&mut bishop::PATTERN[1].iter().map(|v| TunerParameter::new(*v as i16, -999, -40, 40, 999)).collect());

        parameters.append(&mut rook::PATTERN[0].iter().map(|v| TunerParameter::new(*v as i16, -999, -40, 40, 999)).collect());
        parameters.append(&mut rook::PATTERN[1].iter().map(|v| TunerParameter::new(*v as i16, -999, -40, 40, 999)).collect());

        parameters.append(&mut queen::PATTERN[0].iter().map(|v| TunerParameter::new(*v as i16, -999, -40, 40, 999)).collect());
        parameters.append(&mut queen::PATTERN[1].iter().map(|v| TunerParameter::new(*v as i16, -999, -40, 40, 999)).collect());

        parameters.append(&mut king::PATTERN[0].iter().map(|v| TunerParameter::new(*v as i16, -999, -40, 40, 999)).collect());
        parameters.append(&mut king::PATTERN[1].iter().map(|v| TunerParameter::new(*v as i16, -999, -40, 40, 999)).collect());
    }

    if random_values {
        fastrand::seed(Utc::now().timestamp() as u64);
        for parameter in &mut parameters {
            (*parameter).value = fastrand::i16(parameter.min_init..=parameter.max_init);
        }
    }

    for parameter in &mut parameters {
        (*parameter).value = (*parameter).value.clamp(parameter.min, parameter.max);
    }

    parameters
}

/// Transforms `values` into the evaluation parameters, which can be used during real evaluation. Use `lock_material` if the parameters
/// related to piece values should be skipped.
fn save_values(values: &mut Vec<TunerParameter>, lock_material: bool) {
    let mut index = 0;
    unsafe {
        if !lock_material {
            save_values_to_i16_array_internal(values, &mut PIECE_VALUE, &mut index);
        }

        save_values_to_i16_array_internal(values, &mut PIECE_MOBILITY_OPENING, &mut index);
        save_values_to_i16_array_internal(values, &mut PIECE_MOBILITY_ENDING, &mut index);
        save_values_to_i16_array_internal(values, &mut PIECE_MOBILITY_CENTER_MULTIPLIER, &mut index);

        save_values_internal(values, &mut DOUBLED_PAWN_OPENING, &mut index);
        save_values_internal(values, &mut DOUBLED_PAWN_ENDING, &mut index);

        save_values_internal(values, &mut ISOLATED_PAWN_OPENING, &mut index);
        save_values_internal(values, &mut ISOLATED_PAWN_ENDING, &mut index);

        save_values_internal(values, &mut CHAINED_PAWN_OPENING, &mut index);
        save_values_internal(values, &mut CHAINED_PAWN_ENDING, &mut index);

        save_values_internal(values, &mut PASSING_PAWN_OPENING, &mut index);
        save_values_internal(values, &mut PASSING_PAWN_ENDING, &mut index);

        save_values_internal(values, &mut PAWN_SHIELD_OPENING, &mut index);
        save_values_internal(values, &mut PAWN_SHIELD_ENDING, &mut index);

        save_values_internal(values, &mut PAWN_SHIELD_OPEN_FILE_OPENING, &mut index);
        save_values_internal(values, &mut PAWN_SHIELD_OPEN_FILE_ENDING, &mut index);

        save_values_internal(values, &mut KING_ATTACKED_FIELDS_OPENING, &mut index);
        save_values_internal(values, &mut KING_ATTACKED_FIELDS_ENDING, &mut index);

        save_values_to_i8_array_internal(values, &mut pawn::PATTERN[0], &mut index);
        save_values_to_i8_array_internal(values, &mut pawn::PATTERN[1], &mut index);

        save_values_to_i8_array_internal(values, &mut knight::PATTERN[0], &mut index);
        save_values_to_i8_array_internal(values, &mut knight::PATTERN[1], &mut index);

        save_values_to_i8_array_internal(values, &mut bishop::PATTERN[0], &mut index);
        save_values_to_i8_array_internal(values, &mut bishop::PATTERN[1], &mut index);

        save_values_to_i8_array_internal(values, &mut rook::PATTERN[0], &mut index);
        save_values_to_i8_array_internal(values, &mut rook::PATTERN[1], &mut index);

        save_values_to_i8_array_internal(values, &mut queen::PATTERN[0], &mut index);
        save_values_to_i8_array_internal(values, &mut queen::PATTERN[1], &mut index);

        save_values_to_i8_array_internal(values, &mut king::PATTERN[0], &mut index);
        save_values_to_i8_array_internal(values, &mut king::PATTERN[1], &mut index);
    }

    pst::init();
    evaluation::init();
}

/// Saves `index`-th evaluation parameter stored in `values` in the `destination`.
fn save_values_internal(values: &mut Vec<TunerParameter>, destination: &mut i16, index: &mut usize) {
    *destination = values[*index].value;
    *index += 1;
}

/// Saves [i8] array starting at the `index` of `values` in the `array`.
fn save_values_to_i8_array_internal(values: &mut Vec<TunerParameter>, array: &mut [i16], index: &mut usize) {
    array.copy_from_slice(&values[*index..(*index + array.len())].iter().map(|v| (*v).value).collect::<Vec<i16>>());
    *index += array.len();
}

/// Saves [i16] array starting at the `index` of `values` in the `array`.
fn save_values_to_i16_array_internal(values: &mut Vec<TunerParameter>, array: &mut [i16], index: &mut usize) {
    array.copy_from_slice(&values[*index..(*index + array.len())].iter().map(|v| (*v).value).collect::<Vec<i16>>());
    *index += array.len();
}

/// Generates `parameters.rs` file with current evaluation parameters, and saves it into the `output_directory`.
fn write_evaluation_parameters(output_directory: &str, best_error: f64) {
    let mut output = String::new();
    unsafe {
        output.push_str(get_header(best_error).as_str());
        output.push_str("\n");
        output.push_str(get_array(name_of!(PIECE_VALUE), &PIECE_VALUE).as_str());
        output.push_str("\n");
        output.push_str(get_array(name_of!(PIECE_MOBILITY_OPENING), &PIECE_MOBILITY_OPENING).as_str());
        output.push_str(get_array(name_of!(PIECE_MOBILITY_ENDING), &PIECE_MOBILITY_ENDING).as_str());
        output.push_str(get_array(name_of!(PIECE_MOBILITY_CENTER_MULTIPLIER), &PIECE_MOBILITY_CENTER_MULTIPLIER).as_str());
        output.push_str("\n");
        output.push_str(get_parameter(name_of!(DOUBLED_PAWN_OPENING), DOUBLED_PAWN_OPENING).as_str());
        output.push_str(get_parameter(name_of!(DOUBLED_PAWN_ENDING), DOUBLED_PAWN_ENDING).as_str());
        output.push_str("\n");
        output.push_str(get_parameter(name_of!(ISOLATED_PAWN_OPENING), ISOLATED_PAWN_OPENING).as_str());
        output.push_str(get_parameter(name_of!(ISOLATED_PAWN_ENDING), ISOLATED_PAWN_ENDING).as_str());
        output.push_str("\n");
        output.push_str(get_parameter(name_of!(CHAINED_PAWN_OPENING), CHAINED_PAWN_OPENING).as_str());
        output.push_str(get_parameter(name_of!(CHAINED_PAWN_ENDING), CHAINED_PAWN_ENDING).as_str());
        output.push_str("\n");
        output.push_str(get_parameter(name_of!(PASSING_PAWN_OPENING), PASSING_PAWN_OPENING).as_str());
        output.push_str(get_parameter(name_of!(PASSING_PAWN_ENDING), PASSING_PAWN_ENDING).as_str());
        output.push_str("\n");
        output.push_str(get_parameter(name_of!(PAWN_SHIELD_OPENING), PAWN_SHIELD_OPENING).as_str());
        output.push_str(get_parameter(name_of!(PAWN_SHIELD_ENDING), PAWN_SHIELD_ENDING).as_str());
        output.push_str("\n");
        output.push_str(get_parameter(name_of!(PAWN_SHIELD_OPEN_FILE_OPENING), PAWN_SHIELD_OPEN_FILE_OPENING).as_str());
        output.push_str(get_parameter(name_of!(PAWN_SHIELD_OPEN_FILE_ENDING), PAWN_SHIELD_OPEN_FILE_ENDING).as_str());
        output.push_str("\n");
        output.push_str(get_parameter(name_of!(KING_ATTACKED_FIELDS_OPENING), KING_ATTACKED_FIELDS_OPENING).as_str());
        output.push_str(get_parameter(name_of!(KING_ATTACKED_FIELDS_ENDING), KING_ATTACKED_FIELDS_ENDING).as_str());
    }

    let path = Path::new(output_directory);
    fs::create_dir_all(path).unwrap();

    let path = path.join("parameters.rs");
    write!(&mut File::create(path).unwrap(), "{}", output).unwrap();
}

/// Generates piece-square tables (Rust source file with current evaluation parameters), and saves it into the `output_directory`.
fn write_piece_square_table(output_directory: &str, best_error: f64, name: &str, opening: &[i16], ending: &[i16]) {
    let mut output = String::new();

    output.push_str(get_header(best_error).as_str());
    output.push_str("\n");
    output.push_str("#[rustfmt::skip]\n");
    output.push_str("pub static mut PATTERN: [[i16; 64]; 2] =\n");
    output.push_str("[\n");
    output.push_str("    [\n");
    output.push_str(get_piece_square_table(opening).as_str());
    output.push_str("    ],\n");
    output.push_str("    [\n");
    output.push_str(get_piece_square_table(ending).as_str());
    output.push_str("    ],\n");
    output.push_str("];\n");

    let path = Path::new(output_directory).join("pst");
    fs::create_dir_all(path).unwrap();

    let path = Path::new(output_directory).join("pst").join(format!("{}.rs", name));
    write!(&mut File::create(path).unwrap(), "{}", output).unwrap();
}

/// Gets the generated Rust source file header with timestamp and `best_error`.
fn get_header(best_error: f64) -> String {
    let mut output = String::new();

    output.push_str("// --------------------------------------------------- //\n");
    output.push_str(format!("// Generated at {} UTC (e = {:.6}) //\n", Utc::now().format("%Y-%m-%d %H:%M:%S"), best_error).as_str());
    output.push_str("// --------------------------------------------------- //\n");
    output
}

/// Gets the Rust representation of the piece `values` array.
fn get_array(name: &str, values: &[i16]) -> String {
    format!(
        "pub static mut {}: [i16; 6] = [{}, {}, {}, {}, {}, {}];\n",
        name, values[0], values[1], values[2], values[3], values[4], values[5]
    )
}

/// Gets the Rust representation of the parameter with the specified `name` and `value`.
fn get_parameter(name: &str, value: i16) -> String {
    format!("pub static mut {}: i16 = {};\n", name, value)
}

/// Gets the Rust representation of the piece-square tables with the specified `values`.
fn get_piece_square_table(values: &[i16]) -> String {
    let mut output = String::new();

    output.push_str("        ");
    for index in 0..64 {
        output.push_str(format!("{:3}, ", values[index]).as_str());
        if index % 8 == 7 {
            output.push_str("\n");
            if index != 63 {
                output.push_str("        ");
            }
        }
    }

    output
}
