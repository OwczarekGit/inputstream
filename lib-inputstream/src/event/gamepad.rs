use std::{
    fmt::Display,
    str::{FromStr, Split},
};

use super::difference::Difference;

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct GamepadEvent {
    pub left_stick: (f32, f32),
    pub right_stick: (f32, f32),
    pub triggers: (f32, f32),
    pub buttons: u32,
}

impl Display for GamepadEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{};{};{};{};{};{};{}",
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

        add_button_change(&mut buttons, GamepadButton::A);
        add_button_change(&mut buttons, GamepadButton::B);
        add_button_change(&mut buttons, GamepadButton::Y);
        add_button_change(&mut buttons, GamepadButton::X);
        add_button_change(&mut buttons, GamepadButton::Down);
        add_button_change(&mut buttons, GamepadButton::Up);
        add_button_change(&mut buttons, GamepadButton::Left);
        add_button_change(&mut buttons, GamepadButton::Right);
        add_button_change(&mut buttons, GamepadButton::Select);
        add_button_change(&mut buttons, GamepadButton::Select);
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum GamepadButton {
    A = 1 << 0,
    B = 1 << 1,
    Y = 1 << 2,
    X = 1 << 3,
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

/// This maps XBOX 360 Controller.
/// Theese are different for different controllers.
/// For many you will need to use HAT absolute events for DPAD
/// X360 also sends TRIGGER_HAPPY key events for DPAD.
#[cfg(target_os = "linux")]
impl From<GamepadButton> for evdev::Key {
    fn from(value: GamepadButton) -> Self {
        use evdev::Key;
        match value {
            GamepadButton::A => Key::BTN_SOUTH,
            GamepadButton::B => Key::BTN_EAST,
            GamepadButton::Y => Key::BTN_WEST,
            GamepadButton::X => Key::BTN_NORTH,
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
