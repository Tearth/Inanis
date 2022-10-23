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

macro_rules! percent {
    ($from: expr, $all: expr) => {
        (($from as f32) / ($all as f32)) * 100.0
    };
}

pub(crate) use conditional_expression;
pub(crate) use percent;
