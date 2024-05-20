use std::sync::mpsc::Sender;

use lib_inputstream::prelude::{GamepadState, KeyboardState, MouseState, OsuState};

#[derive(Debug, Clone)]
pub struct Senders {
    pub osu_channel: Sender<OsuState>,
    pub keyboard_channel: Sender<KeyboardState>,
    pub mouse_channel: Sender<MouseState>,
    pub gamepad_channel: Sender<GamepadState>,
}
