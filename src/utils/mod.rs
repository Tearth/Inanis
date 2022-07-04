pub mod benchmark;
pub mod fen;
pub mod pgn;
pub mod test;
pub mod tuner;

macro_rules! conditional_expression {
    ($condition: expr, $expression: expr) => {
        if $condition {
            $expression;
        }
    };
}

pub(crate) use conditional_expression;
