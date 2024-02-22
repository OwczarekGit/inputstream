use std::ops::Deref;

#[derive(Debug, Clone)]
pub struct KeyGroupBitmask(u32);

impl Deref for KeyGroupBitmask {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TryFrom<sdl2::keyboard::Keycode> for KeyGroupBitmask {
    type Error = ();
    fn try_from(v: sdl2::keyboard::Keycode) -> Result<Self, Self::Error> {
        use sdl2::keyboard::Keycode;
        match v {
            // Group 1
            Keycode::A => Ok(Self(0)),
            Keycode::B => Ok(Self(1)),
            Keycode::C => Ok(Self(2)),
            Keycode::D => Ok(Self(3)),
            Keycode::E => Ok(Self(4)),
            Keycode::F => Ok(Self(5)),
            Keycode::G => Ok(Self(6)),
            Keycode::H => Ok(Self(7)),
            Keycode::I => Ok(Self(8)),
            Keycode::J => Ok(Self(9)),
            Keycode::K => Ok(Self(10)),
            Keycode::L => Ok(Self(11)),
            Keycode::M => Ok(Self(12)),
            Keycode::N => Ok(Self(13)),
            Keycode::O => Ok(Self(14)),
            Keycode::P => Ok(Self(15)),
            Keycode::Q => Ok(Self(16)),
            Keycode::R => Ok(Self(17)),
            Keycode::S => Ok(Self(18)),
            Keycode::T => Ok(Self(19)),
            Keycode::U => Ok(Self(20)),
            Keycode::V => Ok(Self(21)),
            Keycode::W => Ok(Self(22)),
            Keycode::X => Ok(Self(23)),
            Keycode::Y => Ok(Self(24)),
            Keycode::Z => Ok(Self(25)),
            Keycode::Backslash => Ok(Self(26)),
            Keycode::LeftBracket => Ok(Self(27)),
            Keycode::RightBracket => Ok(Self(28)),
            Keycode::PrintScreen => Ok(Self(29)),
            Keycode::ScrollLock => Ok(Self(30)),
            Keycode::Pause => Ok(Self(31)),

            // Group 2
            Keycode::Num0 => Ok(Self(0)),
            Keycode::Num1 => Ok(Self(1)),
            Keycode::Num2 => Ok(Self(2)),
            Keycode::Num3 => Ok(Self(3)),
            Keycode::Num4 => Ok(Self(4)),
            Keycode::Num5 => Ok(Self(5)),
            Keycode::Num6 => Ok(Self(6)),
            Keycode::Num7 => Ok(Self(7)),
            Keycode::Num8 => Ok(Self(8)),
            Keycode::Num9 => Ok(Self(9)),
            Keycode::Minus => Ok(Self(10)),
            Keycode::Equals => Ok(Self(11)),
            Keycode::Backspace => Ok(Self(12)),
            Keycode::Backquote => Ok(Self(13)),
            Keycode::Return => Ok(Self(14)),
            Keycode::Tab => Ok(Self(15)),
            Keycode::Escape => Ok(Self(16)),
            Keycode::CapsLock => Ok(Self(17)),
            Keycode::LShift => Ok(Self(18)),
            Keycode::LCtrl => Ok(Self(19)),
            Keycode::LAlt => Ok(Self(20)),
            Keycode::LGui => Ok(Self(21)),
            Keycode::Space => Ok(Self(22)),
            Keycode::RAlt => Ok(Self(23)),
            Keycode::Application => Ok(Self(24)),
            Keycode::RCtrl => Ok(Self(25)),
            Keycode::RShift => Ok(Self(26)),
            Keycode::Comma => Ok(Self(27)),
            Keycode::Period => Ok(Self(28)),
            Keycode::Slash => Ok(Self(29)),
            Keycode::Semicolon => Ok(Self(30)),
            Keycode::Quote => Ok(Self(31)),

            // Group 3
            Keycode::F1 => Ok(Self(0)),
            Keycode::F2 => Ok(Self(1)),
            Keycode::F3 => Ok(Self(2)),
            Keycode::F4 => Ok(Self(3)),
            Keycode::F5 => Ok(Self(4)),
            Keycode::F6 => Ok(Self(5)),
            Keycode::F7 => Ok(Self(6)),
            Keycode::F8 => Ok(Self(7)),
            Keycode::F9 => Ok(Self(8)),
            Keycode::F10 => Ok(Self(9)),
            Keycode::F11 => Ok(Self(10)),
            Keycode::F12 => Ok(Self(11)),
            Keycode::Left => Ok(Self(12)),
            Keycode::Right => Ok(Self(13)),
            Keycode::Up => Ok(Self(14)),
            Keycode::Down => Ok(Self(15)),
            Keycode::Insert => Ok(Self(16)),
            Keycode::Delete => Ok(Self(17)),
            Keycode::Home => Ok(Self(18)),
            Keycode::End => Ok(Self(19)),
            Keycode::PageUp => Ok(Self(20)),
            Keycode::PageDown => Ok(Self(21)),
            Keycode::Kp0 => Ok(Self(22)),
            Keycode::Kp1 => Ok(Self(23)),
            Keycode::Kp2 => Ok(Self(24)),
            Keycode::Kp3 => Ok(Self(25)),
            Keycode::Kp4 => Ok(Self(26)),
            Keycode::Kp5 => Ok(Self(27)),
            Keycode::Kp6 => Ok(Self(28)),
            Keycode::Kp7 => Ok(Self(29)),
            Keycode::Kp8 => Ok(Self(30)),
            Keycode::Kp9 => Ok(Self(31)),

            _ => Err(()),
        }
    }
}
