#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum Button {
    Left = 0,
    Right,
    Middle,
    Mouse4,
    Mouse5,
}

impl Button {
    pub fn bit(&self) -> u8 {
        *self as u8
    }
}
