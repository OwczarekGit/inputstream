use std::{fmt::Display, str::FromStr};

use super::{difference::Difference, EventType};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct OsuState(pub u32);

impl OsuState {
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

impl From<OsuState> for EventType {
    fn from(value: OsuState) -> Self {
        Self::Osu(value)
    }
}

impl Display for OsuState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for OsuState {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let keys: u32 = s.trim().parse().unwrap_or(0);
        Ok(Self(keys))
    }
}

impl Difference for OsuState {
    type Output = (Option<(bool, OsuKey)>, Option<(bool, OsuKey)>);

    fn get_diff(&self, other: &Self) -> Self::Output {
        (
            (self.0 & OsuKey::Key1 as u32)
                .ne(&(other.0 & OsuKey::Key1 as u32))
                .then_some((other.0 & OsuKey::Key1 as u32 > 0, OsuKey::Key1)),
            (self.0 & OsuKey::Key2 as u32)
                .ne(&(other.0 & OsuKey::Key2 as u32))
                .then_some((other.0 & OsuKey::Key2 as u32 > 0, OsuKey::Key2)),
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum OsuKey {
    Key1 = 1 << 0,
    Key2 = 1 << 1,
}
