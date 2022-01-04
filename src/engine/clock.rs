pub fn get_time_for_move(total_time: u32, inc_time: u32, moves_to_go: u32) -> u32 {
    if moves_to_go == 0 {
        total_time / 30 + inc_time
    } else {
        total_time / (moves_to_go + 2) + inc_time
    }
}
