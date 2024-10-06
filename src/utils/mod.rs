pub mod bitflags;
pub mod bithelpers;
pub mod divceil;
pub mod minmax;
pub mod rand;

macro_rules! dev {
    ($expression: expr) => {
        if cfg!(feature = "dev") {
            $expression;
        }
    };
}

macro_rules! percent {
    ($from: expr, $all: expr) => {
        (($from as f32) / ($all as f32)) * 100.0
    };
}

macro_rules! param {
    ($a : ident . $b : ident . $c : ident) => {
        if cfg!(feature = "dev") {
            $a.$b.$c
        } else {
            crate::engine::params::SParams::$c
        }
    };
}

macro_rules! panic_fast {
    ($fmt:expr) => ({
        if cfg!(feature = "dev") {
            panic!(concat!($fmt, "\n"));
        } else {
            std::process::abort();
        }
    });
    ($fmt:expr, $($arg:tt)*) => (
    {
        if cfg!(feature = "dev") {
            panic!(concat!($fmt, "\n"), $($arg)*);
        } else {
            std::process::abort();
        }
    });
}

macro_rules! assert_fast {
    ($($arg:tt)*) => {
        if cfg!(debug_assertions) {
            debug_assert!($($arg)*);
        } else {
            if !($($arg)*) {
                unsafe { std::hint::unreachable_unchecked() };
            }
        }
    };
}

pub(crate) use assert_fast;
pub(crate) use dev;
pub(crate) use panic_fast;
pub(crate) use param;
pub(crate) use percent;
