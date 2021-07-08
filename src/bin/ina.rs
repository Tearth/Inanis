use chrono::*;

fn main() {
    let now = Utc::now();

    ina::patterns::init();
    ina::movegen::init();

    let diff = Utc::now() - now;
    println!("");
    println!("Time: {} ms", diff.num_milliseconds());
    println!("");

    let _test1 = ina::patterns::get_file(10);
    let _test2 = ina::patterns::get_rank(20);
    let _test3 = ina::patterns::get_diagonals(3);
    let _test4 = ina::patterns::get_jumps(4);
    let _test5 = ina::patterns::get_box(5);

    println!("WAH");

    loop {}
}
