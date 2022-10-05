pub mod benchmark;
pub mod bitflags;
pub mod rand;
pub mod test;

macro_rules! conditional_expression {
    ($condition: expr, $expression: expr) => {
        if $condition {
            $expression;
        }
    };
}

pub(crate) use conditional_expression;
