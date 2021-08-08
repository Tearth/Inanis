use ina::board::movegen;
use ina::board::patterns;
use ina::board::zobrist;
use ina::interface::terminal;

fn main() {
    fastrand::seed(584578);

    patterns::init();
    movegen::init();
    zobrist::init();
    terminal::run();
}
