use lib_inputstream::event::osu::OsuEvent;

use crate::event_handlers::handler::{EventHandler, OsuEventHandler};

impl EventHandler<OsuEvent> for OsuEventHandler {
    fn listen(&self, receiver: std::sync::mpsc::Receiver<OsuEvent>) -> crate::Result<()> {
        loop {
            if let Ok(msg) = receiver.recv() {
                println!("osu is not implemented for windows: {:?}", msg);
            }
        }
    }
}
