pub trait BitFlags {
    fn contains(&self, value: u8) -> bool;
}

impl BitFlags for u8 {
    fn contains(&self, value: u8) -> bool {
        (self & value) != 0
    }
}
