use std::sync::mpsc::Sender;

use lib_inputstream::input_event::OsuEvent;

#[derive(Debug, Clone)]
pub struct Senders {
    pub osu_channel: Sender<OsuEvent>,
    pub keyboard_channel: Sender<String>,
    pub mouse_channel: Sender<String>,
}
