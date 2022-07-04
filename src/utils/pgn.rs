use crate::engine::see::SEEContainer;
use crate::evaluation::EvaluationParameters;
use crate::state::board::Bitboard;
use crate::state::movegen::MagicContainer;
use crate::state::movescan::Move;
use crate::state::patterns::PatternsContainer;
use crate::state::zobrist::ZobristContainer;
use std::fs::File;
use std::io::BufReader;
use std::io::Lines;
use std::sync::Arc;

pub struct PGNLoader {
    pub file_iterator: Lines<BufReader<File>>,
    pub evaluation_parameters: Arc<EvaluationParameters>,
    pub zobrist_container: Arc<ZobristContainer>,
    pub patterns_container: Arc<PatternsContainer>,
    pub see_container: Arc<SEEContainer>,
    pub magic_container: Arc<MagicContainer>,
}

pub struct ParsedPGN {
    pub result: String,
    pub white_elo: u32,
    pub black_elo: u32,
    pub moves: Vec<Move>,
}

impl PGNLoader {
    pub fn new(file_iterator: Lines<BufReader<File>>) -> PGNLoader {
        let evaluation_parameters = Arc::new(EvaluationParameters::default());
        let zobrist_container = Arc::new(ZobristContainer::default());
        let patterns_container = Arc::new(PatternsContainer::default());
        let see_container = Arc::new(SEEContainer::new(Some(evaluation_parameters.clone())));
        let magic_container = Arc::new(MagicContainer::default());

        PGNLoader {
            file_iterator,
            evaluation_parameters,
            zobrist_container,
            patterns_container,
            see_container,
            magic_container,
        }
    }

    fn parse(&self, pgn: String) -> Result<ParsedPGN, &'static str> {
        let mut result = None;
        let mut white_elo = None;
        let mut black_elo = None;
        let mut moves = Vec::new();

        for line in pgn.lines() {
            if line.is_empty() {
                continue;
            } else if line.starts_with('[') {
                let name_start_index = match line.find(char::is_alphabetic) {
                    Some(value) => value,
                    None => return Err("Invalid property"),
                };

                let name_end_index = match line[name_start_index..].find(' ') {
                    Some(value) => name_start_index + value,
                    None => return Err("Invalid property"),
                };

                let value_start_index = match line.find('\"') {
                    Some(value) => value + 1,
                    None => return Err("Invalid property"),
                };

                let value_end_index = match line[value_start_index..].find('\"') {
                    Some(value) => value_start_index + value,
                    None => return Err("Invalid property"),
                };

                let name = line[name_start_index..name_end_index].to_string();
                let value = line[value_start_index..value_end_index].to_string();

                match name.as_str() {
                    "Result" => {
                        result = Some(value);
                    }
                    "WhiteElo" => {
                        white_elo = Some(value);
                    }
                    "BlackElo" => {
                        black_elo = Some(value);
                    }
                    _ => {}
                };
            } else if line.starts_with('1') {
                let mut board = Bitboard::new_initial_position(
                    Some(self.evaluation_parameters.clone()),
                    Some(self.zobrist_container.clone()),
                    Some(self.patterns_container.clone()),
                    Some(self.see_container.clone()),
                    Some(self.magic_container.clone()),
                );
                for token in line.split_ascii_whitespace() {
                    if token.as_bytes()[0].is_ascii_digit() {
                        continue;
                    }

                    let r#move = match Move::from_short_notation(token, &mut board) {
                        Ok(r#move) => r#move,
                        Err(_) => return Err("Invalid move"),
                    };

                    moves.push(r#move);
                    board.make_move(r#move);
                }
            }
        }

        let result = match result {
            Some(value) => value,
            None => return Err("No Result property"),
        };

        let white_elo = match white_elo {
            Some(value) => match value.parse::<u32>() {
                Ok(value) => value,
                Err(_) => return Err("Invalid WhiteElo property"),
            },
            None => return Err("No WhiteElo property"),
        };

        let black_elo = match black_elo {
            Some(value) => match value.parse::<u32>() {
                Ok(value) => value,
                Err(_) => return Err("Invalid BlackElo property"),
            },
            None => return Err("No BlackElo property"),
        };

        Ok(ParsedPGN::new(result, white_elo, black_elo, moves))
    }
}

impl Iterator for PGNLoader {
    type Item = Result<ParsedPGN, &'static str>;

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
    pub fn new(result: String, white_elo: u32, black_elo: u32, moves: Vec<Move>) -> ParsedPGN {
        ParsedPGN {
            result,
            white_elo,
            black_elo,
            moves,
        }
    }
}
