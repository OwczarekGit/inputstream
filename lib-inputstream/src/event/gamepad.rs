use std::{
    fmt::Display,
    str::{FromStr, Split},
};

use super::difference::Difference;

// NOTE: We target Dualsense controller, so probably
//       could use u8 for sticks and triggers since
//       thats what it uses.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct GamepadEvent {
    pub gamepad_id: u32,
    pub left_stick: (f32, f32),
    pub right_stick: (f32, f32),
    pub triggers: (f32, f32),
    pub buttons: u32,
}

impl GamepadEvent {
    pub fn set_button_state(&mut self, button: impl Into<GamepadButton>, state: bool) {
        let bit = button.into() as u32;
        if state {
            self.buttons |= bit;
        } else {
            self.buttons &= !bit;
        }
    }
}

impl Default for GamepadEvent {
    fn default() -> Self {
        Self {
            left_stick: (127.0, 127.0),
            right_stick: (127.0, 127.0),
            buttons: 0,
            gamepad_id: 0,
            triggers: (0.0, 0.0),
        }
    }
}

impl Display for GamepadEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{};{};{};{};{};{};{};{}",
            self.gamepad_id,
            self.left_stick.0,
            self.left_stick.1,
            self.right_stick.0,
            self.right_stick.1,
            self.triggers.0,
            self.triggers.1,
            self.buttons
        )
    }
}

impl FromStr for GamepadEvent {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(';');

        let next_float = |split: &mut Split<'_, _>, name: &str| {
            Ok::<f32, String>(
                split
                    .next()
                    .ok_or(format!("Missing {name}."))?
                    .trim()
                    .parse()
                    .unwrap_or(0.0),
            )
        };

        let gamepad_id: u32 = split
            .next()
            .ok_or("Missing gamepad id".to_string())?
            .trim()
            .parse()
            .map_err(|_| "Invalid gamepad id")?;

        let left_stick = (
            next_float(&mut split, "left stick x")?,
            next_float(&mut split, "left stick y")?,
        );

        let right_stick = (
            next_float(&mut split, "right stick x")?,
            next_float(&mut split, "right stick y")?,
        );

        let triggers = (
            next_float(&mut split, "trigger left")?,
            next_float(&mut split, "trigger right")?,
        );

        let buttons = split
            .next()
            .ok_or("Missing gamepad buttons")?
            .parse()
            .unwrap_or(0);

        Ok(Self {
            gamepad_id,
            left_stick,
            right_stick,
            triggers,
            buttons,
        })
    }
}

impl Difference for GamepadEvent {
    type Output = (
        (Option<f32>, Option<f32>),
        (Option<f32>, Option<f32>),
        (Option<f32>, Option<f32>),
        Vec<(bool, GamepadButton)>,
    );

    fn get_diff(&self, other: &Self) -> Self::Output {
        let motion_diff = |val: f32, new: f32| val.ne(&new).then_some(new);

        let add_button_change = |v: &mut Vec<(bool, GamepadButton)>, btn: GamepadButton| {
            if (self.buttons & btn as u32) != (other.buttons & btn as u32) {
                v.push((other.buttons & btn as u32 > 0, btn));
            }
        };

        let mut buttons = vec![];

        add_button_change(&mut buttons, GamepadButton::X);
        add_button_change(&mut buttons, GamepadButton::Circle);
        add_button_change(&mut buttons, GamepadButton::Triangle);
        add_button_change(&mut buttons, GamepadButton::Square);
        add_button_change(&mut buttons, GamepadButton::Down);
        add_button_change(&mut buttons, GamepadButton::Up);
        add_button_change(&mut buttons, GamepadButton::Left);
        add_button_change(&mut buttons, GamepadButton::Right);
        add_button_change(&mut buttons, GamepadButton::Select);
        add_button_change(&mut buttons, GamepadButton::Start);
        add_button_change(&mut buttons, GamepadButton::Mode);
        add_button_change(&mut buttons, GamepadButton::BumperLeft);
        add_button_change(&mut buttons, GamepadButton::BumperRight);
        add_button_change(&mut buttons, GamepadButton::StickLeft);
        add_button_change(&mut buttons, GamepadButton::StickRight);

        let diff_left = (
            motion_diff(self.left_stick.0, other.left_stick.0),
            motion_diff(self.left_stick.1, other.left_stick.1),
        );

        let diff_right = (
            motion_diff(self.right_stick.0, other.right_stick.0),
            motion_diff(self.right_stick.1, other.right_stick.1),
        );

        let diff_trigger = (
            motion_diff(self.triggers.0, other.triggers.0),
            motion_diff(self.triggers.1, other.triggers.1),
        );

        (diff_left, diff_right, diff_trigger, buttons)
    }
}

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
