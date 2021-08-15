use ina::state::movegen;
use ina::state::patterns;
use ina::state::zobrist;
use ina::interface::terminal;

fn main() {
    fastrand::seed(584578);

    patterns::init();
    movegen::init();
    zobrist::init();
    terminal::run();
}
