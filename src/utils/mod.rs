pub mod benchmark;
pub mod bitflags;
pub mod rand;
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
