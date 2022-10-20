use crate::state::movescan::Move;

pub mod syzygy;

#[derive(PartialEq, Eq, Debug)]
pub enum WdlResult {
    Win,
    Draw,
    Loss,
}

pub struct WdlDtzResult {
    pub wdl: WdlResult,
    pub dtz: u32,
    pub r#move: Move,
}

impl WdlDtzResult {
    /// Constructs a new instance of [WdlDtzResult] with stored `wdl`, `dtz` and `r#move`.
    pub fn new(wdl: WdlResult, dtz: u32, r#move: Move) -> Self {
        WdlDtzResult { wdl, dtz, r#move }
    }
}
