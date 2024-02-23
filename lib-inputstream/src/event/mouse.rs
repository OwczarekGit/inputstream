use std::{fmt::Display, str::FromStr};

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

    pub fn button_state(&self, button: impl Into<MouseButton>) -> bool {
        let button = button.into() as u32;
        self.buttons & button > 0
    }
}

impl Display for MouseEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{};{};{};{}", self.dx, self.dy, self.dw, self.buttons)
    }
}

impl FromStr for MouseEvent {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(';');
        let dx: f32 = split
            .next()
            .ok_or("Missing delta X.")?
            .trim()
            .parse()
            .unwrap_or(0.0);
        let dy: f32 = split
            .next()
            .ok_or("Missing delta Y.")?
            .trim()
            .parse()
            .unwrap_or(0.0);
        let dw: f32 = split
            .next()
            .ok_or("Missing delta W.")?
            .trim()
            .parse()
            .unwrap_or(0.0);
        let buttons: u32 = split.next().ok_or("Missing buttons.")?.parse().unwrap_or(0);

        Ok(Self {
            dx,
            dy,
            dw,
            buttons,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum MouseButton {
    Left = 1 << 0,
    Right = 1 << 1,
    Middle = 1 << 2,
}
