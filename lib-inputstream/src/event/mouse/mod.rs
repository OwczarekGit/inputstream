use std::{
    fmt::Display,
    str::{FromStr, Split},
};

#[cfg(target_os = "linux")]
mod linux;

use super::{difference::Difference, EventType};

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

impl From<MouseEvent> for EventType {
    fn from(value: MouseEvent) -> Self {
        Self::Mouse(value)
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

        let next_float = |split: &mut Split<'_, _>, name: &str| {
            Ok::<f32, String>(
                split
                    .next()
                    .ok_or(format!("Missing {name}."))?
                    .trim()
                    .parse()
                    .unwrap_or(0.0),
            )
        };

        let dx = next_float(&mut split, "delta X")?;
        let dy = next_float(&mut split, "delta Y")?;
        let dw = next_float(&mut split, "delta W")?;
        let buttons: u32 = split.next().ok_or("Missing buttons.")?.parse().unwrap_or(0);

        Ok(Self {
            dx,
            dy,
            dw,
            buttons,
        })
    }
}

impl Difference for MouseEvent {
    type Output = (
        Option<f32>,
        Option<f32>,
        Option<f32>,
        Vec<(bool, MouseButton)>,
    );

    fn get_diff(&self, other: &Self) -> Self::Output {
        let motion_diff = |val: f32| val.abs().gt(&0.0).then_some(val);

        let add_button_change = |v: &mut Vec<(bool, MouseButton)>, btn: MouseButton| {
            if (self.buttons & btn as u32) != (other.buttons & btn as u32) {
                v.push((other.buttons & btn as u32 > 0, btn));
            }
        };

        let dx = motion_diff(other.dx);
        let dy = motion_diff(other.dy);
        let dw = motion_diff(other.dw);
        let mut buttons = vec![];

        add_button_change(&mut buttons, MouseButton::Left);
        add_button_change(&mut buttons, MouseButton::Right);
        add_button_change(&mut buttons, MouseButton::Middle);

        (dx, dy, dw, buttons)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum MouseButton {
    Left = 1 << 0,
    Right = 1 << 1,
    Middle = 1 << 2,
}
