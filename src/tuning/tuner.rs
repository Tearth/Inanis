use crate::engine::see::SEEContainer;
use crate::evaluation::material;
use crate::evaluation::mobility;
use crate::evaluation::parameters::*;
use crate::evaluation::pawns;
use crate::evaluation::pst;
use crate::evaluation::safety;
use crate::evaluation::EvaluationParameters;
use crate::state::movegen::MagicContainer;
use crate::state::patterns::PatternsContainer;
use crate::state::text::fen;
use crate::state::zobrist::ZobristContainer;
use crate::state::*;
use crate::utils::rand;
use common::time::DateTime;
use std::collections::HashSet;
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

const LEARNING_RATE: f32 = 0.1;
const K_STEP: f32 = 0.0001;
const B1: f32 = 0.9;
const B2: f32 = 0.999;
const OUTPUT_INTERVAL: i32 = 100;

pub struct TunerPosition {
    evaluation: i16,
    result_phase: u8,
    base_index: u32,
    coefficients_count: u8,
}

#[derive(Clone, Copy)]
pub struct TunerParameter {
    value: i16,
    min: i16,
    min_init: i16,
    max_init: i16,
    max: i16,
}

#[derive(Clone)]
pub struct TunerCoefficient {
    pub data: u8,
}

impl TunerPosition {
    /// Constructs a new instance of [TunerPosition] with stored `board` and `result`.
    pub fn new(evaluation: i16, result: u8, phase: u8, base_index: u32, coefficients_count: u8) -> Self {
        Self { evaluation, result_phase: (result << 5) | phase, base_index, coefficients_count }
    }

    pub fn get_result(&self) -> u8 {
        self.result_phase >> 5
    }

    pub fn get_phase(&self) -> u8 {
        self.result_phase & 0x1f
    }
}

impl TunerParameter {
    /// Constructs a new instance of [TunerParameter] with stored `value`, `min`, `min_init`, `max_init` and `max`.
    pub fn new(value: i16, min: i16, min_init: i16, max_init: i16, max: i16) -> Self {
        Self { value, min, min_init, max_init, max }
    }
}

impl TunerCoefficient {
    /// Constructs a new instance of [TunerCoefficient] with stored `value`, `phase` and `index`.
    pub fn new(value: i8, phase: usize) -> Self {
        Self { data: (((value + 32) as u8) << 1) | (phase as u8) }
    }

    pub fn get_data(&self) -> (i8, usize) {
        ((self.data as i8 >> 1) - 32, (self.data & 1) as usize)
    }
}

/// Runs tuner of evaluation parameters. The input file is specified by `epd_filename` with a list of positions and their expected results, and the `output_directory`
/// directory is used to store generated Rust sources with the optimized values. Use `random_values` to initialize evaluation parameters with random values, `k` to
/// set scaling constant (might be None) and `wdl_ratio` to set the ratio between WDL and eval. Multithreading is supported by `threads_count`. The tuner is implemented
/// using gradient descent and Adam optimizer. The result (Rust sources with the calculated values) are saved every iteration, and can be put directly into the code.
pub fn run(epd_filename: &str, output_directory: &str, random_values: bool, k: Option<f32>, wdl_ratio: f32, threads_count: usize) {
    println!("Loading EPD file...");

    let start_time = SystemTime::now();
    let mut weights_indices = HashSet::new();
    let mut weights = Vec::new();
    let mut gradients = Vec::new();
    let mut coefficients = Vec::new();
    let mut indices = Vec::new();

    let mut positions = match load_positions(epd_filename, &mut coefficients, &mut indices, &mut weights_indices) {
        Ok(value) => value,
        Err(error) => {
            println!("Invalid EPD file: {}", error);
            return;
        }
    };

    positions.shrink_to_fit();
    coefficients.shrink_to_fit();
    indices.shrink_to_fit();

    let coefficients = Arc::new(coefficients);
    let indices = Arc::new(indices);
    println!("Loaded {} positions in {} seconds, starting tuner", positions.len(), (start_time.elapsed().unwrap().as_millis() as f32) / 1000.0);

    let mut evaluation_parameters = EvaluationParameters::default();
    let mut tuner_parameters = load_values(&evaluation_parameters, random_values);

    for parameter in &tuner_parameters {
        weights.push(parameter.value as f32);
    }
    gradients.resize(weights.len(), 0.0);

    let mut m = Vec::new();
    m.resize(weights.len(), 0.0);

    let mut v = Vec::new();
    v.resize(weights.len(), 0.0);

    let mut weights_enabled = Vec::new();
    weights_enabled.resize(weights.len(), false);

    for i in 0..weights_enabled.len() {
        weights_enabled[i] = i == 5 || weights_indices.contains(&(i as u16));
    }

    drop(weights_indices);

    let k = k.unwrap_or_else(|| calculate_k(&positions, &coefficients, &indices, &weights, wdl_ratio, threads_count));
    let mut last_error = calculate_error(&positions, &coefficients, &indices, &weights, k, wdl_ratio, threads_count);
    let mut iterations_count = 0;

    println!("Scaling constant: {}", k);

    let mut start_time = SystemTime::now();
    loop {
        gradients.fill(0.0);

        // Calculate gradients for all coefficients
        thread::scope(|scope| {
            let mut threads = Vec::new();
            let weights = Arc::new(weights.clone());

            for chunk in positions.chunks(positions.len() / threads_count) {
                let weights = weights.clone();
                let coefficients = coefficients.clone();
                let indices = indices.clone();

                threads.push(scope.spawn(move || {
                    let mut gradients = vec![0.0; weights.len()];
                    for position in chunk {
                        let position_result = position.get_result() as f32 / 2.0;
                        let position_phase = position.get_phase() as f32 / INITIAL_GAME_PHASE as f32;
                        let evaluation = evaluate_position(position, &coefficients, &indices, &weights);

                        let sig = sigmoid(evaluation, k);
                        let a = ((1.0 - wdl_ratio) * sigmoid(position.evaluation as f32, k) + wdl_ratio * position_result) - sig;
                        let b = sig * (1.0 - sig);

                        for i in 0..position.coefficients_count {
                            let index = position.base_index as usize + i as usize;

                            // Ignore pawn and king values
                            if indices[index] == 5 {
                                continue;
                            }

                            let (value, phase) = coefficients[index].get_data();
                            let phase = if phase == OPENING { position_phase } else { 1.0 - position_phase };
                            let c = phase * value as f32;

                            gradients[indices[index] as usize] += a * b * c;
                        }
                    }

                    gradients
                }));
            }

            for thread in threads {
                for (i, gradient) in thread.join().unwrap().iter().enumerate() {
                    gradients[i] += gradient;
                }
            }
        });

        // Apply gradients and calculate new weights
        for i in 0..weights.len() {
            if weights_enabled[i] {
                let gradient = -2.0 * gradients[i] / positions.len() as f32;
                m[i] = B1 * m[i] + (1.0 - B1) * gradient;
                v[i] = B2 * v[i] + (1.0 - B2) * gradient.powi(2);

                weights[i] -= LEARNING_RATE * m[i] / (v[i] + 0.00000001).sqrt();
                weights[i] = weights[i].clamp(tuner_parameters[i].min as f32, tuner_parameters[i].max as f32);
            } else {
                weights[i] = 0.0;
            }
        }

        if iterations_count % OUTPUT_INTERVAL == 0 {
            for i in 0..tuner_parameters.len() {
                tuner_parameters[i].value = weights[i].round() as i16;
            }

            save_values(&mut evaluation_parameters, &mut tuner_parameters);

            let error = calculate_error(&positions, &coefficients, &indices, &weights, k, wdl_ratio, threads_count);
            write_evaluation_parameters(&evaluation_parameters, output_directory, error, k, wdl_ratio);
            write_piece_square_table(output_directory, error, k, wdl_ratio, "PAWN", &evaluation_parameters.pst_patterns[PAWN]);
            write_piece_square_table(output_directory, error, k, wdl_ratio, "KNIGHT", &evaluation_parameters.pst_patterns[KNIGHT]);
            write_piece_square_table(output_directory, error, k, wdl_ratio, "BISHOP", &evaluation_parameters.pst_patterns[BISHOP]);
            write_piece_square_table(output_directory, error, k, wdl_ratio, "ROOK", &evaluation_parameters.pst_patterns[ROOK]);
            write_piece_square_table(output_directory, error, k, wdl_ratio, "QUEEN", &evaluation_parameters.pst_patterns[QUEEN]);
            write_piece_square_table(output_directory, error, k, wdl_ratio, "KING", &evaluation_parameters.pst_patterns[KING]);

            println!(
                "Iteration {} done in {} seconds, error reduced from {:.6} to {:.6} ({:.6})",
                iterations_count,
                (start_time.elapsed().unwrap().as_millis() as f32) / 1000.0,
                last_error,
                error,
                last_error - error
            );

            last_error = error;
            start_time = SystemTime::now();
        }

        iterations_count += 1;
    }
}

fn calculate_k(positions: &[TunerPosition], coefficients: &[TunerCoefficient], indices: &[u16], weights: &[f32], wdl_ratio: f32, threads_count: usize) -> f32 {
    let mut k = 0.0;
    let mut last_error = calculate_error(positions, coefficients, indices, weights, k, wdl_ratio, threads_count);

    loop {
        let error = calculate_error(positions, coefficients, indices, weights, k + K_STEP, wdl_ratio, threads_count);
        if error >= last_error {
            break;
        }

        last_error = error;
        k += K_STEP;
    }

    k
}

/// Calculates an error by evaluating all loaded positions with the currently set evaluation parameters. Multithreading is supported by `threads_count`.
fn calculate_error(
    positions: &[TunerPosition],
    coefficients: &[TunerCoefficient],
    indices: &[u16],
    weights: &[f32],
    k: f32,
    wdl_ratio: f32,
    threads_count: usize,
) -> f32 {
    let mut sum_of_errors = 0.0;
    let positions_count = positions.len();

    thread::scope(|scope| {
        let mut threads = Vec::new();
        let weights = Arc::new(weights);

        for chunk in positions.chunks(positions_count / threads_count) {
            let weights = weights.clone();
            threads.push(scope.spawn(move || {
                let mut error = 0.0;
                for position in chunk {
                    let position_result = position.get_result() as f32 / 2.0;
                    let evaluation = evaluate_position(position, coefficients, indices, &weights);

                    error += (((1.0 - wdl_ratio) * sigmoid(position.evaluation as f32, k) + wdl_ratio * position_result) - sigmoid(evaluation, k)).powi(2);
                }

                error
            }));
        }

        for thread in threads {
            sum_of_errors += thread.join().unwrap();
        }
    });

    sum_of_errors / (positions_count as f32)
}

/// Evaluates `position` based on `weights`.
fn evaluate_position(position: &TunerPosition, coefficients: &[TunerCoefficient], indices: &[u16], weights: &[f32]) -> f32 {
    let mut opening_score = 0.0;
    let mut ending_score = 0.0;
    let position_phase = position.get_phase() as f32 / INITIAL_GAME_PHASE as f32;

    for i in 0..position.coefficients_count {
        let index = position.base_index as usize + i as usize;
        let (value, phase) = coefficients[index].get_data();
        let value = weights[indices[index] as usize] * value as f32;

        if indices[index] < 6 {
            opening_score += value;
            ending_score += value;
        } else {
            if phase == OPENING {
                opening_score += value;
            } else {
                ending_score += value;
            }
        }
    }

    (opening_score * position_phase) + (ending_score * (1.0 - position_phase))
}

/// Gets simplified sigmoid function.
fn sigmoid(e: f32, k: f32) -> f32 {
    1.0 / (1.0 + (-k * e).exp())
}

/// Loads positions from the `epd_filename` and parses them into a list of [TunerPosition]. Returns [Err] with a proper error message if the
/// file couldn't be parsed.
fn load_positions(
    epd_filename: &str,
    coefficients: &mut Vec<TunerCoefficient>,
    indices: &mut Vec<u16>,
    weights_indices: &mut HashSet<u16>,
) -> Result<Vec<TunerPosition>, String> {
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

        if parsed_epd.comment.is_none() {
            return Err("Game result not found".to_string());
        }

        let comment = parsed_epd.comment.unwrap();
        let comment_tokens = comment.split('|').collect::<Vec<&str>>();
        let evaluation = (comment_tokens[0].parse::<f32>().unwrap() * 100.0) as i16;
        let result = match comment_tokens[1] {
            "0-1" => 0,
            "1/2-1/2" => 1,
            "1-0" => 2,
            _ => return Err(format!("Invalid game result: comment_tokens[1]={}", comment)),
        };

        let mut index = 0;
        let base_index = coefficients.len();
        let mut dangered_white_king_squares = 0;
        let mut dangered_black_king_squares = 0;

        material::get_coefficients(&parsed_epd.board, &mut index, coefficients, indices);
        mobility::get_coefficients(&parsed_epd.board, &mut dangered_white_king_squares, &mut dangered_black_king_squares, &mut index, coefficients, indices);
        pawns::get_coefficients(&parsed_epd.board, &mut index, coefficients, indices);
        safety::get_coefficients(dangered_white_king_squares, dangered_black_king_squares, &mut index, coefficients, indices);
        pst::get_coefficients(&parsed_epd.board, PAWN, &mut index, coefficients, indices);
        pst::get_coefficients(&parsed_epd.board, KNIGHT, &mut index, coefficients, indices);
        pst::get_coefficients(&parsed_epd.board, BISHOP, &mut index, coefficients, indices);
        pst::get_coefficients(&parsed_epd.board, ROOK, &mut index, coefficients, indices);
        pst::get_coefficients(&parsed_epd.board, QUEEN, &mut index, coefficients, indices);
        pst::get_coefficients(&parsed_epd.board, KING, &mut index, coefficients, indices);

        for i in base_index..coefficients.len() {
            weights_indices.insert(indices[i]);
        }

        positions.push(TunerPosition::new(evaluation, result, parsed_epd.board.game_phase, base_index as u32, (coefficients.len() - base_index) as u8));
    }

    Ok(positions)
}

/// Transforms the current evaluation values into a list of [TunerParameter]. Use `lock_material` if the parameters related to piece values should
/// be skipped, and `random_values` if the parameters should have random values (useful when initializing tuner).
fn load_values(evaluation_parameters: &EvaluationParameters, random_values: bool) -> Vec<TunerParameter> {
    let mut parameters = vec![
        TunerParameter::new(evaluation_parameters.piece_value[PAWN], 100, 100, 100, 100),
        TunerParameter::new(evaluation_parameters.piece_value[KNIGHT], 0, 300, 400, 9999),
        TunerParameter::new(evaluation_parameters.piece_value[BISHOP], 0, 300, 400, 9999),
        TunerParameter::new(evaluation_parameters.piece_value[ROOK], 0, 400, 600, 9999),
        TunerParameter::new(evaluation_parameters.piece_value[QUEEN], 0, 900, 1200, 9999),
        TunerParameter::new(evaluation_parameters.piece_value[KING], 10000, 10000, 10000, 10000),
        TunerParameter::new(evaluation_parameters.bishop_pair_opening, -99, 10, 40, 99),
        TunerParameter::new(evaluation_parameters.bishop_pair_ending, -99, 10, 40, 99),
    ];

    parameters.append(&mut evaluation_parameters.mobility_inner_opening.iter().map(|v| TunerParameter::new(*v, 0, 2, 6, 99)).collect());
    parameters.append(&mut evaluation_parameters.mobility_inner_ending.iter().map(|v| TunerParameter::new(*v, 0, 2, 6, 99)).collect());

    parameters.append(&mut evaluation_parameters.mobility_outer_opening.iter().map(|v| TunerParameter::new(*v, 0, 2, 6, 99)).collect());
    parameters.append(&mut evaluation_parameters.mobility_outer_ending.iter().map(|v| TunerParameter::new(*v, 0, 2, 6, 99)).collect());

    parameters.append(&mut evaluation_parameters.doubled_pawn_opening.iter().map(|v| TunerParameter::new(*v, -999, -40, -10, 999)).collect());
    parameters.append(&mut evaluation_parameters.doubled_pawn_ending.iter().map(|v| TunerParameter::new(*v, -999, -40, -10, 999)).collect());

    parameters.append(&mut evaluation_parameters.isolated_pawn_opening.iter().map(|v| TunerParameter::new(*v, -999, -40, -10, 999)).collect());
    parameters.append(&mut evaluation_parameters.isolated_pawn_ending.iter().map(|v| TunerParameter::new(*v, -999, -40, -10, 999)).collect());

    parameters.append(&mut evaluation_parameters.chained_pawn_opening.iter().map(|v| TunerParameter::new(*v, -999, 10, 40, 999)).collect());
    parameters.append(&mut evaluation_parameters.chained_pawn_ending.iter().map(|v| TunerParameter::new(*v, -999, 10, 40, 999)).collect());

    parameters.append(&mut evaluation_parameters.passed_pawn_opening.iter().map(|v| TunerParameter::new(*v, -999, 10, 40, 999)).collect());
    parameters.append(&mut evaluation_parameters.passed_pawn_ending.iter().map(|v| TunerParameter::new(*v, -999, 10, 40, 999)).collect());

    parameters.append(&mut evaluation_parameters.pawn_shield_opening.iter().map(|v| TunerParameter::new(*v, -999, 10, 40, 999)).collect());
    parameters.append(&mut evaluation_parameters.pawn_shield_ending.iter().map(|v| TunerParameter::new(*v, -999, 10, 40, 999)).collect());

    parameters.append(&mut evaluation_parameters.pawn_shield_open_file_opening.iter().map(|v| TunerParameter::new(*v, -999, -40, -10, 999)).collect());
    parameters.append(&mut evaluation_parameters.pawn_shield_open_file_ending.iter().map(|v| TunerParameter::new(*v, -999, -40, -10, 999)).collect());

    parameters.append(&mut evaluation_parameters.king_attacked_squares_opening.iter().map(|v| TunerParameter::new(*v, -999, -40, 40, 999)).collect());
    parameters.append(&mut evaluation_parameters.king_attacked_squares_ending.iter().map(|v| TunerParameter::new(*v, -999, -40, 40, 999)).collect());

    let pawn_pst = &evaluation_parameters.pst_patterns[PAWN];
    for king_file in ALL_FILES {
        parameters.append(&mut pawn_pst[king_file][0].iter().map(|v| TunerParameter::new(*v, -999, -40, 40, 999)).collect());
        parameters.append(&mut pawn_pst[king_file][1].iter().map(|v| TunerParameter::new(*v, -999, -40, 40, 999)).collect());
    }

    for king_file in ALL_FILES {
        let knight_pst = &evaluation_parameters.pst_patterns[KNIGHT];
        parameters.append(&mut knight_pst[king_file][0].iter().map(|v| TunerParameter::new(*v, -999, -40, 40, 999)).collect());
        parameters.append(&mut knight_pst[king_file][1].iter().map(|v| TunerParameter::new(*v, -999, -40, 40, 999)).collect());
    }

    for king_file in ALL_FILES {
        let bishop_pst = &evaluation_parameters.pst_patterns[BISHOP];
        parameters.append(&mut bishop_pst[king_file][0].iter().map(|v| TunerParameter::new(*v, -999, -40, 40, 999)).collect());
        parameters.append(&mut bishop_pst[king_file][1].iter().map(|v| TunerParameter::new(*v, -999, -40, 40, 999)).collect());
    }

    for king_file in ALL_FILES {
        let rook_pst = &evaluation_parameters.pst_patterns[ROOK];
        parameters.append(&mut rook_pst[king_file][0].iter().map(|v| TunerParameter::new(*v, -999, -40, 40, 999)).collect());
        parameters.append(&mut rook_pst[king_file][1].iter().map(|v| TunerParameter::new(*v, -999, -40, 40, 999)).collect());
    }

    for king_file in ALL_FILES {
        let queen_pst = &evaluation_parameters.pst_patterns[QUEEN];
        parameters.append(&mut queen_pst[king_file][0].iter().map(|v| TunerParameter::new(*v, -999, -40, 40, 999)).collect());
        parameters.append(&mut queen_pst[king_file][1].iter().map(|v| TunerParameter::new(*v, -999, -40, 40, 999)).collect());
    }

    for king_file in ALL_FILES {
        let king_pst = &evaluation_parameters.pst_patterns[KING];
        parameters.append(&mut king_pst[king_file][0].iter().map(|v| TunerParameter::new(*v, -999, -40, 40, 999)).collect());
        parameters.append(&mut king_pst[king_file][1].iter().map(|v| TunerParameter::new(*v, -999, -40, 40, 999)).collect());
    }

    if random_values {
        rand::seed(common::time::get_unix_timestamp());
        for parameter in &mut parameters {
            parameter.value = rand::i16(parameter.min_init..=parameter.max_init);
        }
    }

    for parameter in &mut parameters {
        parameter.value = parameter.value.clamp(parameter.min, parameter.max);
    }

    parameters
}

/// Transforms `values` into the evaluation parameters, which can be used during real evaluation. Use `lock_material` if the parameters
/// related to piece values should be skipped.
fn save_values(evaluation_parameters: &mut EvaluationParameters, values: &mut [TunerParameter]) {
    let mut index = 0;

    save_values_to_i16_array_internal(values, &mut evaluation_parameters.piece_value, &mut index);

    save_values_internal(values, &mut evaluation_parameters.bishop_pair_opening, &mut index);
    save_values_internal(values, &mut evaluation_parameters.bishop_pair_ending, &mut index);

    save_values_to_i16_array_internal(values, &mut evaluation_parameters.mobility_inner_opening, &mut index);
    save_values_to_i16_array_internal(values, &mut evaluation_parameters.mobility_inner_ending, &mut index);
    save_values_to_i16_array_internal(values, &mut evaluation_parameters.mobility_outer_opening, &mut index);
    save_values_to_i16_array_internal(values, &mut evaluation_parameters.mobility_outer_ending, &mut index);

    save_values_to_i16_array_internal(values, &mut evaluation_parameters.doubled_pawn_opening, &mut index);
    save_values_to_i16_array_internal(values, &mut evaluation_parameters.doubled_pawn_ending, &mut index);

    save_values_to_i16_array_internal(values, &mut evaluation_parameters.isolated_pawn_opening, &mut index);
    save_values_to_i16_array_internal(values, &mut evaluation_parameters.isolated_pawn_ending, &mut index);

    save_values_to_i16_array_internal(values, &mut evaluation_parameters.chained_pawn_opening, &mut index);
    save_values_to_i16_array_internal(values, &mut evaluation_parameters.chained_pawn_ending, &mut index);

    save_values_to_i16_array_internal(values, &mut evaluation_parameters.passed_pawn_opening, &mut index);
    save_values_to_i16_array_internal(values, &mut evaluation_parameters.passed_pawn_ending, &mut index);

    save_values_to_i16_array_internal(values, &mut evaluation_parameters.pawn_shield_opening, &mut index);
    save_values_to_i16_array_internal(values, &mut evaluation_parameters.pawn_shield_ending, &mut index);

    save_values_to_i16_array_internal(values, &mut evaluation_parameters.pawn_shield_open_file_opening, &mut index);
    save_values_to_i16_array_internal(values, &mut evaluation_parameters.pawn_shield_open_file_ending, &mut index);

    save_values_to_i16_array_internal(values, &mut evaluation_parameters.king_attacked_squares_opening, &mut index);
    save_values_to_i16_array_internal(values, &mut evaluation_parameters.king_attacked_squares_ending, &mut index);

    let pawn_pst = &mut evaluation_parameters.pst_patterns[PAWN];
    for king_file in ALL_FILES {
        save_values_to_i8_array_internal(values, &mut pawn_pst[king_file][0], &mut index);
        save_values_to_i8_array_internal(values, &mut pawn_pst[king_file][1], &mut index);
    }

    for king_file in ALL_FILES {
        let knight_pst = &mut evaluation_parameters.pst_patterns[KNIGHT];
        save_values_to_i8_array_internal(values, &mut knight_pst[king_file][0], &mut index);
        save_values_to_i8_array_internal(values, &mut knight_pst[king_file][1], &mut index);
    }

    for king_file in ALL_FILES {
        let bishop_pst = &mut evaluation_parameters.pst_patterns[BISHOP];
        save_values_to_i8_array_internal(values, &mut bishop_pst[king_file][0], &mut index);
        save_values_to_i8_array_internal(values, &mut bishop_pst[king_file][1], &mut index);
    }

    for king_file in ALL_FILES {
        let rook_pst = &mut evaluation_parameters.pst_patterns[ROOK];
        save_values_to_i8_array_internal(values, &mut rook_pst[king_file][0], &mut index);
        save_values_to_i8_array_internal(values, &mut rook_pst[king_file][1], &mut index);
    }

    for king_file in ALL_FILES {
        let queen_pst = &mut evaluation_parameters.pst_patterns[QUEEN];
        save_values_to_i8_array_internal(values, &mut queen_pst[king_file][0], &mut index);
        save_values_to_i8_array_internal(values, &mut queen_pst[king_file][1], &mut index);
    }

    for king_file in ALL_FILES {
        let king_pst = &mut evaluation_parameters.pst_patterns[KING];
        save_values_to_i8_array_internal(values, &mut king_pst[king_file][0], &mut index);
        save_values_to_i8_array_internal(values, &mut king_pst[king_file][1], &mut index);
    }

    evaluation_parameters.recalculate();
}

/// Saves `index`-th evaluation parameter stored in `values` in the `destination`.
fn save_values_internal(values: &mut [TunerParameter], destination: &mut i16, index: &mut usize) {
    *destination = values[*index].value;
    *index += 1;
}

/// Saves [i8] array starting at the `index` of `values` in the `array`.
fn save_values_to_i8_array_internal(values: &mut [TunerParameter], array: &mut [i16], index: &mut usize) {
    array.copy_from_slice(&values[*index..(*index + array.len())].iter().map(|v| v.value).collect::<Vec<i16>>());
    *index += array.len();
}

/// Saves [i16] array starting at the `index` of `values` in the `array`.
fn save_values_to_i16_array_internal(values: &mut [TunerParameter], array: &mut [i16], index: &mut usize) {
    array.copy_from_slice(&values[*index..(*index + array.len())].iter().map(|v| v.value).collect::<Vec<i16>>());
    *index += array.len();
}

/// Generates `parameters.rs` file with current evaluation parameters, and saves it into the `output_directory`.
fn write_evaluation_parameters(evaluation_parameters: &EvaluationParameters, output_directory: &str, best_error: f32, k: f32, wdl_ratio: f32) {
    let mut output = String::new();

    output.push_str(get_header(best_error, k, wdl_ratio).as_str());
    output.push('\n');
    output.push_str("use super::*;\n");
    output.push('\n');
    output.push_str("pub const INITIAL_GAME_PHASE: u8 = 24;");
    output.push('\n');
    output.push_str("impl Default for EvaluationParameters {\n");
    output.push_str("    fn default() -> Self {\n");
    output.push_str("        let mut evaluation_parameters = Self {\n");
    output.push_str(get_array("piece_value", &evaluation_parameters.piece_value).as_str());
    output.push('\n');
    output.push_str(get_parameter("bishop_pair_opening", evaluation_parameters.bishop_pair_opening).as_str());
    output.push_str(get_parameter("bishop_pair_ending", evaluation_parameters.bishop_pair_ending).as_str());
    output.push('\n');
    output.push_str(get_array("mobility_inner_opening", &evaluation_parameters.mobility_inner_opening).as_str());
    output.push_str(get_array("mobility_inner_ending", &evaluation_parameters.mobility_inner_ending).as_str());
    output.push('\n');
    output.push_str(get_array("mobility_outer_opening", &evaluation_parameters.mobility_outer_opening).as_str());
    output.push_str(get_array("mobility_outer_ending", &evaluation_parameters.mobility_outer_ending).as_str());
    output.push('\n');
    output.push_str(get_array("doubled_pawn_opening", &evaluation_parameters.doubled_pawn_opening).as_str());
    output.push_str(get_array("doubled_pawn_ending", &evaluation_parameters.doubled_pawn_ending).as_str());
    output.push('\n');
    output.push_str(get_array("isolated_pawn_opening", &evaluation_parameters.isolated_pawn_opening).as_str());
    output.push_str(get_array("isolated_pawn_ending", &evaluation_parameters.isolated_pawn_ending).as_str());
    output.push('\n');
    output.push_str(get_array("chained_pawn_opening", &evaluation_parameters.chained_pawn_opening).as_str());
    output.push_str(get_array("chained_pawn_ending", &evaluation_parameters.chained_pawn_ending).as_str());
    output.push('\n');
    output.push_str(get_array("passed_pawn_opening", &evaluation_parameters.passed_pawn_opening).as_str());
    output.push_str(get_array("passed_pawn_ending", &evaluation_parameters.passed_pawn_ending).as_str());
    output.push('\n');
    output.push_str(get_array("pawn_shield_opening", &evaluation_parameters.pawn_shield_opening).as_str());
    output.push_str(get_array("pawn_shield_ending", &evaluation_parameters.pawn_shield_ending).as_str());
    output.push('\n');
    output.push_str(get_array("pawn_shield_open_file_opening", &evaluation_parameters.pawn_shield_open_file_opening).as_str());
    output.push_str(get_array("pawn_shield_open_file_ending", &evaluation_parameters.pawn_shield_open_file_ending).as_str());
    output.push('\n');
    output.push_str(get_array("king_attacked_squares_opening", &evaluation_parameters.king_attacked_squares_opening).as_str());
    output.push_str(get_array("king_attacked_squares_ending", &evaluation_parameters.king_attacked_squares_ending).as_str());
    output.push('\n');
    output.push_str("            pst: Box::new([[[[[0; 64]; 2]; 8]; 6]; 2]),\n");
    output.push_str("            pst_patterns: Box::new([[[[0; 64]; 2]; 8]; 6]),\n");
    output.push('\n');
    output.push_str(get_array("piece_phase_value", &evaluation_parameters.piece_phase_value).as_str());
    output.push_str(get_parameter("initial_game_phase", evaluation_parameters.initial_game_phase).as_str());
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
fn write_piece_square_table(output_directory: &str, best_error: f32, k: f32, wdl_ratio: f32, name: &str, patterns: &[[[i16; 64]; 2]; 8]) {
    let mut output = String::new();

    output.push_str(get_header(best_error, k, wdl_ratio).as_str());
    output.push('\n');
    output.push_str("use super::*;\n");
    output.push('\n');
    output.push_str("impl EvaluationParameters {\n");
    output.push_str("    #[rustfmt::skip]\n");
    output.push_str(&format!("    pub const {}_PST_PATTERN: [[[i16; 64]; 2]; 8] =\n", name));
    output.push_str("    [\n");

    for king_file in ALL_FILES {
        output.push_str("        [\n");
        output.push_str("            [\n");
        output.push_str(get_piece_square_table(&patterns[king_file][0]).as_str());
        output.push_str("            ],\n");
        output.push_str("            [\n");
        output.push_str(get_piece_square_table(&patterns[king_file][1]).as_str());
        output.push_str("            ],\n");
        output.push_str("        ],\n");
    }

    output.push_str("    ];\n");
    output.push_str("}\n");

    let path = Path::new(output_directory).join("pst");
    fs::create_dir_all(path).unwrap();

    let path = Path::new(output_directory).join("pst").join(format!("{}.rs", name.to_lowercase()));
    write!(&mut File::create(path).unwrap(), "{}", output).unwrap();
}

/// Gets a generated Rust source file header with timestamp, `best_error`, `k` and `wdl_ratio`.
fn get_header(best_error: f32, k: f32, wdl_ratio: f32) -> String {
    let mut output = String::new();
    let datetime = DateTime::now();
    let datetime = format!("{:0>2}-{:0>2}-{} {:0>2}:{:0>2}:{:0>2}", datetime.day, datetime.month, datetime.year, datetime.hour, datetime.minute, datetime.day);

    output.push_str("// ------------------------------------------------------------------------- //\n");
    output.push_str(format!("// Generated at {} UTC (e = {:.6}, k = {:.4}, r = {:.2}) //\n", datetime, best_error, k, wdl_ratio).as_str());
    output.push_str("// ------------------------------------------------------------------------- //\n");
    output
}

/// Gets a Rust representation of the piece `values` array.
fn get_array<T>(name: &str, values: &[T]) -> String
where
    T: Display,
{
    format!("            {}: [{}],\n", name, values.iter().map(|p| p.to_string()).collect::<Vec<String>>().join(", "))
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
    for index in ALL_SQUARES {
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

/// Tests the correctness of [load_values] and [save_values] methods.
pub fn validate() -> bool {
    /* let mut context = TunerContext::new(Vec::new());
    let mut values = load_values(&context, false);
    save_values(&mut context, &mut values);

    let values_after_save = load_values(&context, false);
    values.iter().zip(&values_after_save).all(|(a, b)| a.value == b.value)
    */

    true
}
