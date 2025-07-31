use std::sync::mpsc::Receiver;

use evdev::{
    AttributeSet, EventType, InputEvent, KeyCode, uinput::VirtualDevice as EvdevVirtualDevice,
};

use crate::{
    IORemoteResult,
    builtin::messages::{difference::Difference, keyboard::Keyboard},
    devices::{KEYBOARD_DEVICE_NAME, VirtualDevice},
};

pub struct KeyboardDevice(Receiver<Keyboard>);

impl VirtualDevice<EvdevVirtualDevice, Keyboard> for KeyboardDevice {
    fn with_receiver(receiver: Receiver<Keyboard>) -> Self {
        Self(receiver)
    }

    fn receiver(&self) -> &Receiver<Keyboard> {
        &self.0
    }

    fn device(&self) -> IORemoteResult<EvdevVirtualDevice> {
        Ok(EvdevVirtualDevice::builder()?
            .name(KEYBOARD_DEVICE_NAME)
            .with_keys(&get_keys())?
            .build()?)
    }

    fn on_message(
        device: &mut EvdevVirtualDevice,
        msg: &Keyboard,
        prevous_msg: &Keyboard,
    ) -> IORemoteResult<()> {
        let mut evs = vec![];
        let diff = msg.get_diff(prevous_msg);

        for (key, pressed) in diff {
            evs.push(InputEvent::new(
                EventType::KEY.0,
                KeyCode::from(key).0,
                pressed as i32,
            ));
        }

        if evs.len() > 0 {
            device.emit(&evs)?;
        }

        Ok(())
    }
}

fn get_keys() -> AttributeSet<KeyCode> {
    let mut keys = AttributeSet::new();
    keys.insert(KeyCode::KEY_A);
    keys.insert(KeyCode::KEY_B);
    keys.insert(KeyCode::KEY_C);
    keys.insert(KeyCode::KEY_D);
    keys.insert(KeyCode::KEY_E);
    keys.insert(KeyCode::KEY_F);
    keys.insert(KeyCode::KEY_G);
    keys.insert(KeyCode::KEY_H);
    keys.insert(KeyCode::KEY_I);
    keys.insert(KeyCode::KEY_J);
    keys.insert(KeyCode::KEY_K);
    keys.insert(KeyCode::KEY_L);
    keys.insert(KeyCode::KEY_M);
    keys.insert(KeyCode::KEY_N);
    keys.insert(KeyCode::KEY_O);
    keys.insert(KeyCode::KEY_P);
    keys.insert(KeyCode::KEY_Q);
    keys.insert(KeyCode::KEY_R);
    keys.insert(KeyCode::KEY_S);
    keys.insert(KeyCode::KEY_T);
    keys.insert(KeyCode::KEY_U);
    keys.insert(KeyCode::KEY_V);
    keys.insert(KeyCode::KEY_W);
    keys.insert(KeyCode::KEY_X);
    keys.insert(KeyCode::KEY_Y);
    keys.insert(KeyCode::KEY_Z);
    keys.insert(KeyCode::KEY_0);
    keys.insert(KeyCode::KEY_1);
    keys.insert(KeyCode::KEY_2);
    keys.insert(KeyCode::KEY_3);
    keys.insert(KeyCode::KEY_4);
    keys.insert(KeyCode::KEY_5);
    keys.insert(KeyCode::KEY_6);
    keys.insert(KeyCode::KEY_7);
    keys.insert(KeyCode::KEY_8);
    keys.insert(KeyCode::KEY_9);
    keys.insert(KeyCode::KEY_GRAVE);
    keys.insert(KeyCode::KEY_MINUS);
    keys.insert(KeyCode::KEY_EQUAL);
    keys.insert(KeyCode::KEY_BACKSPACE);
    keys.insert(KeyCode::KEY_ESC);
    keys.insert(KeyCode::KEY_TAB);
    keys.insert(KeyCode::KEY_CAPSLOCK);
    keys.insert(KeyCode::KEY_LEFTSHIFT);
    keys.insert(KeyCode::KEY_LEFTCTRL);
    keys.insert(KeyCode::KEY_LEFTMETA);
    keys.insert(KeyCode::KEY_LEFTALT);
    keys.insert(KeyCode::KEY_SPACE);
    keys.insert(KeyCode::KEY_RIGHTALT);
    keys.insert(KeyCode::KEY_CONTEXT_MENU);
    keys.insert(KeyCode::KEY_RIGHTMETA);
    keys.insert(KeyCode::KEY_RIGHTCTRL);
    keys.insert(KeyCode::KEY_RIGHTSHIFT);
    keys.insert(KeyCode::KEY_ENTER);
    keys.insert(KeyCode::KEY_UP);
    keys.insert(KeyCode::KEY_LEFT);
    keys.insert(KeyCode::KEY_RIGHT);
    keys.insert(KeyCode::KEY_DOWN);
    keys.insert(KeyCode::KEY_F1);
    keys.insert(KeyCode::KEY_F2);
    keys.insert(KeyCode::KEY_F3);
    keys.insert(KeyCode::KEY_F4);
    keys.insert(KeyCode::KEY_F5);
    keys.insert(KeyCode::KEY_F6);
    keys.insert(KeyCode::KEY_F7);
    keys.insert(KeyCode::KEY_F8);
    keys.insert(KeyCode::KEY_F9);
    keys.insert(KeyCode::KEY_F10);
    keys.insert(KeyCode::KEY_F11);
    keys.insert(KeyCode::KEY_F12);
    keys.insert(KeyCode::KEY_INSERT);
    keys.insert(KeyCode::KEY_DELETE);
    keys.insert(KeyCode::KEY_HOME);
    keys.insert(KeyCode::KEY_END);
    keys.insert(KeyCode::KEY_PAGEUP);
    keys.insert(KeyCode::KEY_PAGEDOWN);
    keys.insert(KeyCode::KEY_PRINT);
    keys.insert(KeyCode::KEY_SCROLLLOCK);
    keys.insert(KeyCode::KEY_PAUSE);
    keys.insert(KeyCode::KEY_LEFTBRACE);
    keys.insert(KeyCode::KEY_RIGHTBRACE);
    keys.insert(KeyCode::KEY_SEMICOLON);
    keys.insert(KeyCode::KEY_APOSTROPHE);
    keys.insert(KeyCode::KEY_BACKSLASH);
    keys.insert(KeyCode::KEY_DOT);
    keys.insert(KeyCode::KEY_COMMA);
    keys.insert(KeyCode::KEY_SLASH);
    keys
}
