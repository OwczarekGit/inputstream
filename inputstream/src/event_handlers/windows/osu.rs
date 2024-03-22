use lib_inputstream::event::osu::OsuEvent;
use std::mem::{size_of, zeroed};
use winapi::um::winuser::{SendInput, INPUT, INPUT_KEYBOARD, KEYBDINPUT, KEYEVENTF_KEYUP};

use crate::{
    config::config,
    event_handlers::handler::{EventHandler, OsuEventHandler},
};
use lib_inputstream::prelude::Difference;
use winapi::ctypes::c_int;

impl EventHandler<OsuEvent> for OsuEventHandler {
    fn listen(&self, receiver: std::sync::mpsc::Receiver<OsuEvent>) -> crate::Result<()> {
        let (k1, k2) = {
            let config = config().clone();
            (
                c_int::try_from(config.osu_key1).unwrap_or(0x5A),
                c_int::try_from(config.osu_key2).unwrap_or(0x58),
            )
        };

        let mut osu_state = OsuEvent::default();

        loop {
            if let Ok(msg) = receiver.recv() {
                let (k1_change, k2_change) = osu_state.get_diff(&msg);

                if let Some((state, _k)) = k1_change {
                    unsafe {
                        let mut key: KEYBDINPUT = zeroed();
                        key.wVk = k1 as u16;
                        key.dwFlags = if state { 0 } else { KEYEVENTF_KEYUP };

                        let mut input: INPUT = zeroed();
                        input.type_ = INPUT_KEYBOARD;
                        *input.u.ki_mut() = key;

                        SendInput(1, &mut input, size_of::<INPUT>() as i32);
                    }
                }

                if let Some((state, _k)) = k2_change {
                    unsafe {
                        let mut key: KEYBDINPUT = zeroed();
                        key.wVk = k2 as u16;
                        key.dwFlags = if state { 0 } else { KEYEVENTF_KEYUP };

                        let mut input: INPUT = zeroed();
                        input.type_ = INPUT_KEYBOARD;
                        *input.u.ki_mut() = key;

                        SendInput(1, &mut input, size_of::<INPUT>() as i32);
                    }
                }

                osu_state = msg;
            }
        }
    }
}
