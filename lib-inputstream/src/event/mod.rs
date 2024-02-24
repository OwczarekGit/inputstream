use crate::consts::{
    GAMEPAD_PROTOCOL_NAME, KEYBOARD_PROTOCOL_NAME, MOUSE_PROTOCOL_NAME, OSU_PROTOCOL_NAME,
};
use std::{fmt::Display, str::FromStr};

pub mod gamepad;
pub mod keyboard;
pub mod mouse;
pub mod osu;

pub mod difference;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum EventType {
    Osu(osu::OsuEvent),
    Mouse(mouse::MouseEvent),
    Keyboard(keyboard::KeyboardEvent),
    Gamepad(gamepad::GamepadEvent),
}

impl Display for EventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EventType::Osu(ev) => writeln!(f, "{OSU_PROTOCOL_NAME}|{ev}"),
            EventType::Mouse(ev) => writeln!(f, "{MOUSE_PROTOCOL_NAME}|{ev}"),
            EventType::Keyboard(ev) => writeln!(f, "{KEYBOARD_PROTOCOL_NAME}|{ev}"),
            EventType::Gamepad(ev) => writeln!(f, "{GAMEPAD_PROTOCOL_NAME}|{ev}"),
        }
    }
}

impl FromStr for EventType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.trim().split('|');
        let protocol = split.next().ok_or("Missing protocol.")?;
        let data = split.next().ok_or("Missing data.")?;

        match protocol {
            OSU_PROTOCOL_NAME => Ok(Self::Osu(data.parse()?)),
            MOUSE_PROTOCOL_NAME => Ok(Self::Mouse(data.parse()?)),
            KEYBOARD_PROTOCOL_NAME => Ok(Self::Keyboard(data.parse()?)),
            GAMEPAD_PROTOCOL_NAME => Ok(Self::Gamepad(data.parse()?)),
            other => Err(other.to_owned()),
        }
    }
}
