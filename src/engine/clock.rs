use super::context::SearchContext;
use crate::utils::param;

/// Calculates a time which should be allocated for the next move, based on `move_number`, `total_time`, `inc_time` (0 if not available)
/// and `moves_to_go` (0 if not available). Formula and chart used when `moves_to_go` is zeroed can be found in the `/misc/time.xlsx` Excel sheet,
/// but in general outline it tries to allocate more time during mid-game where usually there's a lot of pieces on the board and it's crucial
/// to find some advantage at this phase. Formula used when `moves_to_go` is greater than zero is simpler and allocates time evenly.
pub fn get_time_for_move(context: &SearchContext, move_number: u16, total_time: u32, inc_time: u32, moves_to_go: u32) -> u32 {
    if moves_to_go == 0 {
        let a = param!(context.params.time_a) as f32;
        let b = param!(context.params.time_b) as f32;
        let c = param!(context.params.time_c) as f32;
        let d = param!(context.params.time_d) as f32;
        let e = param!(context.params.time_e) as f32;

        total_time / ((a + b * f32::sin(f32::min(c, (move_number as f32) + d) / e)) as u32) + inc_time
    } else {
        total_time / (moves_to_go + 2) + inc_time
    }
}
