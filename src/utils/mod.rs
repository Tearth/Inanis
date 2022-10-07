pub mod bitflags;
pub mod bithelpers;
pub mod divceil;
pub mod rand;

macro_rules! conditional_expression {
    ($condition: expr, $expression: expr) => {
        if $condition {
            $expression;
        }
    };
}

pub(crate) use conditional_expression;
