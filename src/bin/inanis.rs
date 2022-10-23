use inanis::interface::terminal;

/// Entry point of the Inanis engine.
pub fn main() {
    terminal::run(get_target_features());
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
