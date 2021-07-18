use ina::movegen;
use ina::patterns;
use ina::terminal;

fn main() {
    patterns::init();
    movegen::init();

    terminal::init();
    terminal::run();
}
