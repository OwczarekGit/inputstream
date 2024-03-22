use super::{Group1Event, Group2Event, Group3Event, Group4Event, KeyboardEventGroup};

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
