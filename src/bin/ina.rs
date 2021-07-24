use ina::movegen;
use ina::patterns;
use ina::terminal;
use ina::zobrist;

fn main() {
    patterns::init();
    movegen::init();
    zobrist::init();

    terminal::init();
    terminal::run();
}
