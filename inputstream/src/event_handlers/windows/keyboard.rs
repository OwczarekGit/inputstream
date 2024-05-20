use lib_inputstream::event::keyboard::KeyboardState;

use crate::event_handlers::handler::{EventHandler, KeyboardEventHandler};

impl EventHandler<KeyboardState> for KeyboardEventHandler {
    fn listen(&self, receiver: std::sync::mpsc::Receiver<KeyboardState>) -> crate::Result<()> {
        loop {
            if let Ok(msg) = receiver.recv() {
                println!("Keyboard is not implemented for windows: {:?}", msg);
            }
        }
    }
}
