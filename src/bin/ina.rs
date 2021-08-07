use ina::movegen;
use ina::patterns;
use ina::terminal;
use ina::zobrist;

fn main() {
    fastrand::seed(584578);

    patterns::init();
    movegen::init();
    zobrist::init();
    terminal::run();
}
