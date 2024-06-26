use std::{fmt::Display, str::FromStr};

use super::{Group1Event, Group2Event, Group3Event, KeyboardEventGroup};

#[derive(Debug, Clone, Copy)]
pub enum Key {
    // G1
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    Backslash,
    LeftBrace,
    RightBrace,
    Print,
    ScrollLock,
    Pause,

    // G2
    Num0,
    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,
    Minus,
    Equal,
    Backspace,
    Tilde,
    Enter,
    Tab,
    Esc,
    CapsLock,
    LeftShift,
    LeftCtrl,
    LeftAlt,
    LeftMeta,
    Space,
    RightAlt,
    ContextMenu,
    RightCtrl,
    RightShift,
    Comma,
    Dot,
    Slash,
    Semicolon,
    Quote,

    // G3
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    Left,
    Right,
    Up,
    Down,
    Insert,
    Delete,
    Home,
    End,
    PageUp,
    PageDown,
    Numpad0,
    Numpad1,
    Numpad2,
    Numpad3,
    Numpad4,
    Numpad5,
    Numpad6,
    Numpad7,
    Numpad8,
    Numpad9,
}

impl TryFrom<String> for Key {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl From<Key> for KeyboardEventGroup {
    fn from(value: Key) -> Self {
        match value {
            Key::A => KeyboardEventGroup::G1(Group1Event::A),
            Key::B => KeyboardEventGroup::G1(Group1Event::B),
            Key::C => KeyboardEventGroup::G1(Group1Event::C),
            Key::D => KeyboardEventGroup::G1(Group1Event::D),
            Key::E => KeyboardEventGroup::G1(Group1Event::E),
            Key::F => KeyboardEventGroup::G1(Group1Event::F),
            Key::G => KeyboardEventGroup::G1(Group1Event::G),
            Key::H => KeyboardEventGroup::G1(Group1Event::H),
            Key::I => KeyboardEventGroup::G1(Group1Event::I),
            Key::J => KeyboardEventGroup::G1(Group1Event::J),
            Key::K => KeyboardEventGroup::G1(Group1Event::K),
            Key::L => KeyboardEventGroup::G1(Group1Event::L),
            Key::M => KeyboardEventGroup::G1(Group1Event::M),
            Key::N => KeyboardEventGroup::G1(Group1Event::N),
            Key::O => KeyboardEventGroup::G1(Group1Event::O),
            Key::P => KeyboardEventGroup::G1(Group1Event::P),
            Key::Q => KeyboardEventGroup::G1(Group1Event::Q),
            Key::R => KeyboardEventGroup::G1(Group1Event::R),
            Key::S => KeyboardEventGroup::G1(Group1Event::S),
            Key::T => KeyboardEventGroup::G1(Group1Event::T),
            Key::U => KeyboardEventGroup::G1(Group1Event::U),
            Key::V => KeyboardEventGroup::G1(Group1Event::V),
            Key::W => KeyboardEventGroup::G1(Group1Event::W),
            Key::X => KeyboardEventGroup::G1(Group1Event::X),
            Key::Y => KeyboardEventGroup::G1(Group1Event::Y),
            Key::Z => KeyboardEventGroup::G1(Group1Event::Z),
            Key::Backslash => KeyboardEventGroup::G1(Group1Event::Backslash),
            Key::LeftBrace => KeyboardEventGroup::G1(Group1Event::LeftBrace),
            Key::RightBrace => KeyboardEventGroup::G1(Group1Event::RightBrace),
            Key::Print => KeyboardEventGroup::G1(Group1Event::Print),
            Key::ScrollLock => KeyboardEventGroup::G1(Group1Event::ScrollLock),
            Key::Pause => KeyboardEventGroup::G1(Group1Event::Pause),

            Key::Num0 => KeyboardEventGroup::G2(Group2Event::Num0),
            Key::Num1 => KeyboardEventGroup::G2(Group2Event::Num1),
            Key::Num2 => KeyboardEventGroup::G2(Group2Event::Num2),
            Key::Num3 => KeyboardEventGroup::G2(Group2Event::Num3),
            Key::Num4 => KeyboardEventGroup::G2(Group2Event::Num4),
            Key::Num5 => KeyboardEventGroup::G2(Group2Event::Num5),
            Key::Num6 => KeyboardEventGroup::G2(Group2Event::Num6),
            Key::Num7 => KeyboardEventGroup::G2(Group2Event::Num7),
            Key::Num8 => KeyboardEventGroup::G2(Group2Event::Num8),
            Key::Num9 => KeyboardEventGroup::G2(Group2Event::Num9),
            Key::Minus => KeyboardEventGroup::G2(Group2Event::Minus),
            Key::Equal => KeyboardEventGroup::G2(Group2Event::Equal),
            Key::Backspace => KeyboardEventGroup::G2(Group2Event::Backspace),
            Key::Tilde => KeyboardEventGroup::G2(Group2Event::Tilde),
            Key::Enter => KeyboardEventGroup::G2(Group2Event::Enter),
            Key::Tab => KeyboardEventGroup::G2(Group2Event::Tab),
            Key::Esc => KeyboardEventGroup::G2(Group2Event::Esc),
            Key::CapsLock => KeyboardEventGroup::G2(Group2Event::CapsLock),
            Key::LeftShift => KeyboardEventGroup::G2(Group2Event::LeftShift),
            Key::LeftCtrl => KeyboardEventGroup::G2(Group2Event::LeftCtrl),
            Key::LeftAlt => KeyboardEventGroup::G2(Group2Event::LeftAlt),
            Key::LeftMeta => KeyboardEventGroup::G2(Group2Event::LeftMeta),
            Key::Space => KeyboardEventGroup::G2(Group2Event::Space),
            Key::RightAlt => KeyboardEventGroup::G2(Group2Event::RightAlt),
            Key::ContextMenu => KeyboardEventGroup::G2(Group2Event::ContextMenu),
            Key::RightCtrl => KeyboardEventGroup::G2(Group2Event::RightCtrl),
            Key::RightShift => KeyboardEventGroup::G2(Group2Event::RightShift),
            Key::Comma => KeyboardEventGroup::G2(Group2Event::Comma),
            Key::Dot => KeyboardEventGroup::G2(Group2Event::Dot),
            Key::Slash => KeyboardEventGroup::G2(Group2Event::Slash),
            Key::Semicolon => KeyboardEventGroup::G2(Group2Event::Semicolon),
            Key::Quote => KeyboardEventGroup::G2(Group2Event::Quote),

            Key::F1 => KeyboardEventGroup::G3(Group3Event::F1),
            Key::F2 => KeyboardEventGroup::G3(Group3Event::F2),
            Key::F3 => KeyboardEventGroup::G3(Group3Event::F3),
            Key::F4 => KeyboardEventGroup::G3(Group3Event::F4),
            Key::F5 => KeyboardEventGroup::G3(Group3Event::F5),
            Key::F6 => KeyboardEventGroup::G3(Group3Event::F6),
            Key::F7 => KeyboardEventGroup::G3(Group3Event::F7),
            Key::F8 => KeyboardEventGroup::G3(Group3Event::F8),
            Key::F9 => KeyboardEventGroup::G3(Group3Event::F9),
            Key::F10 => KeyboardEventGroup::G3(Group3Event::F10),
            Key::F11 => KeyboardEventGroup::G3(Group3Event::F11),
            Key::F12 => KeyboardEventGroup::G3(Group3Event::F12),
            Key::Left => KeyboardEventGroup::G3(Group3Event::Left),
            Key::Right => KeyboardEventGroup::G3(Group3Event::Right),
            Key::Up => KeyboardEventGroup::G3(Group3Event::Up),
            Key::Down => KeyboardEventGroup::G3(Group3Event::Down),
            Key::Insert => KeyboardEventGroup::G3(Group3Event::Insert),
            Key::Delete => KeyboardEventGroup::G3(Group3Event::Delete),
            Key::Home => KeyboardEventGroup::G3(Group3Event::Home),
            Key::End => KeyboardEventGroup::G3(Group3Event::End),
            Key::PageUp => KeyboardEventGroup::G3(Group3Event::PageUp),
            Key::PageDown => KeyboardEventGroup::G3(Group3Event::PageDown),
            Key::Numpad0 => KeyboardEventGroup::G3(Group3Event::Numpad0),
            Key::Numpad1 => KeyboardEventGroup::G3(Group3Event::Numpad1),
            Key::Numpad2 => KeyboardEventGroup::G3(Group3Event::Numpad2),
            Key::Numpad3 => KeyboardEventGroup::G3(Group3Event::Numpad3),
            Key::Numpad4 => KeyboardEventGroup::G3(Group3Event::Numpad4),
            Key::Numpad5 => KeyboardEventGroup::G3(Group3Event::Numpad5),
            Key::Numpad6 => KeyboardEventGroup::G3(Group3Event::Numpad6),
            Key::Numpad7 => KeyboardEventGroup::G3(Group3Event::Numpad7),
            Key::Numpad8 => KeyboardEventGroup::G3(Group3Event::Numpad8),
            Key::Numpad9 => KeyboardEventGroup::G3(Group3Event::Numpad9),
        }
    }
}

impl Display for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl FromStr for Key {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_lowercase();
        match s.as_str() {
            // FN keys
            "esc" | "escape" => Ok(Self::Esc),
            "f1" => Ok(Self::F1),
            "f2" => Ok(Self::F2),
            "f3" => Ok(Self::F3),
            "f4" => Ok(Self::F4),
            "f5" => Ok(Self::F5),
            "f6" => Ok(Self::F6),
            "f7" => Ok(Self::F7),
            "f8" => Ok(Self::F8),
            "f9" => Ok(Self::F9),
            "f10" => Ok(Self::F10),
            "f11" => Ok(Self::F11),
            "f12" => Ok(Self::F12),

            // Number row
            "~" | "`" => Ok(Self::Tilde),
            "1" => Ok(Self::Num1),
            "2" => Ok(Self::Num2),
            "3" => Ok(Self::Num3),
            "4" => Ok(Self::Num4),
            "5" => Ok(Self::Num5),
            "6" => Ok(Self::Num6),
            "7" => Ok(Self::Num7),
            "8" => Ok(Self::Num8),
            "9" => Ok(Self::Num9),
            "0" => Ok(Self::Num0),
            "-" | "minus" | "dash" => Ok(Self::Minus),
            "+" | "equal" | "=" => Ok(Self::Equal),
            "backspace" => Ok(Self::Backspace),

            // Row 1
            "tab" => Ok(Self::Tab),
            "q" => Ok(Self::Q),
            "w" => Ok(Self::W),
            "e" => Ok(Self::E),
            "r" => Ok(Self::R),
            "t" => Ok(Self::T),
            "y" => Ok(Self::Y),
            "u" => Ok(Self::U),
            "i" => Ok(Self::I),
            "o" => Ok(Self::O),
            "p" => Ok(Self::P),
            "[" => Ok(Self::LeftBrace),
            "]" => Ok(Self::RightBrace),
            "\\" | "|" => Ok(Self::Backslash),

            // Row 2
            "caps" => Ok(Self::CapsLock),
            "a" => Ok(Self::A),
            "s" => Ok(Self::S),
            "d" => Ok(Self::D),
            "f" => Ok(Self::F),
            "g" => Ok(Self::G),
            "h" => Ok(Self::H),
            "j" => Ok(Self::J),
            "k" => Ok(Self::K),
            "l" => Ok(Self::L),
            ";" | "semicolon" => Ok(Self::Semicolon),
            "'" | "\"" => Ok(Self::Quote),
            "enter" | "return" => Ok(Self::Enter),

            // Row 3
            "lshift" | "leftshift" | "shift" => Ok(Self::LeftShift),
            "z" => Ok(Self::Z),
            "x" => Ok(Self::X),
            "c" => Ok(Self::C),
            "v" => Ok(Self::V),
            "b" => Ok(Self::B),
            "n" => Ok(Self::N),
            "m" => Ok(Self::M),
            "," | "comma" => Ok(Self::Comma),
            "." | "dot" => Ok(Self::Dot),
            "/" | "slash" => Ok(Self::Slash),
            "rshift" | "rightshift" => Ok(Self::RightShift),

            // Row 4
            "lctrl" | "leftctrl" | "leftcontrol" | "ctrl" => Ok(Self::LeftCtrl),
            "super" | "mod" | "lwin" | "leftwin" | "win" => Ok(Self::LeftMeta),
            "lalt" | "leftalt" | "alt" => Ok(Self::LeftAlt),
            "space" => Ok(Self::Space),
            "ralt" | "rightalt" | "altgr" => Ok(Self::RightAlt),
            "context" => Ok(Self::ContextMenu),
            "rctrl" | "rightctrl" | "rightcontrol" => Ok(Self::RightCtrl),

            // Middle part
            "print" => Ok(Self::Print),
            "scroll" => Ok(Self::ScrollLock),
            "pause" => Ok(Self::Pause),
            "insert" | "ins" => Ok(Self::Insert),
            "home" => Ok(Self::Home),
            "pageup" | "pgup" => Ok(Self::PageUp),
            "delete" | "del" => Ok(Self::Delete),
            "end" => Ok(Self::End),
            "pagedown" | "pgdown" => Ok(Self::PageDown),
            "left" => Ok(Self::Left),
            "right" => Ok(Self::Right),
            "up" => Ok(Self::Up),
            "down" => Ok(Self::Down),

            // Numpad
            "num0" => Ok(Self::Numpad0),
            "num1" => Ok(Self::Numpad1),
            "num2" => Ok(Self::Numpad2),
            "num3" => Ok(Self::Numpad3),
            "num4" => Ok(Self::Numpad4),
            "num5" => Ok(Self::Numpad5),
            "num6" => Ok(Self::Numpad6),
            "num7" => Ok(Self::Numpad7),
            "num8" => Ok(Self::Numpad8),
            "num9" => Ok(Self::Numpad9),

            _ => Err(s.to_owned()),
        }
    }
}

#[cfg(target_os = "linux")]
impl TryFrom<Key> for evdev::Key {
    type Error = String;

    fn try_from(value: Key) -> Result<Self, Self::Error> {
        match value {
            Key::A => Ok(evdev::Key::KEY_A),
            Key::B => Ok(evdev::Key::KEY_B),
            Key::C => Ok(evdev::Key::KEY_C),
            Key::D => Ok(evdev::Key::KEY_D),
            Key::E => Ok(evdev::Key::KEY_E),
            Key::F => Ok(evdev::Key::KEY_F),
            Key::G => Ok(evdev::Key::KEY_G),
            Key::H => Ok(evdev::Key::KEY_H),
            Key::I => Ok(evdev::Key::KEY_I),
            Key::J => Ok(evdev::Key::KEY_J),
            Key::K => Ok(evdev::Key::KEY_K),
            Key::L => Ok(evdev::Key::KEY_L),
            Key::M => Ok(evdev::Key::KEY_M),
            Key::N => Ok(evdev::Key::KEY_N),
            Key::O => Ok(evdev::Key::KEY_O),
            Key::P => Ok(evdev::Key::KEY_P),
            Key::Q => Ok(evdev::Key::KEY_Q),
            Key::R => Ok(evdev::Key::KEY_R),
            Key::S => Ok(evdev::Key::KEY_S),
            Key::T => Ok(evdev::Key::KEY_T),
            Key::U => Ok(evdev::Key::KEY_U),
            Key::V => Ok(evdev::Key::KEY_V),
            Key::W => Ok(evdev::Key::KEY_W),
            Key::X => Ok(evdev::Key::KEY_X),
            Key::Y => Ok(evdev::Key::KEY_Y),
            Key::Z => Ok(evdev::Key::KEY_Z),
            Key::Backslash => Ok(evdev::Key::KEY_BACKSLASH),
            Key::LeftBrace => Ok(evdev::Key::KEY_LEFTBRACE),
            Key::RightBrace => Ok(evdev::Key::KEY_RIGHTBRACE),
            Key::Print => Ok(evdev::Key::KEY_PRINT),
            Key::ScrollLock => Ok(evdev::Key::KEY_SCROLLLOCK),
            Key::Pause => Ok(evdev::Key::KEY_PAUSE),
            Key::Num0 => Ok(evdev::Key::KEY_0),
            Key::Num1 => Ok(evdev::Key::KEY_1),
            Key::Num2 => Ok(evdev::Key::KEY_1),
            Key::Num3 => Ok(evdev::Key::KEY_3),
            Key::Num4 => Ok(evdev::Key::KEY_4),
            Key::Num5 => Ok(evdev::Key::KEY_5),
            Key::Num6 => Ok(evdev::Key::KEY_6),
            Key::Num7 => Ok(evdev::Key::KEY_7),
            Key::Num8 => Ok(evdev::Key::KEY_8),
            Key::Num9 => Ok(evdev::Key::KEY_9),
            Key::Minus => Ok(evdev::Key::KEY_MINUS),
            Key::Equal => Ok(evdev::Key::KEY_EQUAL),
            Key::Backspace => Ok(evdev::Key::KEY_BACKSPACE),
            Key::Tilde => Ok(evdev::Key::KEY_GRAVE),
            Key::Enter => Ok(evdev::Key::KEY_ENTER),
            Key::Tab => Ok(evdev::Key::KEY_TAB),
            Key::Esc => Ok(evdev::Key::KEY_ESC),
            Key::CapsLock => Ok(evdev::Key::KEY_CAPSLOCK),
            Key::LeftShift => Ok(evdev::Key::KEY_LEFTSHIFT),
            Key::LeftCtrl => Ok(evdev::Key::KEY_LEFTCTRL),
            Key::LeftAlt => Ok(evdev::Key::KEY_LEFTALT),
            Key::LeftMeta => Ok(evdev::Key::KEY_LEFTMETA),
            Key::Space => Ok(evdev::Key::KEY_SPACE),
            Key::RightAlt => Ok(evdev::Key::KEY_RIGHTALT),
            Key::ContextMenu => Ok(evdev::Key::KEY_CONTEXT_MENU),
            Key::RightCtrl => Ok(evdev::Key::KEY_RIGHTCTRL),
            Key::RightShift => Ok(evdev::Key::KEY_RIGHTSHIFT),
            Key::Comma => Ok(evdev::Key::KEY_COMMA),
            Key::Dot => Ok(evdev::Key::KEY_DOT),
            Key::Slash => Ok(evdev::Key::KEY_SLASH),
            Key::Semicolon => Ok(evdev::Key::KEY_SEMICOLON),
            Key::Quote => Ok(evdev::Key::KEY_APOSTROPHE),
            Key::F1 => Ok(evdev::Key::KEY_F1),
            Key::F2 => Ok(evdev::Key::KEY_F2),
            Key::F3 => Ok(evdev::Key::KEY_F3),
            Key::F4 => Ok(evdev::Key::KEY_F4),
            Key::F5 => Ok(evdev::Key::KEY_F5),
            Key::F6 => Ok(evdev::Key::KEY_F6),
            Key::F7 => Ok(evdev::Key::KEY_F7),
            Key::F8 => Ok(evdev::Key::KEY_F8),
            Key::F9 => Ok(evdev::Key::KEY_F9),
            Key::F10 => Ok(evdev::Key::KEY_F10),
            Key::F11 => Ok(evdev::Key::KEY_F11),
            Key::F12 => Ok(evdev::Key::KEY_F12),
            Key::Left => Ok(evdev::Key::KEY_LEFT),
            Key::Right => Ok(evdev::Key::KEY_RIGHT),
            Key::Up => Ok(evdev::Key::KEY_UP),
            Key::Down => Ok(evdev::Key::KEY_DOWN),
            Key::Insert => Ok(evdev::Key::KEY_INSERT),
            Key::Delete => Ok(evdev::Key::KEY_DELETE),
            Key::Home => Ok(evdev::Key::KEY_HOME),
            Key::End => Ok(evdev::Key::KEY_END),
            Key::PageUp => Ok(evdev::Key::KEY_PAGEUP),
            Key::PageDown => Ok(evdev::Key::KEY_PAGEDOWN),
            Key::Numpad0 => Ok(evdev::Key::KEY_NUMERIC_0),
            Key::Numpad1 => Ok(evdev::Key::KEY_NUMERIC_1),
            Key::Numpad2 => Ok(evdev::Key::KEY_NUMERIC_2),
            Key::Numpad3 => Ok(evdev::Key::KEY_NUMERIC_3),
            Key::Numpad4 => Ok(evdev::Key::KEY_NUMERIC_4),
            Key::Numpad5 => Ok(evdev::Key::KEY_NUMERIC_5),
            Key::Numpad6 => Ok(evdev::Key::KEY_NUMERIC_6),
            Key::Numpad7 => Ok(evdev::Key::KEY_NUMERIC_7),
            Key::Numpad8 => Ok(evdev::Key::KEY_NUMERIC_8),
            Key::Numpad9 => Ok(evdev::Key::KEY_NUMERIC_9),
        }
    }
}

#[cfg(feature = "sdl2")]
impl TryFrom<sdl2::keyboard::Keycode> for Key {
    type Error = String;

    fn try_from(value: sdl2::keyboard::Keycode) -> Result<Self, Self::Error> {
        use sdl2::keyboard::Keycode;
        match value {
            // Group 1
            Keycode::A => Ok(Self::A),
            Keycode::B => Ok(Self::B),
            Keycode::C => Ok(Self::C),
            Keycode::D => Ok(Self::D),
            Keycode::E => Ok(Self::E),
            Keycode::F => Ok(Self::F),
            Keycode::G => Ok(Self::G),
            Keycode::H => Ok(Self::H),
            Keycode::I => Ok(Self::I),
            Keycode::J => Ok(Self::J),
            Keycode::K => Ok(Self::K),
            Keycode::L => Ok(Self::L),
            Keycode::M => Ok(Self::M),
            Keycode::N => Ok(Self::N),
            Keycode::O => Ok(Self::O),
            Keycode::P => Ok(Self::P),
            Keycode::Q => Ok(Self::Q),
            Keycode::R => Ok(Self::R),
            Keycode::S => Ok(Self::S),
            Keycode::T => Ok(Self::T),
            Keycode::U => Ok(Self::U),
            Keycode::V => Ok(Self::V),
            Keycode::W => Ok(Self::W),
            Keycode::X => Ok(Self::X),
            Keycode::Y => Ok(Self::Y),
            Keycode::Z => Ok(Self::Z),
            Keycode::Backslash => Ok(Self::Backslash),
            Keycode::LeftBracket => Ok(Self::LeftBrace),
            Keycode::RightBracket => Ok(Self::RightBrace),
            Keycode::PrintScreen => Ok(Self::Print),
            Keycode::ScrollLock => Ok(Self::ScrollLock),
            Keycode::Pause => Ok(Self::Pause),

            // Group 2
            Keycode::Num0 => Ok(Self::Num0),
            Keycode::Num1 => Ok(Self::Num1),
            Keycode::Num2 => Ok(Self::Num2),
            Keycode::Num3 => Ok(Self::Num3),
            Keycode::Num4 => Ok(Self::Num4),
            Keycode::Num5 => Ok(Self::Num5),
            Keycode::Num6 => Ok(Self::Num6),
            Keycode::Num7 => Ok(Self::Num7),
            Keycode::Num8 => Ok(Self::Num8),
            Keycode::Num9 => Ok(Self::Num9),
            Keycode::Minus => Ok(Self::Minus),
            Keycode::Equals => Ok(Self::Equal),
            Keycode::Backspace => Ok(Self::Backspace),
            Keycode::Backquote => Ok(Self::Tilde),
            Keycode::Return => Ok(Self::Enter),
            Keycode::Tab => Ok(Self::Tab),
            Keycode::Escape => Ok(Self::Esc),
            Keycode::CapsLock => Ok(Self::CapsLock),
            Keycode::LShift => Ok(Self::LeftShift),
            Keycode::LCtrl => Ok(Self::LeftCtrl),
            Keycode::LAlt => Ok(Self::LeftAlt),
            Keycode::LGui => Ok(Self::LeftMeta),
            Keycode::Space => Ok(Self::Space),
            Keycode::RAlt => Ok(Self::RightAlt),
            Keycode::Application => Ok(Self::ContextMenu),
            Keycode::RCtrl => Ok(Self::RightCtrl),
            Keycode::RShift => Ok(Self::RightShift),
            Keycode::Comma => Ok(Self::Comma),
            Keycode::Period => Ok(Self::Dot),
            Keycode::Slash => Ok(Self::Slash),
            Keycode::Semicolon => Ok(Self::Semicolon),
            Keycode::Quote => Ok(Self::Quote),

            // Group 3
            Keycode::F1 => Ok(Self::F1),
            Keycode::F2 => Ok(Self::F2),
            Keycode::F3 => Ok(Self::F3),
            Keycode::F4 => Ok(Self::F4),
            Keycode::F5 => Ok(Self::F5),
            Keycode::F6 => Ok(Self::F6),
            Keycode::F7 => Ok(Self::F7),
            Keycode::F8 => Ok(Self::F8),
            Keycode::F9 => Ok(Self::F9),
            Keycode::F10 => Ok(Self::F10),
            Keycode::F11 => Ok(Self::F11),
            Keycode::F12 => Ok(Self::F12),
            Keycode::Left => Ok(Self::Left),
            Keycode::Right => Ok(Self::Right),
            Keycode::Up => Ok(Self::Up),
            Keycode::Down => Ok(Self::Down),
            Keycode::Insert => Ok(Self::Insert),
            Keycode::Delete => Ok(Self::Delete),
            Keycode::Home => Ok(Self::Home),
            Keycode::End => Ok(Self::End),
            Keycode::PageUp => Ok(Self::PageUp),
            Keycode::PageDown => Ok(Self::PageDown),
            Keycode::Kp0 => Ok(Self::Num0),
            Keycode::Kp1 => Ok(Self::Num1),
            Keycode::Kp2 => Ok(Self::Num2),
            Keycode::Kp3 => Ok(Self::Num3),
            Keycode::Kp4 => Ok(Self::Num4),
            Keycode::Kp5 => Ok(Self::Num5),
            Keycode::Kp6 => Ok(Self::Num6),
            Keycode::Kp7 => Ok(Self::Num7),
            Keycode::Kp8 => Ok(Self::Num8),
            Keycode::Kp9 => Ok(Self::Num9),

            other => Err(other.to_string()),
        }
    }
}

#[cfg(target_os = "windows")]
impl TryFrom<Key> for i32 {
    type Error = String;

    fn try_from(value: Key) -> Result<Self, Self::Error> {
        // TODO: Implement the rest.
        use winapi::um::winuser::*;
        match value {
            Key::A => Ok(0x41),
            Key::B => Ok(0x42),
            Key::C => Ok(0x43),
            Key::D => Ok(0x44),
            Key::E => Ok(0x45),
            Key::F => Ok(0x46),
            Key::G => Ok(0x47),
            Key::H => Ok(0x48),
            Key::I => Ok(0x49),
            Key::J => Ok(0x4a),
            Key::K => Ok(0x4b),
            Key::L => Ok(0x4c),
            Key::M => Ok(0x4d),
            Key::N => Ok(0x4e),
            Key::O => Ok(0x4f),
            Key::P => Ok(0x50),
            Key::Q => Ok(0x51),
            Key::R => Ok(0x52),
            Key::S => Ok(0x53),
            Key::T => Ok(0x54),
            Key::U => Ok(0x55),
            Key::V => Ok(0x56),
            Key::W => Ok(0x57),
            Key::X => Ok(0x58),
            Key::Y => Ok(0x59),
            Key::Z => Ok(0x5a),
            Key::Backslash => Ok(VK_OEM_5),
            Key::LeftBrace => Ok(VK_OEM_4),
            Key::RightBrace => Ok(VK_OEM_6),
            // Key::Print => Ok(),
            // Key::ScrollLock => Ok(),
            // Key::Pause => Ok(),
            Key::Num0 => Ok(0x30),
            Key::Num1 => Ok(0x31),
            Key::Num2 => Ok(0x32),
            Key::Num3 => Ok(0x33),
            Key::Num4 => Ok(0x34),
            Key::Num5 => Ok(0x35),
            Key::Num6 => Ok(0x36),
            Key::Num7 => Ok(0x37),
            Key::Num8 => Ok(0x38),
            Key::Num9 => Ok(0x39),
            Key::Minus => Ok(VK_OEM_MINUS),
            Key::Equal => Ok(VK_OEM_PLUS),
            // Key::Backspace => Ok(),
            // Key::Tilde => Ok(),
            // Key::Enter => Ok(),
            // Key::Tab => Ok(),
            // Key::Esc => Ok(),
            // Key::CapsLock => Ok(),
            // Key::LeftShift => Ok(),
            // Key::LeftCtrl => Ok(),
            // Key::LeftAlt => Ok(),
            // Key::LeftMeta => Ok(),
            // Key::Space => Ok(),
            // Key::RightAlt => Ok(),
            // Key::ContextMenu => Ok(),
            // Key::RightCtrl => Ok(),
            // Key::RightShift => Ok(),
            Key::Comma => Ok(VK_OEM_COMMA),
            Key::Dot => Ok(VK_OEM_PERIOD),
            Key::Slash => Ok(VK_OEM_2),
            Key::Semicolon => Ok(VK_OEM_1),
            Key::Quote => Ok(VK_OEM_7),
            // Key::F1 => Ok(),
            // Key::F2 => Ok(),
            // Key::F3 => Ok(),
            // Key::F4 => Ok(),
            // Key::F5 => Ok(),
            // Key::F6 => Ok(),
            // Key::F7 => Ok(),
            // Key::F8 => Ok(),
            // Key::F9 => Ok(),
            // Key::F10 => Ok(),
            // Key::F11 => Ok(),
            // Key::F12 => Ok(),
            // Key::Left => Ok(),
            // Key::Right => Ok(),
            // Key::Up => Ok(),
            // Key::Down => Ok(),
            // Key::Insert => Ok(),
            // Key::Delete => Ok(),
            // Key::Home => Ok(),
            // Key::End => Ok(),
            // Key::PageUp => Ok(),
            // Key::PageDown => Ok(),
            // Key::Numpad0 => Ok(),
            // Key::Numpad1 => Ok(),
            // Key::Numpad2 => Ok(),
            // Key::Numpad3 => Ok(),
            // Key::Numpad4 => Ok(),
            // Key::Numpad5 => Ok(),
            // Key::Numpad6 => Ok(),
            // Key::Numpad7 => Ok(),
            // Key::Numpad8 => Ok(),
            // Key::Numpad9 => Ok(),
            other => Err(other.to_string()),
        }
    }
}
