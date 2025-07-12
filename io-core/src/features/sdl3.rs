use crate::builtin::messages::{
    gamepad::gamepad_button::GamepadButton, keyboard::key::Key, mouse::button::Button,
};
use sdl3::{keyboard::Keycode, mouse::MouseButton};

pub fn map_sdl3_trigger_to_gamepad(value: i16) -> u8 {
    (value as f32 * 255.0 / 32767.0).round() as u8
}

pub fn map_sdl3_axis_to_gamepad(value: i16) -> i8 {
    let value = value as i32;
    const FROM_MIN: i32 = i16::MIN as i32;
    const FROM_MAX: i32 = i16::MAX as i32;
    const TO_MIN: i32 = i8::MIN as i32;
    const TO_MAX: i32 = i8::MAX as i32;
    (TO_MIN + ((value - FROM_MIN) * (TO_MAX - TO_MIN) / (FROM_MAX - FROM_MIN)))
        .clamp(TO_MIN, TO_MAX) as i8
}

impl From<MouseButton> for Button {
    fn from(value: MouseButton) -> Self {
        match value {
            MouseButton::Left => Button::Left,
            MouseButton::Middle => Button::Middle,
            MouseButton::Right => Button::Right,
            MouseButton::X1 => Button::Mouse4,
            MouseButton::X2 => Button::Mouse5,
            k => unimplemented!("Unimplemented mapping: {:?}", k),
        }
    }
}

impl From<Keycode> for Key {
    fn from(value: Keycode) -> Self {
        match value {
            Keycode::_0 => Key::K0,
            Keycode::_1 => Key::K1,
            Keycode::_2 => Key::K2,
            Keycode::_3 => Key::K3,
            Keycode::_4 => Key::K4,
            Keycode::_5 => Key::K5,
            Keycode::_6 => Key::K6,
            Keycode::_7 => Key::K7,
            Keycode::_8 => Key::K8,
            Keycode::_9 => Key::K9,
            Keycode::A => Key::A,
            Keycode::B => Key::B,
            Keycode::C => Key::C,
            Keycode::D => Key::D,
            Keycode::E => Key::E,
            Keycode::F => Key::F,
            Keycode::G => Key::G,
            Keycode::H => Key::H,
            Keycode::I => Key::I,
            Keycode::J => Key::J,
            Keycode::K => Key::K,
            Keycode::L => Key::L,
            Keycode::M => Key::M,
            Keycode::N => Key::N,
            Keycode::O => Key::O,
            Keycode::P => Key::P,
            Keycode::Q => Key::Q,
            Keycode::R => Key::R,
            Keycode::S => Key::S,
            Keycode::T => Key::T,
            Keycode::U => Key::U,
            Keycode::V => Key::V,
            Keycode::W => Key::W,
            Keycode::X => Key::X,
            Keycode::Y => Key::Y,
            Keycode::Z => Key::Z,
            Keycode::F1 => Key::F1,
            Keycode::F2 => Key::F2,
            Keycode::F3 => Key::F3,
            Keycode::F4 => Key::F4,
            Keycode::F5 => Key::F5,
            Keycode::F6 => Key::F6,
            Keycode::F7 => Key::F7,
            Keycode::F8 => Key::F8,
            Keycode::F9 => Key::F9,
            Keycode::F10 => Key::F10,
            Keycode::F11 => Key::F11,
            Keycode::F12 => Key::F12,
            Keycode::Return => Key::Enter,
            Keycode::Escape => Key::Escape,
            Keycode::Backspace => Key::Backspace,
            Keycode::Tab => Key::Tab,
            Keycode::Space => Key::Space,
            Keycode::Comma => Key::Comma,
            Keycode::Minus => Key::Minus,
            Keycode::Period => Key::Dot,
            Keycode::Slash => Key::Slash,
            Keycode::Semicolon => Key::Semicolon,
            Keycode::Equals => Key::Equals,
            Keycode::Grave => Key::Tilde,
            Keycode::Backslash => Key::Backslash,
            Keycode::LeftBracket => Key::BracketLeft,
            Keycode::RightBracket => Key::BracketRight,
            Keycode::Delete => Key::Delete,
            Keycode::CapsLock => Key::CapsLock,
            Keycode::PrintScreen => Key::PrintScreen,
            Keycode::ScrollLock => Key::ScrollLock,
            Keycode::Pause => Key::Pause,
            Keycode::Insert => Key::Insert,
            Keycode::Home => Key::Home,
            Keycode::PageUp => Key::PageUp,
            Keycode::End => Key::End,
            Keycode::PageDown => Key::PageDown,
            Keycode::Right => Key::Right,
            Keycode::Left => Key::Left,
            Keycode::Down => Key::Down,
            Keycode::Up => Key::Up,
            Keycode::Application => Key::ContextMenu,
            Keycode::LCtrl => Key::LeftCtrl,
            Keycode::LShift => Key::LeftShift,
            Keycode::LAlt => Key::LeftAlt,
            Keycode::LGui => Key::LeftSuper,
            Keycode::RCtrl => Key::RightCtrl,
            Keycode::RShift => Key::RightShift,
            Keycode::RAlt => Key::RightAlt,
            Keycode::RGui => Key::RightSuper,
            Keycode::Mode => Key::RightAlt,
            Keycode::Apostrophe => Key::Quote,
            k => unimplemented!("Unimplemented mapping: {}", k),
        }
    }
}

impl From<sdl3::gamepad::Button> for GamepadButton {
    fn from(value: sdl3::gamepad::Button) -> Self {
        match value {
            sdl3::gamepad::Button::North => GamepadButton::Triangle,
            sdl3::gamepad::Button::East => GamepadButton::Circle,
            sdl3::gamepad::Button::South => GamepadButton::X,
            sdl3::gamepad::Button::West => GamepadButton::Square,
            sdl3::gamepad::Button::Start => GamepadButton::Options,
            sdl3::gamepad::Button::Back => GamepadButton::Share,
            sdl3::gamepad::Button::LeftStick => GamepadButton::LeftStick,
            sdl3::gamepad::Button::RightStick => GamepadButton::RightStick,
            sdl3::gamepad::Button::LeftShoulder => GamepadButton::LeftBumper,
            sdl3::gamepad::Button::RightShoulder => GamepadButton::RightBumper,
            sdl3::gamepad::Button::DPadUp => GamepadButton::Up,
            sdl3::gamepad::Button::DPadDown => GamepadButton::Down,
            sdl3::gamepad::Button::DPadLeft => GamepadButton::Left,
            sdl3::gamepad::Button::DPadRight => GamepadButton::Right,
            sdl3::gamepad::Button::Guide => GamepadButton::Logo,
            sdl3::gamepad::Button::Misc1 => GamepadButton::Mute,
            other => unimplemented!("Unimplemented mapping: {:?}", other),
        }
    }
}
