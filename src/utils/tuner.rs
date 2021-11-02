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

struct TuningPosition {
    board: Bitboard,
    result: f64,
}

impl TuningPosition {
    pub fn new(board: Bitboard, result: f64) -> TuningPosition {
        TuningPosition { board, result }
    }
}

pub fn run() {
    println!("Loading EPD file...");
    let mut positions = load_positions();
    println!("Loaded {} positions", positions.len());

    let mut best_values = load_values();
    let mut best_error = calculate_error(&mut positions, 1.13);
    let mut improved = true;

    while improved {
        improved = false;
        for value_index in 0..best_values.len() {
            // Ignore king value (no point to tune it)
            if value_index == 5 {
                continue;
            }

            let mut values = best_values.to_vec();
            let mut value_changed = false;

            values[value_index] += 1;
            save_values(&mut values);

            let error = calculate_error(&mut positions, 1.13);
            if error < best_error {
                best_error = error;
                best_values = values;
                improved = true;
                value_changed = true;

                println!("Value {} changed by {} (new error: {})", value_index, 1, best_error);
            } else if error > best_error {
                values[value_index] -= 2;
                save_values(&mut values);

                let error = calculate_error(&mut positions, 1.13);
                if error < best_error {
                    best_error = error;
                    best_values = values;
                    improved = true;
                    value_changed = true;

                    println!("Value {} changed by {} (new error: {})", value_index, -1, best_error);
                }
            }

            if !value_changed {
                println!("Value {} skipped", value_index);
            }
        }

        save_evaluation_parameters();
        save_piece_square_table("pawn", unsafe { &pawn::PATTERN[0] }, unsafe { &pawn::PATTERN[1] });
        save_piece_square_table("knight", unsafe { &knight::PATTERN[0] }, unsafe { &knight::PATTERN[1] });
        save_piece_square_table("bishop", unsafe { &bishop::PATTERN[0] }, unsafe { &bishop::PATTERN[1] });
        save_piece_square_table("rook", unsafe { &rook::PATTERN[0] }, unsafe { &rook::PATTERN[1] });
        save_piece_square_table("queen", unsafe { &queen::PATTERN[0] }, unsafe { &queen::PATTERN[1] });
        save_piece_square_table("king", unsafe { &king::PATTERN[0] }, unsafe { &king::PATTERN[1] });
    }
}

pub fn validate() -> bool {
    let mut values = load_values();
    save_values(&mut values);

    let values_after_save = load_values();
    values.iter().zip(&values_after_save).all(|(a, b)| a == b)
}

fn load_positions() -> Vec<TuningPosition> {
    let mut positions = Vec::new();
    let file = File::open("./input/quiet.epd").unwrap();

    for line in BufReader::new(file).lines() {
        let position = line.unwrap();
        let board = fen_to_board(position.as_str()).unwrap();
        let result = if position.contains("1-0") {
            1.0
        } else if position.contains("0-1") {
            0.0
        } else if position.contains("1/2-1/2") {
            0.5
        } else {
            panic!("Invalid game result: position={}", position);
        };

        positions.push(TuningPosition::new(board, result));
    }

    positions
}

fn calculate_error(positions: &mut Vec<TuningPosition>, scaling_constant: f64) -> f64 {
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

fn load_values() -> Vec<i16> {
    let mut values = Vec::new();
    values.append(unsafe { &mut PIECE_VALUE.to_vec() });

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

    values
}

fn save_values(values: &mut Vec<i16>) {
    let mut index = 0;
    save_values_to_i16_array_internal(values, unsafe { &mut PIECE_VALUE }, &mut index);

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
}

fn save_values_internal(values: &mut Vec<i16>, destination: &mut i16, index: &mut usize) {
    *destination = values[*index];
    *index += 1;
}

fn save_values_to_i8_array_internal(values: &mut Vec<i16>, array: &mut [i8], index: &mut usize) {
    array.copy_from_slice(&values[*index..(*index + array.len())].iter().map(|v| *v as i8).collect::<Vec<i8>>());
    *index += array.len();
}

fn save_values_to_i16_array_internal(values: &mut Vec<i16>, array: &mut [i16], index: &mut usize) {
    array.copy_from_slice(&values[*index..(*index + array.len())]);
    *index += array.len();
}

fn save_evaluation_parameters() {
    let mut output = String::new();
    output.push_str(get_header().as_str());
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

    fs::create_dir_all("output/").unwrap();
    write!(&mut File::create("output/parameters.rs").unwrap(), "{}", output.to_string()).unwrap();
}

fn save_piece_square_table(name: &str, opening: &[i8], ending: &[i8]) {
    let mut output = String::new();

    output.push_str(get_header().as_str());
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

    fs::create_dir_all("output/pst/").unwrap();
    write!(
        &mut File::create(format!("output/pst/{}.rs", name)).unwrap(),
        "{}",
        output.to_string()
    )
    .unwrap();
}

fn get_header() -> String {
    let mut output = String::new();

    output.push_str("// ------------------------------------ //\n");
    output.push_str(format!("// Generated at {} UTC //\n", Utc::now().format("%Y-%m-%d %H:%M:%S")).as_str());
    output.push_str("// ------------------------------------ //\n");
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
