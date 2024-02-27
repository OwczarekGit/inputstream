use std::{
    fmt::Display,
    str::{FromStr, Split},
};

use crate::event::difference::Difference;

use super::gamepad_button::GamepadButton;

/// We use normalized values.
/// For stick its i8, where -128 is maximum left tilt, and 127 is max right tilt.
/// And same goes for Y axis except up and down tilts.
/// This approach allows us to have normalized, gamepad independant values.
/// For triggers we use u8, 0 is no press, 255 is fully pressed.
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct GamepadState {
    pub gamepad_id: u8,
    pub left_stick: (i8, i8),
    pub right_stick: (i8, i8),
    pub triggers: (u8, u8),
    pub buttons: u32,
}

impl GamepadState {
    pub fn set_button_state(&mut self, button: impl Into<GamepadButton>, state: bool) {
        let bit = button.into() as u32;
        if state {
            self.buttons |= bit;
        } else {
            self.buttons &= !bit;
        }
    }
}

impl Display for GamepadState {
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

impl FromStr for GamepadState {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(';');

        let next_i8 = |split: &mut Split<'_, _>, name: &str| {
            Ok::<i8, String>(
                split
                    .next()
                    .ok_or(format!("Missing {name}."))?
                    .trim()
                    .parse()
                    .unwrap_or(0),
            )
        };

        let next_u8 = |split: &mut Split<'_, _>, name: &str| {
            Ok::<u8, String>(
                split
                    .next()
                    .ok_or(format!("Missing {name}."))?
                    .trim()
                    .parse()
                    .unwrap_or(0),
            )
        };

        let gamepad_id: u8 = split
            .next()
            .ok_or("Missing gamepad id".to_string())?
            .trim()
            .parse()
            .map_err(|_| "Invalid gamepad id")?;

        let left_stick = (
            next_i8(&mut split, "left stick x")?,
            next_i8(&mut split, "left stick y")?,
        );

        let right_stick = (
            next_i8(&mut split, "right stick x")?,
            next_i8(&mut split, "right stick y")?,
        );

        let triggers = (
            next_u8(&mut split, "trigger left")?,
            next_u8(&mut split, "trigger right")?,
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

impl Difference for GamepadState {
    type Output = (
        (Option<i8>, Option<i8>),
        (Option<i8>, Option<i8>),
        (Option<u8>, Option<u8>),
        Vec<(bool, GamepadButton)>,
    );

    fn get_diff(&self, other: &Self) -> Self::Output {
        let motion_diff = |val: i8, new: i8| val.ne(&new).then_some(new);
        let motion_diff_u8 = |val: u8, new: u8| val.ne(&new).then_some(new);

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
            motion_diff_u8(self.triggers.0, other.triggers.0),
            motion_diff_u8(self.triggers.1, other.triggers.1),
        );

        (diff_left, diff_right, diff_trigger, buttons)
    }
}
