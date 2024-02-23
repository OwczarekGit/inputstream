use crate::consts::{KEYBOARD_PROTOCOL_NAME, MOUSE_PROTOCOL_NAME, OSU_PROTOCOL_NAME};
use std::fmt::Display;

pub mod keyboard;
pub mod mouse;
pub mod osu;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum EventType {
    Osu(osu::OsuEvent),
    Mouse(mouse::MouseEvent),
    Keyboard(keyboard::KeyboardEvent),
}

impl Display for EventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EventType::Osu(ev) => writeln!(f, "{OSU_PROTOCOL_NAME}|{ev}"),
            EventType::Mouse(ev) => writeln!(f, "{MOUSE_PROTOCOL_NAME}|{ev}"),
            EventType::Keyboard(ev) => writeln!(f, "{KEYBOARD_PROTOCOL_NAME}|{ev}"),
        }
    }
}
