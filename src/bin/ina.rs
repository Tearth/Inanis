use ina::evaluation::pst;
use ina::interface::terminal;
use ina::state::movegen;
use ina::state::patterns;
use ina::state::zobrist;

fn main() {
    fastrand::seed(584578);

    pst::init();
    patterns::init();
    movegen::init();
    zobrist::init();
    terminal::run();
}
