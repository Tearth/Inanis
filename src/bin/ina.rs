use ina::{movegen, patterns, terminal};

fn main() {
    patterns::init();
    movegen::init();

    terminal::init();
    terminal::run();
}
