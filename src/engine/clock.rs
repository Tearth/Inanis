pub fn get_time_for_move(move_number: u16, total_time: u32, inc_time: u32, moves_to_go: u32) -> u32 {
    if moves_to_go == 0 {
        const A: f32 = 45.0;
        const B: f32 = -25.0;
        const C: f32 = 25.0;
        const D: f32 = -10.0;
        const E: f32 = 15.0;

        total_time / ((A + B * (C.min((move_number as f32) + D) / E).sin()) as u32) + inc_time
    } else {
        total_time / (moves_to_go + 1) + inc_time
    }
}
