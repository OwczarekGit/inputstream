use std::mem::{size_of, transmute, zeroed};

use lib_inputstream::prelude::*;
use winapi::um::winuser::{
    SendInput, INPUT, INPUT_MOUSE, MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP,
    MOUSEEVENTF_MIDDLEDOWN, MOUSEEVENTF_MIDDLEUP, MOUSEEVENTF_MOVE, MOUSEEVENTF_RIGHTDOWN,
    MOUSEEVENTF_RIGHTUP, MOUSEEVENTF_WHEEL,
};

use crate::event_handlers::handler::{EventHandler, MouseEventHandler};

// NOTE: This probably will need a refactor.
impl EventHandler<MouseEvent> for MouseEventHandler {
    fn listen(&self, receiver: std::sync::mpsc::Receiver<MouseEvent>) -> crate::Result<()> {
        let mut mouse_state = MouseEvent::default();
        loop {
            if let Ok(msg) = receiver.recv() {
                let (dx, dy, dw, buttons) = mouse_state.get_diff(&msg);

                // Mouse movement
                unsafe {
                    if dx.is_some() || dy.is_some() {
                        let mut input: INPUT = zeroed();

                        input.type_ = INPUT_MOUSE;
                        input.u.mi_mut().dwFlags = MOUSEEVENTF_MOVE;

                        if let Some(dx) = dx {
                            input.u.mi_mut().dx = dx as i32;
                        }
                        if let Some(dy) = dy {
                            input.u.mi_mut().dy = dy as i32;
                        }

                        SendInput(1, &mut input, size_of::<INPUT>() as i32);
                    }
                }

                // Mouse wheel
                unsafe {
                    if let Some(dw) = dw {
                        let mut input: INPUT = zeroed();

                        input.type_ = INPUT_MOUSE;
                        input.u.mi_mut().dwFlags = MOUSEEVENTF_WHEEL;

                        // NOTE: This seems to be necessary because mouseData is u32 instead of i32.
                        input.u.mi_mut().mouseData = transmute(dw as i32 * 120);

                        SendInput(1, &mut input, size_of::<INPUT>() as i32);
                    }
                }

                // Mouse buttons
                unsafe {
                    if !buttons.is_empty() {
                        for (state, btn) in buttons {
                            let mut input: INPUT = zeroed();
                            input.type_ = INPUT_MOUSE;

                            if state {
                                match btn {
                                    MouseButton::Left => {
                                        input.u.mi_mut().dwFlags |= MOUSEEVENTF_LEFTDOWN
                                    }
                                    MouseButton::Right => {
                                        input.u.mi_mut().dwFlags |= MOUSEEVENTF_RIGHTDOWN
                                    }
                                    MouseButton::Middle => {
                                        input.u.mi_mut().dwFlags |= MOUSEEVENTF_MIDDLEDOWN
                                    }
                                }
                            } else {
                                match btn {
                                    MouseButton::Left => {
                                        input.u.mi_mut().dwFlags |= MOUSEEVENTF_LEFTUP
                                    }
                                    MouseButton::Right => {
                                        input.u.mi_mut().dwFlags |= MOUSEEVENTF_RIGHTUP
                                    }
                                    MouseButton::Middle => {
                                        input.u.mi_mut().dwFlags |= MOUSEEVENTF_MIDDLEUP
                                    }
                                }
                            }

                            SendInput(1, &mut input, size_of::<INPUT>() as i32);
                        }
                    }
                }

                mouse_state = msg;
            }
        }
    }
}
