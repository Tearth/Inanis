use inanis::{interface::terminal, state::movegen};
use std::env;

/// Entry point of the Inanis engine.
pub fn main() {
    let args = env::args_os().collect();
    let target_features = get_target_features();

    movegen::init();

    terminal::run(args, target_features);
}

/// Gets a list of target features (POPCNT, BMI1, BMI2) with which the executable was built.
fn get_target_features() -> Vec<&'static str> {
    let mut target_features = Vec::new();

    if cfg!(target_feature = "popcnt") {
        target_features.push("POPCNT");
    }

    if cfg!(target_feature = "bmi1") {
        target_features.push("BMI1");
    }

    if cfg!(target_feature = "bmi2") {
        target_features.push("BMI2");
    }

    target_features
}
