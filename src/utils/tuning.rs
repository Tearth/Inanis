use std::{
    fs::{self, File},
    io::Write,
};

use chrono::Utc;

pub fn run() {
    safe_piece_square_table(
        "pawn",
        &crate::evaluation::pst::pawn::PATTERN[0],
        &crate::evaluation::pst::pawn::PATTERN[1],
    );

    safe_piece_square_table(
        "knight",
        &crate::evaluation::pst::knight::PATTERN[0],
        &crate::evaluation::pst::knight::PATTERN[1],
    );

    safe_piece_square_table(
        "bishop",
        &crate::evaluation::pst::bishop::PATTERN[0],
        &crate::evaluation::pst::bishop::PATTERN[1],
    );

    safe_piece_square_table(
        "rook",
        &crate::evaluation::pst::rook::PATTERN[0],
        &crate::evaluation::pst::rook::PATTERN[1],
    );

    safe_piece_square_table(
        "queen",
        &crate::evaluation::pst::queen::PATTERN[0],
        &crate::evaluation::pst::queen::PATTERN[1],
    );

    safe_piece_square_table(
        "king",
        &crate::evaluation::pst::king::PATTERN[0],
        &crate::evaluation::pst::king::PATTERN[1],
    );
}

fn safe_piece_square_table(name: &str, opening: &[i8], ending: &[i8]) {
    let mut output = String::new();
    output.push_str("// ------------------------------------ //\n");
    output.push_str(format!("// Generated at {} UTC //\n", Utc::now().format("%Y-%m-%d %H:%M:%S")).as_str());
    output.push_str("// ------------------------------------ //\n");
    output.push_str("\n");
    output.push_str("#[rustfmt::skip]\n");
    output.push_str("pub static PATTERN: [[i8; 64]; 2] =\n");
    output.push_str("[\n");
    output.push_str("    [\n");
    safe_piece_square_table_internal(&mut output, opening);
    output.push_str("    ],\n");
    output.push_str("    [\n");
    safe_piece_square_table_internal(&mut output, ending);
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

fn safe_piece_square_table_internal(output: &mut String, values: &[i8]) {
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
