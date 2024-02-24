use std::{fmt::Display, mem::transmute, str::FromStr};

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

        // NOTE: This should be fine as long as there is a variant coresponding for each bitmask.
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

#[cfg(feature = "sdl2")]
impl TryFrom<sdl2::keyboard::Keycode> for KeyboardEventGroup {
    type Error = ();

    fn try_from(v: sdl2::keyboard::Keycode) -> Result<Self, Self::Error> {
        use sdl2::keyboard::Keycode;
        match v {
            // Group 1
            Keycode::A => Ok(Self::G1(Group1Event::A)),
            Keycode::B => Ok(Self::G1(Group1Event::B)),
            Keycode::C => Ok(Self::G1(Group1Event::C)),
            Keycode::D => Ok(Self::G1(Group1Event::D)),
            Keycode::E => Ok(Self::G1(Group1Event::E)),
            Keycode::F => Ok(Self::G1(Group1Event::F)),
            Keycode::G => Ok(Self::G1(Group1Event::G)),
            Keycode::H => Ok(Self::G1(Group1Event::H)),
            Keycode::I => Ok(Self::G1(Group1Event::I)),
            Keycode::J => Ok(Self::G1(Group1Event::J)),
            Keycode::K => Ok(Self::G1(Group1Event::K)),
            Keycode::L => Ok(Self::G1(Group1Event::L)),
            Keycode::M => Ok(Self::G1(Group1Event::M)),
            Keycode::N => Ok(Self::G1(Group1Event::N)),
            Keycode::O => Ok(Self::G1(Group1Event::O)),
            Keycode::P => Ok(Self::G1(Group1Event::P)),
            Keycode::Q => Ok(Self::G1(Group1Event::Q)),
            Keycode::R => Ok(Self::G1(Group1Event::R)),
            Keycode::S => Ok(Self::G1(Group1Event::S)),
            Keycode::T => Ok(Self::G1(Group1Event::T)),
            Keycode::U => Ok(Self::G1(Group1Event::U)),
            Keycode::V => Ok(Self::G1(Group1Event::V)),
            Keycode::W => Ok(Self::G1(Group1Event::W)),
            Keycode::X => Ok(Self::G1(Group1Event::X)),
            Keycode::Y => Ok(Self::G1(Group1Event::Y)),
            Keycode::Z => Ok(Self::G1(Group1Event::Z)),
            Keycode::Backslash => Ok(Self::G1(Group1Event::Backslash)),
            Keycode::LeftBracket => Ok(Self::G1(Group1Event::LeftBrace)),
            Keycode::RightBracket => Ok(Self::G1(Group1Event::RightBrace)),
            Keycode::PrintScreen => Ok(Self::G1(Group1Event::Print)),
            Keycode::ScrollLock => Ok(Self::G1(Group1Event::ScrollLock)),
            Keycode::Pause => Ok(Self::G1(Group1Event::Pause)),

            // Group 2
            Keycode::Num0 => Ok(Self::G2(Group2Event::Num0)),
            Keycode::Num1 => Ok(Self::G2(Group2Event::Num1)),
            Keycode::Num2 => Ok(Self::G2(Group2Event::Num2)),
            Keycode::Num3 => Ok(Self::G2(Group2Event::Num3)),
            Keycode::Num4 => Ok(Self::G2(Group2Event::Num4)),
            Keycode::Num5 => Ok(Self::G2(Group2Event::Num5)),
            Keycode::Num6 => Ok(Self::G2(Group2Event::Num6)),
            Keycode::Num7 => Ok(Self::G2(Group2Event::Num7)),
            Keycode::Num8 => Ok(Self::G2(Group2Event::Num8)),
            Keycode::Num9 => Ok(Self::G2(Group2Event::Num9)),
            Keycode::Minus => Ok(Self::G2(Group2Event::Minus)),
            Keycode::Equals => Ok(Self::G2(Group2Event::Equal)),
            Keycode::Backspace => Ok(Self::G2(Group2Event::Backspace)),
            Keycode::Backquote => Ok(Self::G2(Group2Event::Tilde)),
            Keycode::Return => Ok(Self::G2(Group2Event::Enter)),
            Keycode::Tab => Ok(Self::G2(Group2Event::Tab)),
            Keycode::Escape => Ok(Self::G2(Group2Event::Esc)),
            Keycode::CapsLock => Ok(Self::G2(Group2Event::CapsLock)),
            Keycode::LShift => Ok(Self::G2(Group2Event::LeftShift)),
            Keycode::LCtrl => Ok(Self::G2(Group2Event::LeftCtrl)),
            Keycode::LAlt => Ok(Self::G2(Group2Event::LeftAlt)),
            Keycode::LGui => Ok(Self::G2(Group2Event::LeftMeta)),
            Keycode::Space => Ok(Self::G2(Group2Event::Space)),
            Keycode::RAlt => Ok(Self::G2(Group2Event::RightAlt)),
            Keycode::Application => Ok(Self::G2(Group2Event::ContextMenu)),
            Keycode::RCtrl => Ok(Self::G2(Group2Event::RightAlt)),
            Keycode::RShift => Ok(Self::G2(Group2Event::RightShift)),
            Keycode::Comma => Ok(Self::G2(Group2Event::Comma)),
            Keycode::Period => Ok(Self::G2(Group2Event::Dot)),
            Keycode::Slash => Ok(Self::G2(Group2Event::Slash)),
            Keycode::Semicolon => Ok(Self::G2(Group2Event::Semicolon)),
            Keycode::Quote => Ok(Self::G2(Group2Event::Quote)),

            // Group 3
            Keycode::F1 => Ok(Self::G3(Group3Event::F1)),
            Keycode::F2 => Ok(Self::G3(Group3Event::F2)),
            Keycode::F3 => Ok(Self::G3(Group3Event::F3)),
            Keycode::F4 => Ok(Self::G3(Group3Event::F4)),
            Keycode::F5 => Ok(Self::G3(Group3Event::F5)),
            Keycode::F6 => Ok(Self::G3(Group3Event::F6)),
            Keycode::F7 => Ok(Self::G3(Group3Event::F7)),
            Keycode::F8 => Ok(Self::G3(Group3Event::F8)),
            Keycode::F9 => Ok(Self::G3(Group3Event::F9)),
            Keycode::F10 => Ok(Self::G3(Group3Event::F10)),
            Keycode::F11 => Ok(Self::G3(Group3Event::F11)),
            Keycode::F12 => Ok(Self::G3(Group3Event::F12)),
            Keycode::Left => Ok(Self::G3(Group3Event::Left)),
            Keycode::Right => Ok(Self::G3(Group3Event::Right)),
            Keycode::Up => Ok(Self::G3(Group3Event::Up)),
            Keycode::Down => Ok(Self::G3(Group3Event::Down)),
            Keycode::Insert => Ok(Self::G3(Group3Event::Insert)),
            Keycode::Delete => Ok(Self::G3(Group3Event::Delete)),
            Keycode::Home => Ok(Self::G3(Group3Event::Home)),
            Keycode::End => Ok(Self::G3(Group3Event::End)),
            Keycode::PageUp => Ok(Self::G3(Group3Event::PageUp)),
            Keycode::PageDown => Ok(Self::G3(Group3Event::PageDown)),
            Keycode::Kp0 => Ok(Self::G3(Group3Event::Numpad0)),
            Keycode::Kp1 => Ok(Self::G3(Group3Event::Numpad1)),
            Keycode::Kp2 => Ok(Self::G3(Group3Event::Numpad2)),
            Keycode::Kp3 => Ok(Self::G3(Group3Event::Numpad3)),
            Keycode::Kp4 => Ok(Self::G3(Group3Event::Numpad4)),
            Keycode::Kp5 => Ok(Self::G3(Group3Event::Numpad5)),
            Keycode::Kp6 => Ok(Self::G3(Group3Event::Numpad6)),
            Keycode::Kp7 => Ok(Self::G3(Group3Event::Numpad7)),
            Keycode::Kp8 => Ok(Self::G3(Group3Event::Numpad8)),
            Keycode::Kp9 => Ok(Self::G3(Group3Event::Numpad9)),

            _ => Err(()),
        }
    }
}

#[cfg(target_os = "linux")]
impl From<Group1Event> for evdev::Key {
    fn from(value: Group1Event) -> Self {
        use evdev::Key;
        match value {
            Group1Event::A => Key::KEY_A,
            Group1Event::B => Key::KEY_B,
            Group1Event::C => Key::KEY_C,
            Group1Event::D => Key::KEY_D,
            Group1Event::E => Key::KEY_E,
            Group1Event::F => Key::KEY_F,
            Group1Event::G => Key::KEY_G,
            Group1Event::H => Key::KEY_H,
            Group1Event::I => Key::KEY_I,
            Group1Event::J => Key::KEY_J,
            Group1Event::K => Key::KEY_K,
            Group1Event::L => Key::KEY_L,
            Group1Event::M => Key::KEY_M,
            Group1Event::N => Key::KEY_N,
            Group1Event::O => Key::KEY_O,
            Group1Event::P => Key::KEY_P,
            Group1Event::Q => Key::KEY_Q,
            Group1Event::R => Key::KEY_R,
            Group1Event::S => Key::KEY_S,
            Group1Event::T => Key::KEY_T,
            Group1Event::U => Key::KEY_U,
            Group1Event::V => Key::KEY_V,
            Group1Event::W => Key::KEY_W,
            Group1Event::X => Key::KEY_X,
            Group1Event::Y => Key::KEY_Y,
            Group1Event::Z => Key::KEY_Z,
            Group1Event::Backslash => Key::KEY_BACKSLASH,
            Group1Event::LeftBrace => Key::KEY_LEFTBRACE,
            Group1Event::RightBrace => Key::KEY_RIGHTBRACE,
            Group1Event::Print => Key::KEY_PRINT,
            Group1Event::ScrollLock => Key::KEY_SCROLLLOCK,
            Group1Event::Pause => Key::KEY_PAUSE,
        }
    }
}

#[cfg(target_os = "linux")]
impl From<Group2Event> for evdev::Key {
    fn from(value: Group2Event) -> Self {
        use evdev::Key;
        match value {
            Group2Event::Num0 => Key::KEY_NUMERIC_0,
            Group2Event::Num1 => Key::KEY_NUMERIC_1,
            Group2Event::Num2 => Key::KEY_NUMERIC_2,
            Group2Event::Num3 => Key::KEY_NUMERIC_3,
            Group2Event::Num4 => Key::KEY_NUMERIC_4,
            Group2Event::Num5 => Key::KEY_NUMERIC_5,
            Group2Event::Num6 => Key::KEY_NUMERIC_6,
            Group2Event::Num7 => Key::KEY_NUMERIC_7,
            Group2Event::Num8 => Key::KEY_NUMERIC_8,
            Group2Event::Num9 => Key::KEY_NUMERIC_9,
            Group2Event::Minus => Key::KEY_MINUS,
            Group2Event::Equal => Key::KEY_EQUAL,
            Group2Event::Backspace => Key::KEY_BACKSPACE,
            Group2Event::Tilde => Key::KEY_GRAVE,
            Group2Event::Enter => Key::KEY_ENTER,
            Group2Event::Tab => Key::KEY_TAB,
            Group2Event::Esc => Key::KEY_ESC,
            Group2Event::CapsLock => Key::KEY_CAPSLOCK,
            Group2Event::LeftShift => Key::KEY_LEFTSHIFT,
            Group2Event::LeftCtrl => Key::KEY_LEFTCTRL,
            Group2Event::LeftAlt => Key::KEY_LEFTALT,
            Group2Event::LeftMeta => Key::KEY_LEFTMETA,
            Group2Event::Space => Key::KEY_SPACE,
            Group2Event::RightAlt => Key::KEY_RIGHTALT,
            Group2Event::ContextMenu => Key::KEY_CONTEXT_MENU,
            Group2Event::Rightctrl => Key::KEY_RIGHTCTRL,
            Group2Event::RightShift => Key::KEY_RIGHTSHIFT,
            Group2Event::Comma => Key::KEY_COMMA,
            Group2Event::Dot => Key::KEY_DOT,
            Group2Event::Slash => Key::KEY_SLASH,
            Group2Event::Semicolon => Key::KEY_SEMICOLON,
            Group2Event::Quote => Key::KEY_APOSTROPHE,
        }
    }
}

#[cfg(target_os = "linux")]
impl From<Group3Event> for evdev::Key {
    fn from(value: Group3Event) -> Self {
        use evdev::Key;
        match value {
            Group3Event::F1 => Key::KEY_F1,
            Group3Event::F2 => Key::KEY_F2,
            Group3Event::F3 => Key::KEY_F3,
            Group3Event::F4 => Key::KEY_F4,
            Group3Event::F5 => Key::KEY_F5,
            Group3Event::F6 => Key::KEY_F6,
            Group3Event::F7 => Key::KEY_F7,
            Group3Event::F8 => Key::KEY_F8,
            Group3Event::F9 => Key::KEY_F9,
            Group3Event::F10 => Key::KEY_F10,
            Group3Event::F11 => Key::KEY_F11,
            Group3Event::F12 => Key::KEY_F12,
            Group3Event::Left => Key::KEY_LEFT,
            Group3Event::Right => Key::KEY_RIGHT,
            Group3Event::Up => Key::KEY_UP,
            Group3Event::Down => Key::KEY_DOWN,
            Group3Event::Insert => Key::KEY_INSERT,
            Group3Event::Delete => Key::KEY_DELETE,
            Group3Event::Home => Key::KEY_HOME,
            Group3Event::End => Key::KEY_END,
            Group3Event::PageUp => Key::KEY_PAGEUP,
            Group3Event::PageDown => Key::KEY_PAGEDOWN,
            Group3Event::Numpad0 => Key::KEY_KP0,
            Group3Event::Numpad1 => Key::KEY_KP1,
            Group3Event::Numpad2 => Key::KEY_KP2,
            Group3Event::Numpad3 => Key::KEY_KP3,
            Group3Event::Numpad4 => Key::KEY_KP4,
            Group3Event::Numpad5 => Key::KEY_KP5,
            Group3Event::Numpad6 => Key::KEY_KP6,
            Group3Event::Numpad7 => Key::KEY_KP7,
            Group3Event::Numpad8 => Key::KEY_KP8,
            Group3Event::Numpad9 => Key::KEY_KP9,
        }
    }
}

#[cfg(target_os = "linux")]
impl From<KeyboardEventGroup> for evdev::Key {
    fn from(v: KeyboardEventGroup) -> Self {
        match v {
            KeyboardEventGroup::G1(k) => k.into(),
            KeyboardEventGroup::G2(k) => k.into(),
            KeyboardEventGroup::G3(k) => k.into(),
            KeyboardEventGroup::G4(_k) => evdev::Key::KEY_A,
        }
    }
}
