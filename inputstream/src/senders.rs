use std::sync::mpsc::Sender;

use lib_inputstream::input_event::{KeyboardEvent, MouseEvent, OsuEvent};

#[derive(Debug, Clone)]
pub struct Senders {
    pub osu_channel: Sender<OsuEvent>,
    pub keyboard_channel: Sender<KeyboardEvent>,
    pub mouse_channel: Sender<MouseEvent>,
}
