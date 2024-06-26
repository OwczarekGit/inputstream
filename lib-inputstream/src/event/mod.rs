use std::{fmt::Display, str::FromStr};

use self::{gamepad::gamepad_state::GamepadState, protocol::Protocol};

pub mod gamepad;
pub mod keyboard;
pub mod mouse;
pub mod osu;
pub mod protocol;

pub mod difference;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum EventType {
    Ping,
    Osu(osu::OsuState),
    Mouse(mouse::MouseState),
    Keyboard(keyboard::KeyboardState),
    Gamepad(GamepadState),
}

impl Default for EventType {
    fn default() -> Self {
        Self::Ping
    }
}

impl Display for EventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EventType::Ping => write!(f, "PING"),
            EventType::Osu(ev) => write!(f, "{}|{ev}", Protocol::Osu),
            EventType::Mouse(ev) => write!(f, "{}|{ev}", Protocol::Mouse),
            EventType::Keyboard(ev) => write!(f, "{}|{ev}", Protocol::Keyboard),
            EventType::Gamepad(ev) => write!(f, "{}|{ev}", Protocol::Gamepad),
        }
    }
}

impl FromStr for EventType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "PING" {
            return Ok(Self::Ping);
        }
        let mut split = s.trim().split('|');
        let protocol = split.next().ok_or("Missing protocol.")?.trim();
        let data = split.next().ok_or("Missing data.")?.trim();

        match protocol.parse()? {
            Protocol::Osu => Ok(EventType::Osu(data.parse()?)),
            Protocol::Mouse => Ok(EventType::Mouse(data.parse()?)),
            Protocol::Keyboard => Ok(EventType::Keyboard(data.parse()?)),
            Protocol::Gamepad => Ok(EventType::Gamepad(data.parse()?)),
        }
    }
}
