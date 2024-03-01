use lib_inputstream::event::mouse::MouseEvent;
use winapi::{
    shared::windef::POINT,
    um::winuser::{GetCursorPos, SetCursorPos},
};

use crate::event_handlers::handler::{EventHandler, MouseEventHandler};

impl EventHandler<MouseEvent> for MouseEventHandler {
    fn listen(&self, receiver: std::sync::mpsc::Receiver<MouseEvent>) -> crate::Result<()> {
        loop {
            if let Ok(msg) = receiver.recv() {
                unsafe {
                    let mut current_pos = POINT { x: 0, y: 0 };
                    GetCursorPos(&mut current_pos);
                    SetCursorPos(current_pos.x + msg.dx as i32, current_pos.y + msg.dy as i32);
                }
            }
        }
    }
}
