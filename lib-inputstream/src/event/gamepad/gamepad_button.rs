/// NOTE: Dualsense also has touchpad and microphone mute buttons.
///       Touchpad seems to be its own separate device on real hardware,
///       but we maybe could add it here.
///       Microphone mute is a weird one. It does not seem to be
///       showing in any of the devices in `evtest`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum GamepadButton {
    X = 1 << 0,
    Circle = 1 << 1,
    Triangle = 1 << 2,
    Square = 1 << 3,
    Down = 1 << 4,
    Right = 1 << 5,
    Up = 1 << 6,
    Left = 1 << 7,
    Start = 1 << 8,
    Select = 1 << 9,
    Mode = 1 << 10,
    BumperLeft = 1 << 11,
    BumperRight = 1 << 12,
    StickLeft = 1 << 13,
    StickRight = 1 << 14,
}

/// Theese are different for different controllers.
/// X360 sends TRIGGER_HAPPY and HAT events for DPAD.
/// Dualsense only sends HAT.
/// We will just map them like that, and handle
/// accordingly on the service side.
#[cfg(target_os = "linux")]
impl From<GamepadButton> for evdev::Key {
    fn from(value: GamepadButton) -> Self {
        use evdev::Key;
        match value {
            GamepadButton::X => Key::BTN_SOUTH,
            GamepadButton::Circle => Key::BTN_EAST,
            GamepadButton::Triangle => Key::BTN_NORTH,
            GamepadButton::Square => Key::BTN_WEST,
            GamepadButton::Start => Key::BTN_START,
            GamepadButton::Select => Key::BTN_SELECT,
            GamepadButton::Mode => Key::BTN_MODE,
            GamepadButton::BumperLeft => Key::BTN_TL,
            GamepadButton::BumperRight => Key::BTN_TR,
            GamepadButton::StickLeft => Key::BTN_THUMBL,
            GamepadButton::StickRight => Key::BTN_THUMBR,
            GamepadButton::Down => Key::BTN_TRIGGER_HAPPY4,
            GamepadButton::Right => Key::BTN_TRIGGER_HAPPY2,
            GamepadButton::Up => Key::BTN_TRIGGER_HAPPY3,
            GamepadButton::Left => Key::BTN_TRIGGER_HAPPY1,
        }
    }
}

#[cfg(feature = "sdl2")]
impl TryFrom<sdl2::controller::Button> for GamepadButton {
    type Error = ();

    fn try_from(value: sdl2::controller::Button) -> Result<Self, Self::Error> {
        match value {
            sdl2::controller::Button::A => Ok(GamepadButton::X),
            sdl2::controller::Button::B => Ok(GamepadButton::Circle),
            sdl2::controller::Button::X => Ok(GamepadButton::Square),
            sdl2::controller::Button::Y => Ok(GamepadButton::Triangle),
            sdl2::controller::Button::Guide => Ok(GamepadButton::Mode),
            sdl2::controller::Button::Start => Ok(GamepadButton::Start),
            sdl2::controller::Button::LeftStick => Ok(GamepadButton::StickLeft),
            sdl2::controller::Button::RightStick => Ok(GamepadButton::StickRight),
            sdl2::controller::Button::LeftShoulder => Ok(GamepadButton::BumperLeft),
            sdl2::controller::Button::RightShoulder => Ok(GamepadButton::BumperRight),
            sdl2::controller::Button::DPadUp => Ok(GamepadButton::Up),
            sdl2::controller::Button::DPadDown => Ok(GamepadButton::Down),
            sdl2::controller::Button::DPadLeft => Ok(GamepadButton::Left),
            sdl2::controller::Button::DPadRight => Ok(GamepadButton::Right),
            sdl2::controller::Button::Back => Ok(GamepadButton::Select),
            _ => Err(()),
        }
    }
}
