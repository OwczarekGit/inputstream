use std::{fmt::Display, str::FromStr};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct OsuEvent(pub u32);

impl OsuEvent {
    pub fn set_key_state(&mut self, button: impl Into<OsuKey>, state: bool) {
        let bit = button.into() as u32;
        if state {
            self.0 |= bit;
        } else {
            self.0 &= !bit;
        }
    }

    pub fn key_state(&self, key: impl Into<OsuKey>) -> bool {
        self.0 & key.into() as u32 > 0
    }
}

impl Display for OsuEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for OsuEvent {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let keys: u32 = s.trim().parse().unwrap_or(0);
        Ok(Self(keys))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum OsuKey {
    Key1 = 1 << 0,
    Key2 = 1 << 1,
}
