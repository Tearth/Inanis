use ina::{
    board::{movegen, patterns, zobrist},
    frontend::terminal,
};

fn main() {
    fastrand::seed(584578);

    patterns::init();
    movegen::init();
    zobrist::init();
    terminal::run();
}
