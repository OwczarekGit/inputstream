use crate::event::keyboard::{Group1Event, Group2Event, Group3Event};

use super::KeyboardEventGroup;

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
