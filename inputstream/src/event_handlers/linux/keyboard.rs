use crate::Result;
use evdev::{uinput::VirtualDeviceBuilder, AttributeSet, EventType, InputEvent, Key};
use lib_inputstream::{
    consts::KEYBOARD_DEVICE_NAME,
    event::{difference::Difference, keyboard::KeyboardEvent},
};

use crate::event_handlers::handler::{EventHandler, KeyboardEventHandler};

impl EventHandler<KeyboardEvent> for KeyboardEventHandler {
    fn listen(&self, receiver: std::sync::mpsc::Receiver<KeyboardEvent>) -> Result<()> {
        let mut buttons = AttributeSet::<Key>::new();
        add_buttons(&mut buttons);

        let mut device = VirtualDeviceBuilder::new()?
            .name(KEYBOARD_DEVICE_NAME)
            .with_keys(&buttons)?
            .build()?;

        let mut keyboard_state = KeyboardEvent::default();

        loop {
            if let Ok(msg) = receiver.recv() {
                let mut events = vec![];

                for (state, key) in keyboard_state.get_diff(&msg) {
                    let key: Key = key.into();
                    events.push(InputEvent::new(EventType::KEY, key.code(), state.into()));
                }

                keyboard_state = msg;
                let _ = device.emit(&events);
            }
        }
    }
}

fn add_buttons(buttons: &mut AttributeSet<Key>) {
    // Group 1
    buttons.insert(Key::KEY_A);
    buttons.insert(Key::KEY_B);
    buttons.insert(Key::KEY_C);
    buttons.insert(Key::KEY_D);
    buttons.insert(Key::KEY_E);
    buttons.insert(Key::KEY_F);
    buttons.insert(Key::KEY_G);
    buttons.insert(Key::KEY_H);
    buttons.insert(Key::KEY_I);
    buttons.insert(Key::KEY_J);
    buttons.insert(Key::KEY_K);
    buttons.insert(Key::KEY_L);
    buttons.insert(Key::KEY_M);
    buttons.insert(Key::KEY_N);
    buttons.insert(Key::KEY_O);
    buttons.insert(Key::KEY_P);
    buttons.insert(Key::KEY_Q);
    buttons.insert(Key::KEY_R);
    buttons.insert(Key::KEY_S);
    buttons.insert(Key::KEY_T);
    buttons.insert(Key::KEY_U);
    buttons.insert(Key::KEY_V);
    buttons.insert(Key::KEY_W);
    buttons.insert(Key::KEY_X);
    buttons.insert(Key::KEY_Y);
    buttons.insert(Key::KEY_Z);
    buttons.insert(Key::KEY_BACKSLASH);
    buttons.insert(Key::KEY_LEFTBRACE);
    buttons.insert(Key::KEY_RIGHTBRACE);
    buttons.insert(Key::KEY_PRINT);
    buttons.insert(Key::KEY_SCROLLLOCK);
    buttons.insert(Key::KEY_PAUSE);

    // Group 2
    buttons.insert(Key::KEY_0);
    buttons.insert(Key::KEY_1);
    buttons.insert(Key::KEY_2);
    buttons.insert(Key::KEY_3);
    buttons.insert(Key::KEY_4);
    buttons.insert(Key::KEY_5);
    buttons.insert(Key::KEY_6);
    buttons.insert(Key::KEY_7);
    buttons.insert(Key::KEY_8);
    buttons.insert(Key::KEY_9);
    buttons.insert(Key::KEY_MINUS);
    buttons.insert(Key::KEY_EQUAL);
    buttons.insert(Key::KEY_BACKSPACE);
    buttons.insert(Key::KEY_GRAVE);
    buttons.insert(Key::KEY_ENTER);
    buttons.insert(Key::KEY_TAB);
    buttons.insert(Key::KEY_ESC);
    buttons.insert(Key::KEY_CAPSLOCK);
    buttons.insert(Key::KEY_LEFTSHIFT);
    buttons.insert(Key::KEY_LEFTCTRL);
    buttons.insert(Key::KEY_LEFTALT);
    buttons.insert(Key::KEY_LEFTMETA);
    buttons.insert(Key::KEY_SPACE);
    buttons.insert(Key::KEY_RIGHTALT);
    buttons.insert(Key::KEY_CONTEXT_MENU);
    buttons.insert(Key::KEY_RIGHTCTRL);
    buttons.insert(Key::KEY_RIGHTSHIFT);
    buttons.insert(Key::KEY_COMMA);
    buttons.insert(Key::KEY_DOT);
    buttons.insert(Key::KEY_SLASH);
    buttons.insert(Key::KEY_SEMICOLON);
    buttons.insert(Key::KEY_APOSTROPHE);

    // Group 3
    buttons.insert(Key::KEY_F1);
    buttons.insert(Key::KEY_F2);
    buttons.insert(Key::KEY_F3);
    buttons.insert(Key::KEY_F4);
    buttons.insert(Key::KEY_F5);
    buttons.insert(Key::KEY_F6);
    buttons.insert(Key::KEY_F7);
    buttons.insert(Key::KEY_F8);
    buttons.insert(Key::KEY_F9);
    buttons.insert(Key::KEY_F10);
    buttons.insert(Key::KEY_F11);
    buttons.insert(Key::KEY_F12);
    buttons.insert(Key::KEY_LEFT);
    buttons.insert(Key::KEY_RIGHT);
    buttons.insert(Key::KEY_UP);
    buttons.insert(Key::KEY_DOWN);
    buttons.insert(Key::KEY_INSERT);
    buttons.insert(Key::KEY_DELETE);
    buttons.insert(Key::KEY_HOME);
    buttons.insert(Key::KEY_END);
    buttons.insert(Key::KEY_PAGEUP);
    buttons.insert(Key::KEY_PAGEDOWN);
    buttons.insert(Key::KEY_NUMERIC_0);
    buttons.insert(Key::KEY_NUMERIC_1);
    buttons.insert(Key::KEY_NUMERIC_2);
    buttons.insert(Key::KEY_NUMERIC_3);
    buttons.insert(Key::KEY_NUMERIC_4);
    buttons.insert(Key::KEY_NUMERIC_5);
    buttons.insert(Key::KEY_NUMERIC_6);
    buttons.insert(Key::KEY_NUMERIC_7);
    buttons.insert(Key::KEY_NUMERIC_8);
    buttons.insert(Key::KEY_NUMERIC_9);
}
