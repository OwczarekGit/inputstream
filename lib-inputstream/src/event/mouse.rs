use std::fmt::Display;

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct MouseEvent {
    pub dx: f32,
    pub dy: f32,
    pub dw: f32,
    pub buttons: u32,
}

impl MouseEvent {
    pub fn set_button_state(&mut self, button: impl Into<MouseButton>, state: bool) {
        let bit = button.into() as u32;
        if state {
            self.buttons |= bit;
        } else {
            self.buttons &= !bit;
        }
    }
}

impl Display for MouseEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{};{};{};{}", self.dx, self.dy, self.dw, self.buttons)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum MouseButton {
    Left = 1 << 0,
    Right = 1 << 1,
    Middle = 1 << 2,
}
