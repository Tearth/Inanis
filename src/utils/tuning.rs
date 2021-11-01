use crate::evaluation::parameters::*;
use crate::evaluation::pst::bishop;
use crate::evaluation::pst::king;
use crate::evaluation::pst::knight;
use crate::evaluation::pst::pawn;
use crate::evaluation::pst::queen;
use crate::evaluation::pst::rook;
use chrono::Utc;
use nameof::name_of;
use std::fs;
use std::fs::File;
use std::io::Write;

pub fn run() {
    save_evaluation_parameters();
    save_piece_square_table("pawn", unsafe { &pawn::PATTERN[0] }, unsafe { &pawn::PATTERN[1] });
    save_piece_square_table("knight", unsafe { &knight::PATTERN[0] }, unsafe { &knight::PATTERN[1] });
    save_piece_square_table("bishop", unsafe { &bishop::PATTERN[0] }, unsafe { &bishop::PATTERN[1] });
    save_piece_square_table("rook", unsafe { &rook::PATTERN[0] }, unsafe { &rook::PATTERN[1] });
    save_piece_square_table("queen", unsafe { &queen::PATTERN[0] }, unsafe { &queen::PATTERN[1] });
    save_piece_square_table("king", unsafe { &king::PATTERN[0] }, unsafe { &king::PATTERN[1] });
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

fn get_header() -> String {
    let mut output = String::new();

    output.push_str("// ------------------------------------ //\n");
    output.push_str(format!("// Generated at {} UTC //\n", Utc::now().format("%Y-%m-%d %H:%M:%S")).as_str());
    output.push_str("// ------------------------------------ //\n");
    output
}
