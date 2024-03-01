use lib_inputstream::event::gamepad::gamepad_state::GamepadState;

use crate::event_handlers::handler::{EventHandler, GamepadEventHandler};

impl EventHandler<GamepadState> for GamepadEventHandler {
    fn listen(&self, receiver: std::sync::mpsc::Receiver<GamepadState>) -> crate::Result<()> {
        loop {
            if let Ok(msg) = receiver.recv() {
                println!("Gamepad is not implemented for windows: {:?}", msg);
            }
        }
    }
}
