use std::fmt::Display;

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
