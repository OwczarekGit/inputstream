use std::sync::mpsc::{channel, Receiver, Sender};

use lib_inputstream::input_event::{MouseEvent, OsuEvent};

use crate::senders::Senders;

fn create_channel<T>() -> (Sender<T>, Receiver<T>) {
    channel()
}

pub fn create_channels() -> (
    Senders,
    Receiver<OsuEvent>,
    Receiver<String>,
    Receiver<MouseEvent>,
) {
    let osu_channel = create_channel::<OsuEvent>();
    let keyboard_channel = create_channel::<String>();
    let mouse_channel = create_channel::<MouseEvent>();

    (
        Senders {
            osu_channel: osu_channel.0,
            keyboard_channel: keyboard_channel.0,
            mouse_channel: mouse_channel.0,
        },
        osu_channel.1,
        keyboard_channel.1,
        mouse_channel.1,
    )
}
