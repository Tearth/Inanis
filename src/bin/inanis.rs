use inanis::interface::terminal;

/// Entry point of the Inanis engine, initializes all subsystems and runs the terminal.
pub fn main() {
    fastrand::seed(584578);
    terminal::run();
}
