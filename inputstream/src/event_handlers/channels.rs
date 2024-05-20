use std::sync::mpsc::{channel, Receiver, Sender};

use crate::senders::Senders;
use lib_inputstream::prelude::*;

fn create_channel<T>() -> (Sender<T>, Receiver<T>) {
    channel()
}

pub fn create_channels() -> (
    Senders,
    Receiver<OsuState>,
    Receiver<KeyboardState>,
    Receiver<MouseState>,
    Receiver<GamepadState>,
) {
    let osu_channel = create_channel::<OsuState>();
    let keyboard_channel = create_channel::<KeyboardState>();
    let mouse_channel = create_channel::<MouseState>();
    let gamepad_channel = create_channel::<GamepadState>();

    (
        Senders {
            osu_channel: osu_channel.0,
            keyboard_channel: keyboard_channel.0,
            mouse_channel: mouse_channel.0,
            gamepad_channel: gamepad_channel.0,
        },
        osu_channel.1,
        keyboard_channel.1,
        mouse_channel.1,
        gamepad_channel.1,
    )
}
