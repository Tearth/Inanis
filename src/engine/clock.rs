use super::context::SearchContext;
use crate::utils::param;

/// Calculates time bounds (soft and hard) which should be used for the next move. Formula and plot for the case when `moves_to_go` is zeroed can
/// be found in the `/misc/time.xlsx` Excel sheet, but in general outline it tries to allocate more time during midgame where usually there's
/// a lot of pieces on the board and it's crucial to find some advantage at this phase, that can be converted later to decisive result. Formula used
/// when `moves_to_go` is greater than zero is simpler and allocates time evenly.
pub fn get_time_bounds(context: &SearchContext) -> (u32, u32) {
    if context.max_move_time != 0 {
        let soft_bound = context.max_move_time * param!(context.params.time_soft_bound) as u32 / 100;
        let hard_bound = context.max_move_time;

        return (soft_bound, hard_bound);
    }

    let allocated_time = if context.moves_to_go == 0 {
        let a = param!(context.params.time_a) as f32;
        let b = param!(context.params.time_b) as f32;
        let c = param!(context.params.time_c) as f32;
        let d = param!(context.params.time_d) as f32;
        let e = param!(context.params.time_e) as f32;

        context.time / ((a + b * f32::sin(f32::min(c, (context.board.fullmove_number as f32) + d) / e)) as u32) + context.inc_time
    } else {
        context.time / (context.moves_to_go + 2) + context.inc_time
    };

    let soft_bound = allocated_time * param!(context.params.time_soft_bound) as u32 / 100;
    let hard_bound = allocated_time * param!(context.params.time_hard_bound) as u32 / 100;

    (soft_bound.min(context.time), hard_bound.min(context.time))
}
