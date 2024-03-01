use lib_inputstream::event::keyboard::KeyboardEvent;

use crate::event_handlers::handler::{EventHandler, KeyboardEventHandler};

impl EventHandler<KeyboardEvent> for KeyboardEventHandler {
    fn listen(&self, receiver: std::sync::mpsc::Receiver<KeyboardEvent>) -> crate::Result<()> {
        loop {
            if let Ok(msg) = receiver.recv() {
                println!("Keyboard is not implemented for windows: {:?}", msg);
            }
        }
    }
}
