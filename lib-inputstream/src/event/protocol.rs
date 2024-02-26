use std::{fmt::Display, str::FromStr};

use crate::consts::{
    GAMEPAD_PROTOCOL_NAME, KEYBOARD_PROTOCOL_NAME, MOUSE_PROTOCOL_NAME, OSU_PROTOCOL_NAME,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Protocol {
    Osu = 0,
    Mouse = 1,
    Keyboard = 2,
    Gamepad = 3,
}

impl Display for Protocol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self as u32)
    }
}

impl FromStr for Protocol {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" | OSU_PROTOCOL_NAME => Ok(Self::Osu),
            "1" | MOUSE_PROTOCOL_NAME => Ok(Self::Mouse),
            "2" | KEYBOARD_PROTOCOL_NAME => Ok(Self::Keyboard),
            "3" | GAMEPAD_PROTOCOL_NAME => Ok(Self::Gamepad),
            s => Err(s.to_owned()),
        }
    }
}
