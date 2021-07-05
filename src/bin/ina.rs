use chrono::*;
use ina::movegen::*;
use ina::patterns::*;

fn main() {
    let now = Utc::now();

    fastrand::seed(9000);
    patterns_init();
    magic_init();

    let diff = Utc::now() - now;
    println!("");
    println!("Time: {} ms", diff.num_milliseconds());
    println!("");

    let _test1 = patterns_get_file(10);
    let _test2 = patterns_get_rank(20);
    let _test3 = patterns_get_diagonals(3);
    let _test4 = patterns_get_jumps(4);
    let _test5 = patterns_get_box(5);

    println!("WAH");

    loop {}
}
