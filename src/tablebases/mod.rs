pub mod syzygy;

#[derive(PartialEq, Eq, Debug)]
pub enum WdlResult {
    Win,
    Draw,
    Loss,
}
