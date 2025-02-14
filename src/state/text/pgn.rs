use super::*;
use crate::state::movescan::Move;
use crate::state::representation::Board;
use std::fs::File;
use std::io::BufReader;
use std::io::Lines;

pub struct PGNLoader {
    pub file_iterator: Lines<BufReader<File>>,
}

pub struct ParsedPGN {
    pub result: String,
    pub fen: Option<String>,
    pub data: Vec<ParsedPGNMove>,
}

pub struct ParsedPGNMove {
    pub r#move: Move,
    pub eval: f32,
}

impl PGNLoader {
    /// Constructs a new instance of [PGNLoader] with the specified `file_iterator`, which will be used to read input PGN file.
    pub fn new(file_iterator: Lines<BufReader<File>>) -> PGNLoader {
        PGNLoader { file_iterator }
    }

    /// Parses a single `pgn` and returns [Some] if it has been done with success, otherwise [Err].
    fn parse(&self, pgn: String) -> Result<ParsedPGN, String> {
        let mut result = None;
        let mut fen = None;
        let mut moves = Vec::new();

        for line in pgn.lines() {
            if line.is_empty() {
                continue;
            } else if line.starts_with('[') {
                let name_start_index = match line.find(char::is_alphabetic) {
                    Some(value) => value,
                    None => return Err(format!("Invalid property: line={}", line)),
                };

                let name_end_index = match line[name_start_index..].find(' ') {
                    Some(value) => name_start_index + value,
                    None => return Err(format!("Invalid property: line={}", line)),
                };

                let value_start_index = match line.find('\"') {
                    Some(value) => value + 1,
                    None => return Err(format!("Invalid property: line={}", line)),
                };

                let value_end_index = match line[value_start_index..].find('\"') {
                    Some(value) => value_start_index + value,
                    None => return Err(format!("Invalid property: line={}", line)),
                };

                let name = line[name_start_index..name_end_index].to_string();
                let value = line[value_start_index..value_end_index].to_string();

                match name.as_str() {
                    "Result" => result = Some(value),
                    "FEN" => fen = Some(value),
                    _ => {}
                }
            } else if line.starts_with('1') {
                let mut board = match fen.clone() {
                    Some(value) => {
                        let fen_result = Board::new_from_fen(&value);

                        match fen_result {
                            Ok(board) => board,
                            Err(error) => return Err(format!("Invalid initial FEN position: {}", error)),
                        }
                    }
                    None => Board::new_initial_position(),
                };

                let mut comment = false;
                let mut pgn_move = None;
                let mut pgn_eval: Option<&str> = None;

                for token in line.split_ascii_whitespace() {
                    if token.ends_with('}') {
                        comment = false;
                        continue;
                    }

                    if token.as_bytes()[0].is_ascii_digit() {
                        continue;
                    }

                    if let Some(r#move) = pgn_move {
                        if let Some(eval) = pgn_eval {
                            let mut eval = if eval.starts_with("+M") {
                                100.0
                            } else if eval.starts_with("-M") {
                                -100.0
                            } else {
                                eval.parse::<f32>().unwrap()
                            };

                            if board.stm == BLACK {
                                eval = -eval;
                            }

                            moves.push(ParsedPGNMove::new(r#move, eval));
                            board.make_move(r#move);

                            pgn_move = None;
                            pgn_eval = None;
                        }
                    }

                    if let Some(token) = token.strip_prefix('{') {
                        pgn_eval = Some(token.split('/').collect::<Vec<&str>>()[0]);
                        comment = true;
                        continue;
                    }

                    if comment {
                        continue;
                    }

                    if token == "*" || token == "h1h1" {
                        break;
                    }

                    pgn_move = match Move::from_short_notation(token, &mut board) {
                        Ok(r#move) => Some(r#move),
                        Err(error) => return Err(format!("Invalid move: {}", error)),
                    };
                }
            }
        }

        let result = match result {
            Some(value) => value,
            None => return Err("No Result property".to_string()),
        };

        Ok(ParsedPGN::new(result, fen, moves))
    }
}

impl Iterator for PGNLoader {
    type Item = Result<ParsedPGN, String>;

    /// Performs the next iteration by parsing the following PGN from the input file. If there are none left, returns [None].
    fn next(&mut self) -> Option<Self::Item> {
        let mut pgn = String::new();

        while let Some(Ok(line)) = self.file_iterator.next() {
            if line.starts_with("[Event") && !pgn.is_empty() {
                break;
            }

            let trimmed_line = line.trim();
            if trimmed_line.is_empty() {
                continue;
            }

            pgn.push_str(trimmed_line);

            if line.starts_with('[') {
                pgn.push('\n');
            } else {
                pgn.push(' ');
            }
        }

        if !pgn.is_empty() {
            return Some(self.parse(pgn));
        }

        None
    }
}

impl ParsedPGN {
    /// Constructs a new instance of [ParsedPGN] with stored `result`, `fen` and `moves`.
    pub fn new(result: String, fen: Option<String>, moves: Vec<ParsedPGNMove>) -> ParsedPGN {
        ParsedPGN { result, fen, data: moves }
    }
}

impl ParsedPGNMove {
    pub fn new(r#move: Move, eval: f32) -> Self {
        Self { r#move, eval }
    }
}
