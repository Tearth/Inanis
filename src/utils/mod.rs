pub mod benchmark;
pub mod bitflags;
pub mod fen;
pub mod pgn;
pub mod test;
pub mod tuner;
pub mod tunerset;

macro_rules! conditional_expression {
    ($condition: expr, $expression: expr) => {
        if $condition {
            $expression;
        }
    };
}

pub(crate) use conditional_expression;
