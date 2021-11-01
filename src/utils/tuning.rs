use crate::evaluation::parameters::*;
use crate::evaluation::pst::bishop;
use crate::evaluation::pst::king;
use crate::evaluation::pst::knight;
use crate::evaluation::pst::pawn;
use crate::evaluation::pst::queen;
use crate::evaluation::pst::rook;
use chrono::Utc;
use std::fs;
use std::fs::File;
use std::io::Write;

pub fn run() {
    save_evaluation_parameters();
    save_piece_square_table("pawn", &pawn::PATTERN[0], &pawn::PATTERN[1]);
    save_piece_square_table("knight", &knight::PATTERN[0], &knight::PATTERN[1]);
    save_piece_square_table("bishop", &bishop::PATTERN[0], &bishop::PATTERN[1]);
    save_piece_square_table("rook", &rook::PATTERN[0], &rook::PATTERN[1]);
    save_piece_square_table("queen", &queen::PATTERN[0], &queen::PATTERN[1]);
    save_piece_square_table("king", &king::PATTERN[0], &king::PATTERN[1]);
}

fn save_evaluation_parameters() {
    let mut output = String::new();
    save_header(&mut output);
    output.push_str("\n");
    output.push_str(
        format!(
            "pub static PIECE_VALUE: [i16; 6] = [{}, {}, {}, {}, {}, {}];\n",
            PIECE_VALUE[0], PIECE_VALUE[1], PIECE_VALUE[2], PIECE_VALUE[3], PIECE_VALUE[4], PIECE_VALUE[5]
        )
        .as_str(),
    );
    output.push_str("\n");
    output.push_str(format!("pub static MOBILITY_OPENING: i16 = {};\n", MOBILITY_OPENING).as_str());
    output.push_str(format!("pub static MOBILITY_ENDING: i16 = {};\n", MOBILITY_ENDING).as_str());
    output.push_str("\n");
    output.push_str(format!("pub static DOUBLED_PAWN_OPENING: i16 = {};\n", DOUBLED_PAWN_OPENING).as_str());
    output.push_str(format!("pub static DOUBLED_PAWN_ENDING: i16 = {};\n", DOUBLED_PAWN_ENDING).as_str());
    output.push_str("\n");
    output.push_str(format!("pub static ISOLATED_PAWN_OPENING: i16 = {};\n", ISOLATED_PAWN_OPENING).as_str());
    output.push_str(format!("pub static ISOLATED_PAWN_ENDING: i16 = {};\n", ISOLATED_PAWN_ENDING).as_str());
    output.push_str("\n");
    output.push_str(format!("pub static CHAINED_PAWN_OPENING: i16 = {};\n", CHAINED_PAWN_OPENING).as_str());
    output.push_str(format!("pub static CHAINED_PAWN_ENDING: i16 = {};\n", CHAINED_PAWN_ENDING).as_str());
    output.push_str("\n");
    output.push_str(format!("pub static PASSING_PAWN_OPENING: i16 = {};\n", PASSING_PAWN_OPENING).as_str());
    output.push_str(format!("pub static PASSING_PAWN_ENDING: i16 = {};\n", PASSING_PAWN_ENDING).as_str());
    output.push_str("\n");
    output.push_str(format!("pub static PAWN_SHIELD_OPENING: i16 = {};\n", PAWN_SHIELD_OPENING).as_str());
    output.push_str(format!("pub static PAWN_SHIELD_ENDING: i16 = {};\n", PAWN_SHIELD_ENDING).as_str());
    output.push_str("\n");
    output.push_str(format!("pub static PAWN_SHIELD_OPEN_FILE_OPENING: i16 = {};\n", PAWN_SHIELD_OPEN_FILE_OPENING).as_str());
    output.push_str(format!("pub static PAWN_SHIELD_OPEN_FILE_ENDING: i16 = {};\n", PAWN_SHIELD_OPEN_FILE_ENDING).as_str());
    output.push_str("\n");
    output.push_str(format!("pub static KING_ATTACKED_FIELDS_OPENING: i16 = {};\n", KING_ATTACKED_FIELDS_OPENING).as_str());
    output.push_str(format!("pub static KING_ATTACKED_FIELDS_ENDING: i16 = {};\n", KING_ATTACKED_FIELDS_ENDING).as_str());
    output.push_str("\n");

    fs::create_dir_all("output/").unwrap();
    write!(&mut File::create("output/parameters.rs").unwrap(), "{}", output.to_string()).unwrap();
}

fn save_piece_square_table(name: &str, opening: &[i8], ending: &[i8]) {
    let mut output = String::new();
    save_header(&mut output);
    output.push_str("\n");
    output.push_str("#[rustfmt::skip]\n");
    output.push_str("pub static PATTERN: [[i8; 64]; 2] =\n");
    output.push_str("[\n");
    output.push_str("    [\n");
    save_piece_square_table_internal(&mut output, opening);
    output.push_str("    ],\n");
    output.push_str("    [\n");
    save_piece_square_table_internal(&mut output, ending);
    output.push_str("    ],\n");
    output.push_str("];\n");

    fs::create_dir_all("output/pst/").unwrap();
    write!(&mut File::create(format!("output/pst/{}.rs", name)).unwrap(), "{}", output.to_string()).unwrap();
}

fn save_piece_square_table_internal(output: &mut String, values: &[i8]) {
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
}

fn save_header(output: &mut String) {
    output.push_str("// ------------------------------------ //\n");
    output.push_str(format!("// Generated at {} UTC //\n", Utc::now().format("%Y-%m-%d %H:%M:%S")).as_str());
    output.push_str("// ------------------------------------ //\n");
}
