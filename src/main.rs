use crate::patterns::*;

mod constants;
mod movegen;
mod patterns;

fn main() {
    patterns_init();

    let _test1 = patterns_get_file(1);
    let _test2 = patterns_get_rank(2);
    let _test3 = patterns_get_jump(3);
    let _test4 = patterns_get_box(4);

    println!("WAH");
}
