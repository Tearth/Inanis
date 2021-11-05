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
use crate::state::fen::fen_to_board;
use chrono::Utc;
use nameof::name_of;
use std::fs;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;
use std::path::Path;

struct TunerPosition {
    board: Bitboard,
    result: f64,
}

impl TunerPosition {
    pub fn new(board: Bitboard, result: f64) -> TunerPosition {
        TunerPosition { board, result }
    }
}

pub fn run(epd_filename: &str, output_directory: &str, lock_material: bool, random_values: bool) {
    println!("Loading EPD file...");
    let mut positions = match load_positions(epd_filename) {
        Ok(value) => value,
        Err(message) => {
            println!("{}", message);
            return;
        }
    };
    println!("Loaded {} positions, starting tuner", positions.len());

    let mut tendence = Vec::new();
    tendence.resize(positions.len(), 1i8);

    let mut best_values = load_values(lock_material, random_values);
    save_values(&mut best_values, lock_material);

    let mut best_error = calculate_error(&mut positions, 1.13);
    let mut improved = true;
    let mut iterations_count = 0;

    while improved {
        improved = false;
        iterations_count += 1;

        let mut changes = 0;
        let last_best_error = best_error;
        let start_time = Utc::now();

        for value_index in 0..best_values.len() {
            // Ignore king value (no point to tune it)
            if !lock_material && value_index == 5 {
                continue;
            }

            let step = tendence[value_index] as i16;
            let mut values = best_values.to_vec();
            let mut value_changed = false;

            values[value_index] += step;
            save_values(&mut values, lock_material);

            let error = calculate_error(&mut positions, 1.13);
            if error < best_error {
                tendence[value_index] *= 2;
                best_error = error;
                best_values = values;
                improved = true;
                value_changed = true;
                changes += 1;

                println!("Value {} changed by {} (new error: {:.6})", value_index, step, best_error);
            } else if error > best_error {
                values[value_index] -= step;
                values[value_index] -= step.signum();
                save_values(&mut values, lock_material);

                let error = calculate_error(&mut positions, 1.13);
                if error < best_error {
                    tendence[value_index] = -step.signum() as i8;
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

pub fn validate() -> bool {
    let mut values = load_values(false, false);
    save_values(&mut values, false);

    let values_after_save = load_values(false, false);
    values.iter().zip(&values_after_save).all(|(a, b)| a == b)
}

fn load_positions(epd_filename: &str) -> Result<Vec<TunerPosition>, &'static str> {
    let mut positions = Vec::new();
    let file = match File::open(epd_filename) {
        Ok(value) => value,
        Err(_) => return Err("Can't open EPD file"),
    };

    for line in BufReader::new(file).lines() {
        let position = line.unwrap();
        let board = fen_to_board(position.as_str())?;
        let result = if position.contains("1-0") {
            1.0
        } else if position.contains("0-1") {
            0.0
        } else if position.contains("1/2-1/2") {
            0.5
        } else {
            return Err("Invalid game result");
        };

        positions.push(TunerPosition::new(board, result));
    }

    Ok(positions)
}

fn calculate_error(positions: &mut Vec<TunerPosition>, scaling_constant: f64) -> f64 {
    let mut sum_of_errors = 0.0;
    let positions_count = positions.len();

    for position in positions {
        position.board.recalculate_incremental_values();

        let evaluation = position.board.evaluate_without_cache() as f64;
        let sigmoid = 1.0 / (1.0 + 10.0f64.powf(-scaling_constant * evaluation / 400.0));
        sum_of_errors += (position.result - sigmoid).powi(2);
    }

    sum_of_errors / (positions_count as f64)
}

fn load_values(lock_material: bool, random_values: bool) -> Vec<i16> {
    let mut values = Vec::new();

    if !lock_material {
        values.append(unsafe { &mut PIECE_VALUE.to_vec() });
    }

    values.push(unsafe { MOBILITY_OPENING });
    values.push(unsafe { MOBILITY_ENDING });

    values.push(unsafe { DOUBLED_PAWN_OPENING });
    values.push(unsafe { DOUBLED_PAWN_ENDING });

    values.push(unsafe { ISOLATED_PAWN_OPENING });
    values.push(unsafe { ISOLATED_PAWN_ENDING });

    values.push(unsafe { CHAINED_PAWN_OPENING });
    values.push(unsafe { CHAINED_PAWN_ENDING });

    values.push(unsafe { PASSING_PAWN_OPENING });
    values.push(unsafe { PASSING_PAWN_ENDING });

    values.push(unsafe { PAWN_SHIELD_OPENING });
    values.push(unsafe { PAWN_SHIELD_ENDING });

    values.push(unsafe { PAWN_SHIELD_OPEN_FILE_OPENING });
    values.push(unsafe { PAWN_SHIELD_OPEN_FILE_ENDING });

    values.push(unsafe { KING_ATTACKED_FIELDS_OPENING });
    values.push(unsafe { KING_ATTACKED_FIELDS_ENDING });

    values.append(unsafe { &mut pawn::PATTERN[0].iter().map(|v| *v as i16).collect::<Vec<i16>>() });
    values.append(unsafe { &mut pawn::PATTERN[1].iter().map(|v| *v as i16).collect::<Vec<i16>>() });

    values.append(unsafe { &mut knight::PATTERN[0].iter().map(|v| *v as i16).collect::<Vec<i16>>() });
    values.append(unsafe { &mut knight::PATTERN[1].iter().map(|v| *v as i16).collect::<Vec<i16>>() });

    values.append(unsafe { &mut bishop::PATTERN[0].iter().map(|v| *v as i16).collect::<Vec<i16>>() });
    values.append(unsafe { &mut bishop::PATTERN[1].iter().map(|v| *v as i16).collect::<Vec<i16>>() });

    values.append(unsafe { &mut rook::PATTERN[0].iter().map(|v| *v as i16).collect::<Vec<i16>>() });
    values.append(unsafe { &mut rook::PATTERN[1].iter().map(|v| *v as i16).collect::<Vec<i16>>() });

    values.append(unsafe { &mut queen::PATTERN[0].iter().map(|v| *v as i16).collect::<Vec<i16>>() });
    values.append(unsafe { &mut queen::PATTERN[1].iter().map(|v| *v as i16).collect::<Vec<i16>>() });

    values.append(unsafe { &mut king::PATTERN[0].iter().map(|v| *v as i16).collect::<Vec<i16>>() });
    values.append(unsafe { &mut king::PATTERN[1].iter().map(|v| *v as i16).collect::<Vec<i16>>() });

    if random_values {
        for value in &mut values {
            *value = fastrand::i16(-16..16);
        }
    }

    values
}

fn save_values(values: &mut Vec<i16>, lock_material: bool) {
    let mut index = 0;

    if !lock_material {
        save_values_to_i16_array_internal(values, unsafe { &mut PIECE_VALUE }, &mut index);
    }

    save_values_internal(values, unsafe { &mut MOBILITY_OPENING }, &mut index);
    save_values_internal(values, unsafe { &mut MOBILITY_ENDING }, &mut index);

    save_values_internal(values, unsafe { &mut DOUBLED_PAWN_OPENING }, &mut index);
    save_values_internal(values, unsafe { &mut DOUBLED_PAWN_ENDING }, &mut index);

    save_values_internal(values, unsafe { &mut ISOLATED_PAWN_OPENING }, &mut index);
    save_values_internal(values, unsafe { &mut ISOLATED_PAWN_ENDING }, &mut index);

    save_values_internal(values, unsafe { &mut CHAINED_PAWN_OPENING }, &mut index);
    save_values_internal(values, unsafe { &mut CHAINED_PAWN_ENDING }, &mut index);

    save_values_internal(values, unsafe { &mut PASSING_PAWN_OPENING }, &mut index);
    save_values_internal(values, unsafe { &mut PASSING_PAWN_ENDING }, &mut index);

    save_values_internal(values, unsafe { &mut PAWN_SHIELD_OPENING }, &mut index);
    save_values_internal(values, unsafe { &mut PAWN_SHIELD_ENDING }, &mut index);

    save_values_internal(values, unsafe { &mut PAWN_SHIELD_OPEN_FILE_OPENING }, &mut index);
    save_values_internal(values, unsafe { &mut PAWN_SHIELD_OPEN_FILE_ENDING }, &mut index);

    save_values_internal(values, unsafe { &mut KING_ATTACKED_FIELDS_OPENING }, &mut index);
    save_values_internal(values, unsafe { &mut KING_ATTACKED_FIELDS_ENDING }, &mut index);

    save_values_to_i8_array_internal(values, unsafe { &mut pawn::PATTERN[0] }, &mut index);
    save_values_to_i8_array_internal(values, unsafe { &mut pawn::PATTERN[1] }, &mut index);

    save_values_to_i8_array_internal(values, unsafe { &mut knight::PATTERN[0] }, &mut index);
    save_values_to_i8_array_internal(values, unsafe { &mut knight::PATTERN[1] }, &mut index);

    save_values_to_i8_array_internal(values, unsafe { &mut bishop::PATTERN[0] }, &mut index);
    save_values_to_i8_array_internal(values, unsafe { &mut bishop::PATTERN[1] }, &mut index);

    save_values_to_i8_array_internal(values, unsafe { &mut rook::PATTERN[0] }, &mut index);
    save_values_to_i8_array_internal(values, unsafe { &mut rook::PATTERN[1] }, &mut index);

    save_values_to_i8_array_internal(values, unsafe { &mut queen::PATTERN[0] }, &mut index);
    save_values_to_i8_array_internal(values, unsafe { &mut queen::PATTERN[1] }, &mut index);

    save_values_to_i8_array_internal(values, unsafe { &mut king::PATTERN[0] }, &mut index);
    save_values_to_i8_array_internal(values, unsafe { &mut king::PATTERN[1] }, &mut index);

    pst::init();
    evaluation::init();
}

fn save_values_internal(values: &mut Vec<i16>, destination: &mut i16, index: &mut usize) {
    *destination = values[*index];
    *index += 1;
}

fn save_values_to_i8_array_internal(values: &mut Vec<i16>, array: &mut [i8], index: &mut usize) {
    array.copy_from_slice(
        &values[*index..(*index + array.len())]
            .iter()
            .map(|v| (*v).clamp(i8::MIN as i16, i8::MAX as i16) as i8)
            .collect::<Vec<i8>>(),
    );
    *index += array.len();
}

fn save_values_to_i16_array_internal(values: &mut Vec<i16>, array: &mut [i16], index: &mut usize) {
    array.copy_from_slice(&values[*index..(*index + array.len())]);
    *index += array.len();
}

fn write_evaluation_parameters(output_directory: &str, best_error: f64) {
    let mut output = String::new();
    output.push_str(get_header(best_error).as_str());
    output.push_str("\n");
    output.push_str(unsafe { get_material(name_of!(PIECE_VALUE), &PIECE_VALUE).as_str() });
    output.push_str("\n");
    output.push_str(unsafe { get_parameter(name_of!(MOBILITY_OPENING), MOBILITY_OPENING).as_str() });
    output.push_str(unsafe { get_parameter(name_of!(MOBILITY_ENDING), MOBILITY_ENDING).as_str() });
    output.push_str("\n");
    output.push_str(unsafe { get_parameter(name_of!(DOUBLED_PAWN_OPENING), DOUBLED_PAWN_OPENING).as_str() });
    output.push_str(unsafe { get_parameter(name_of!(DOUBLED_PAWN_ENDING), DOUBLED_PAWN_ENDING).as_str() });
    output.push_str("\n");
    output.push_str(unsafe { get_parameter(name_of!(ISOLATED_PAWN_OPENING), ISOLATED_PAWN_OPENING).as_str() });
    output.push_str(unsafe { get_parameter(name_of!(ISOLATED_PAWN_ENDING), ISOLATED_PAWN_ENDING).as_str() });
    output.push_str("\n");
    output.push_str(unsafe { get_parameter(name_of!(CHAINED_PAWN_OPENING), CHAINED_PAWN_OPENING).as_str() });
    output.push_str(unsafe { get_parameter(name_of!(CHAINED_PAWN_ENDING), CHAINED_PAWN_ENDING).as_str() });
    output.push_str("\n");
    output.push_str(unsafe { get_parameter(name_of!(PASSING_PAWN_OPENING), PASSING_PAWN_OPENING).as_str() });
    output.push_str(unsafe { get_parameter(name_of!(PASSING_PAWN_ENDING), PASSING_PAWN_ENDING).as_str() });
    output.push_str("\n");
    output.push_str(unsafe { get_parameter(name_of!(PAWN_SHIELD_OPENING), PAWN_SHIELD_OPENING).as_str() });
    output.push_str(unsafe { get_parameter(name_of!(PAWN_SHIELD_ENDING), PAWN_SHIELD_ENDING).as_str() });
    output.push_str("\n");
    output.push_str(unsafe { get_parameter(name_of!(PAWN_SHIELD_OPEN_FILE_OPENING), PAWN_SHIELD_OPEN_FILE_OPENING).as_str() });
    output.push_str(unsafe { get_parameter(name_of!(PAWN_SHIELD_OPEN_FILE_ENDING), PAWN_SHIELD_OPEN_FILE_ENDING).as_str() });
    output.push_str("\n");
    output.push_str(unsafe { get_parameter(name_of!(KING_ATTACKED_FIELDS_OPENING), KING_ATTACKED_FIELDS_OPENING).as_str() });
    output.push_str(unsafe { get_parameter(name_of!(KING_ATTACKED_FIELDS_ENDING), KING_ATTACKED_FIELDS_ENDING).as_str() });
    output.push_str("\n");

    let path = Path::new(output_directory);
    fs::create_dir_all(path).unwrap();

    let path = path.join("parameters.rs");
    write!(&mut File::create(path).unwrap(), "{}", output.to_string()).unwrap();
}

fn write_piece_square_table(output_directory: &str, best_error: f64, name: &str, opening: &[i8], ending: &[i8]) {
    let mut output = String::new();

    output.push_str(get_header(best_error).as_str());
    output.push_str("\n");
    output.push_str("#[rustfmt::skip]\n");
    output.push_str("pub static mut PATTERN: [[i8; 64]; 2] =\n");
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
    write!(&mut File::create(path).unwrap(), "{}", output.to_string()).unwrap();
}

fn get_header(best_error: f64) -> String {
    let mut output = String::new();

    output.push_str("// --------------------------------------------------- //\n");
    output.push_str(
        format!(
            "// Generated at {} UTC (e = {:.6}) //\n",
            Utc::now().format("%Y-%m-%d %H:%M:%S"),
            best_error
        )
        .as_str(),
    );
    output.push_str("// --------------------------------------------------- //\n");
    output
}

fn get_material(name: &str, values: &[i16]) -> String {
    format!(
        "pub static mut {}: [i16; 6] = [{}, {}, {}, {}, {}, {}];\n",
        name, values[0], values[1], values[2], values[3], values[4], values[5]
    )
}

fn get_parameter(name: &str, value: i16) -> String {
    format!("pub static mut {}: i16 = {};\n", name, value)
}

fn get_piece_square_table(values: &[i8]) -> String {
    let mut output = String::new();

    output.push_str("        ");
    for index in 0..64 {
        output.push_str(format!("{:4}, ", values[index]).as_str());
        if index % 8 == 7 {
            output.push_str("\n");
            if index != 63 {
                output.push_str("        ");
            }
        }
    }

    output
}
