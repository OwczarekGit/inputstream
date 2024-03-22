use std::{fmt::Display, mem::transmute, str::FromStr};

#[cfg(target_os = "linux")]
mod linux;

#[cfg(feature = "sdl2")]
mod sdl2;

use super::difference::Difference;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct KeyboardEvent(pub u32, pub u32, pub u32, pub u32);

impl KeyboardEvent {
    pub fn update(&mut self, ev: impl Into<KeyboardEventGroup>, state: bool) {
        let ev = ev.into();
        match ev {
            KeyboardEventGroup::G1(key) => {
                if state {
                    self.0 |= key as u32;
                } else {
                    self.0 &= !(key as u32);
                }
            }
            KeyboardEventGroup::G2(key) => {
                if state {
                    self.1 |= key as u32;
                } else {
                    self.1 &= !(key as u32);
                }
            }
            KeyboardEventGroup::G3(key) => {
                if state {
                    self.2 |= key as u32;
                } else {
                    self.2 &= !(key as u32);
                }
            }
            KeyboardEventGroup::G4(key) => {
                if state {
                    self.3 |= key as u32;
                } else {
                    self.3 &= !(key as u32);
                }
            }
        }
    }
}

impl Display for KeyboardEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{};{};{};{}", self.0, self.1, self.2, self.3,)
    }
}

impl From<(u32, u32, u32, u32)> for KeyboardEvent {
    fn from(v: (u32, u32, u32, u32)) -> Self {
        Self(v.0, v.1, v.2, v.3)
    }
}

impl Difference for KeyboardEvent {
    type Output = Vec<(bool, KeyboardEventGroup)>;
    fn get_diff(&self, other: &Self) -> Self::Output {
        let mut results = vec![];
        let g1_diff = self.0 ^ other.0;
        let g2_diff = self.1 ^ other.1;
        let g3_diff = self.2 ^ other.2;

        // NOTE: This should be fine as long as there is a variant corresponding to each bitmask.
        for bit in 0..32u32 {
            let mask = 1 << bit;
            if mask & g1_diff == mask {
                let key: Group1Event = unsafe { transmute(mask) };
                let pressed = other.0 & mask == mask;
                results.push((pressed, KeyboardEventGroup::G1(key)));
            }
            if mask & g2_diff == mask {
                let key: Group2Event = unsafe { transmute(mask) };
                let pressed = other.1 & mask == mask;
                results.push((pressed, KeyboardEventGroup::G2(key)));
            }
            if mask & g3_diff == mask {
                let key: Group3Event = unsafe { transmute(mask) };
                let pressed = other.2 & mask == mask;
                results.push((pressed, KeyboardEventGroup::G3(key)));
            }
        }

        results
    }
}

impl FromStr for KeyboardEvent {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.trim().split(';');

        let g1: u32 = split
            .next()
            .ok_or("Missing keyboard group1")?
            .trim()
            .parse()
            .unwrap_or(0);

        let g2: u32 = split
            .next()
            .ok_or("Missing keyboard group2")?
            .trim()
            .parse()
            .unwrap_or(0);

        let g3: u32 = split
            .next()
            .ok_or("Missing keyboard group3")?
            .trim()
            .parse()
            .unwrap_or(0);

        let g4: u32 = split
            .next()
            .ok_or("Missing keyboard group4")?
            .trim()
            .parse()
            .unwrap_or(0);

        Ok(Self(g1, g2, g3, g4))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum KeyboardEventGroup {
    G1(Group1Event),
    G2(Group2Event),
    G3(Group3Event),
    G4(Group4Event),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Group1Event {
    A = 1 << 0,
    B = 1 << 1,
    C = 1 << 2,
    D = 1 << 3,
    E = 1 << 4,
    F = 1 << 5,
    G = 1 << 6,
    H = 1 << 7,
    I = 1 << 8,
    J = 1 << 9,
    K = 1 << 10,
    L = 1 << 11,
    M = 1 << 12,
    N = 1 << 13,
    O = 1 << 14,
    P = 1 << 15,
    Q = 1 << 16,
    R = 1 << 17,
    S = 1 << 18,
    T = 1 << 19,
    U = 1 << 20,
    V = 1 << 21,
    W = 1 << 22,
    X = 1 << 23,
    Y = 1 << 24,
    Z = 1 << 25,
    Backslash = 1 << 26,
    LeftBrace = 1 << 27,
    RightBrace = 1 << 28,
    Print = 1 << 29,
    ScrollLock = 1 << 30,
    Pause = 1 << 31,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Group2Event {
    Num0 = 1 << 0,
    Num1 = 1 << 1,
    Num2 = 1 << 2,
    Num3 = 1 << 3,
    Num4 = 1 << 4,
    Num5 = 1 << 5,
    Num6 = 1 << 6,
    Num7 = 1 << 7,
    Num8 = 1 << 8,
    Num9 = 1 << 9,
    Minus = 1 << 10,
    Equal = 1 << 11,
    Backspace = 1 << 12,
    Tilde = 1 << 13,
    Enter = 1 << 14,
    Tab = 1 << 15,
    Esc = 1 << 16,
    CapsLock = 1 << 17,
    LeftShift = 1 << 18,
    LeftCtrl = 1 << 19,
    LeftAlt = 1 << 20,
    LeftMeta = 1 << 21,
    Space = 1 << 22,
    RightAlt = 1 << 23,
    ContextMenu = 1 << 24,
    Rightctrl = 1 << 25,
    RightShift = 1 << 26,
    Comma = 1 << 27,
    Dot = 1 << 28,
    Slash = 1 << 29,
    Semicolon = 1 << 30,
    Quote = 1 << 31,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Group3Event {
    F1 = 1 << 0,
    F2 = 1 << 1,
    F3 = 1 << 2,
    F4 = 1 << 3,
    F5 = 1 << 4,
    F6 = 1 << 5,
    F7 = 1 << 6,
    F8 = 1 << 7,
    F9 = 1 << 8,
    F10 = 1 << 9,
    F11 = 1 << 10,
    F12 = 1 << 11,
    Left = 1 << 12,
    Right = 1 << 13,
    Up = 1 << 14,
    Down = 1 << 15,
    Insert = 1 << 16,
    Delete = 1 << 17,
    Home = 1 << 18,
    End = 1 << 19,
    PageUp = 1 << 20,
    PageDown = 1 << 21,
    Numpad0 = 1 << 22,
    Numpad1 = 1 << 23,
    Numpad2 = 1 << 24,
    Numpad3 = 1 << 25,
    Numpad4 = 1 << 26,
    Numpad5 = 1 << 27,
    Numpad6 = 1 << 28,
    Numpad7 = 1 << 29,
    Numpad8 = 1 << 30,
    Numpad9 = 1 << 31,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Group4Event {
    None = 1 << 0,
}
