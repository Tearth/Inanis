use inanis::interface::terminal;

/// Entry point of the Inanis engine, initializes all subsystems and runs the terminal.
pub fn main() {
    inanis::init();
    terminal::run();
}
