use std::{
    fmt::Display,
    str::{FromStr, Split},
};

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
enum GamepadButton {
    X = 1 << 0,
    Circle = 1 << 1,
    Triangle = 1 << 2,
    Square = 1 << 3,
    Down = 1 << 4,
    Right = 1 << 5,
    Up = 1 << 6,
    Left = 1 << 7,
    Options = 1 << 8,
    Create = 1 << 9,
    Ps = 1 << 10,
    Mute = 1 << 11,
    BumperLeft = 1 << 12,
    BumperRight = 1 << 13,
    StickLeft = 1 << 14,
    StickRight = 1 << 15,
    Touchpad = 1 << 16,
}

#[cfg(target_os = "linux")]
impl From<GamepadButton> for evdev::Key {
    fn from(value: GamepadButton) -> Self {
        use evdev::Key;
        match value {
            // I'm not sure what is going on here.
            // I'm very likely doing something wrong.
            GamepadButton::X => todo!(),
            GamepadButton::Circle => Key::BTN_THUMB2,
            GamepadButton::Triangle => Key::BTN_START,
            GamepadButton::Square => Key::BTN_THUMB,
            GamepadButton::Down => todo!(),
            GamepadButton::Right => todo!(),
            GamepadButton::Up => todo!(),
            GamepadButton::Left => Key::BTN_LEFT,
            GamepadButton::Options => Key::BTN_1,
            GamepadButton::Create => Key::BTN_0,
            GamepadButton::Ps => Key::BTN_4, // Or maybe swap with mute?
            GamepadButton::Mute => Key::BTN_5,
            GamepadButton::BumperLeft => Key::BTN_DPAD_UP, // What?
            GamepadButton::BumperRight => Key::BTN_DPAD_DOWN, // I don't even know…
            GamepadButton::StickLeft => Key::BTN_2,
            GamepadButton::StickRight => Key::BTN_3,
            GamepadButton::Touchpad => todo!(),
        }
    }
}
