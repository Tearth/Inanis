pub mod bitflags;
pub mod bithelpers;
pub mod divceil;
pub mod minmax;
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

macro_rules! parameter {
    ($a : ident . $b : ident . $c : ident) => {
        if cfg!(feature = "dev") {
            $a.$b.$c
        } else {
            crate::engine::parameters::SearchParameters::$c
        }
    };
}

pub(crate) use conditional_expression;
pub(crate) use parameter;
pub(crate) use percent;
