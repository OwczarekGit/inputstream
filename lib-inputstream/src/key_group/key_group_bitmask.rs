#[derive(Debug, Clone)]
pub struct KeyGroupBitmask(u32, u8);

impl KeyGroupBitmask {
    pub fn get(&self) -> (u32, u8) {
        (self.0, self.1)
    }
}

#[cfg(feature = "sdl2")]
impl TryFrom<sdl2::keyboard::Keycode> for KeyGroupBitmask {
    type Error = ();
    fn try_from(v: sdl2::keyboard::Keycode) -> Result<Self, Self::Error> {
        use sdl2::keyboard::Keycode;
        match v {
            // Group 1
            Keycode::A => Ok(Self(0, 1)),
            Keycode::B => Ok(Self(1, 1)),
            Keycode::C => Ok(Self(2, 1)),
            Keycode::D => Ok(Self(3, 1)),
            Keycode::E => Ok(Self(4, 1)),
            Keycode::F => Ok(Self(5, 1)),
            Keycode::G => Ok(Self(6, 1)),
            Keycode::H => Ok(Self(7, 1)),
            Keycode::I => Ok(Self(8, 1)),
            Keycode::J => Ok(Self(9, 1)),
            Keycode::K => Ok(Self(10, 1)),
            Keycode::L => Ok(Self(11, 1)),
            Keycode::M => Ok(Self(12, 1)),
            Keycode::N => Ok(Self(13, 1)),
            Keycode::O => Ok(Self(14, 1)),
            Keycode::P => Ok(Self(15, 1)),
            Keycode::Q => Ok(Self(16, 1)),
            Keycode::R => Ok(Self(17, 1)),
            Keycode::S => Ok(Self(18, 1)),
            Keycode::T => Ok(Self(19, 1)),
            Keycode::U => Ok(Self(20, 1)),
            Keycode::V => Ok(Self(21, 1)),
            Keycode::W => Ok(Self(22, 1)),
            Keycode::X => Ok(Self(23, 1)),
            Keycode::Y => Ok(Self(24, 1)),
            Keycode::Z => Ok(Self(25, 1)),
            Keycode::Backslash => Ok(Self(26, 1)),
            Keycode::LeftBracket => Ok(Self(27, 1)),
            Keycode::RightBracket => Ok(Self(28, 1)),
            Keycode::PrintScreen => Ok(Self(29, 1)),
            Keycode::ScrollLock => Ok(Self(30, 1)),
            Keycode::Pause => Ok(Self(31, 1)),

            // Group 2
            Keycode::Num0 => Ok(Self(0, 2)),
            Keycode::Num1 => Ok(Self(1, 2)),
            Keycode::Num2 => Ok(Self(2, 2)),
            Keycode::Num3 => Ok(Self(3, 2)),
            Keycode::Num4 => Ok(Self(4, 2)),
            Keycode::Num5 => Ok(Self(5, 2)),
            Keycode::Num6 => Ok(Self(6, 2)),
            Keycode::Num7 => Ok(Self(7, 2)),
            Keycode::Num8 => Ok(Self(8, 2)),
            Keycode::Num9 => Ok(Self(9, 2)),
            Keycode::Minus => Ok(Self(10, 2)),
            Keycode::Equals => Ok(Self(11, 2)),
            Keycode::Backspace => Ok(Self(12, 2)),
            Keycode::Backquote => Ok(Self(13, 2)),
            Keycode::Return => Ok(Self(14, 2)),
            Keycode::Tab => Ok(Self(15, 2)),
            Keycode::Escape => Ok(Self(16, 2)),
            Keycode::CapsLock => Ok(Self(17, 2)),
            Keycode::LShift => Ok(Self(18, 2)),
            Keycode::LCtrl => Ok(Self(19, 2)),
            Keycode::LAlt => Ok(Self(20, 2)),
            Keycode::LGui => Ok(Self(21, 2)),
            Keycode::Space => Ok(Self(22, 2)),
            Keycode::RAlt => Ok(Self(23, 2)),
            Keycode::Application => Ok(Self(24, 2)),
            Keycode::RCtrl => Ok(Self(25, 2)),
            Keycode::RShift => Ok(Self(26, 2)),
            Keycode::Comma => Ok(Self(27, 2)),
            Keycode::Period => Ok(Self(28, 2)),
            Keycode::Slash => Ok(Self(29, 2)),
            Keycode::Semicolon => Ok(Self(30, 2)),
            Keycode::Quote => Ok(Self(31, 2)),

            // Group 3
            Keycode::F1 => Ok(Self(0, 3)),
            Keycode::F2 => Ok(Self(1, 3)),
            Keycode::F3 => Ok(Self(2, 3)),
            Keycode::F4 => Ok(Self(3, 3)),
            Keycode::F5 => Ok(Self(4, 3)),
            Keycode::F6 => Ok(Self(5, 3)),
            Keycode::F7 => Ok(Self(6, 3)),
            Keycode::F8 => Ok(Self(7, 3)),
            Keycode::F9 => Ok(Self(8, 3)),
            Keycode::F10 => Ok(Self(9, 3)),
            Keycode::F11 => Ok(Self(10, 3)),
            Keycode::F12 => Ok(Self(11, 3)),
            Keycode::Left => Ok(Self(12, 3)),
            Keycode::Right => Ok(Self(13, 3)),
            Keycode::Up => Ok(Self(14, 3)),
            Keycode::Down => Ok(Self(15, 3)),
            Keycode::Insert => Ok(Self(16, 3)),
            Keycode::Delete => Ok(Self(17, 3)),
            Keycode::Home => Ok(Self(18, 3)),
            Keycode::End => Ok(Self(19, 3)),
            Keycode::PageUp => Ok(Self(20, 3)),
            Keycode::PageDown => Ok(Self(21, 3)),
            Keycode::Kp0 => Ok(Self(22, 3)),
            Keycode::Kp1 => Ok(Self(23, 3)),
            Keycode::Kp2 => Ok(Self(24, 3)),
            Keycode::Kp3 => Ok(Self(25, 3)),
            Keycode::Kp4 => Ok(Self(26, 3)),
            Keycode::Kp5 => Ok(Self(27, 3)),
            Keycode::Kp6 => Ok(Self(28, 3)),
            Keycode::Kp7 => Ok(Self(29, 3)),
            Keycode::Kp8 => Ok(Self(30, 3)),
            Keycode::Kp9 => Ok(Self(31, 3)),

            _ => Err(()),
        }
    }
}
