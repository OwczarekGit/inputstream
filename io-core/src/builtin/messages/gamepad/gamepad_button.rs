#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum GamepadButton {
    Triangle = 0,
    Circle,
    X,
    Square,

    Up,
    Right,
    Down,
    Left,

    LeftBumper,
    RightBumper,

    Share,
    Options,

    LeftStick,
    RightStick,

    Logo,
    Mute,
}

impl GamepadButton {
    pub fn bit(&self) -> u32 {
        *self as u32
    }
}
