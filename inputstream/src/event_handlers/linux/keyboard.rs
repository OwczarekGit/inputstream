use crate::Result;
use evdev::{uinput::VirtualDeviceBuilder, AttributeSet, EventType, InputEvent, Key};
use lib_inputstream::{
    consts::KEYBOARD_DEVICE_NAME, input_event::KeyboardEvent,
    key_group::keyboard_key_group1::KeyboardKeyGroup1,
};

use crate::event_handlers::handler::{EventHandler, KeyboardEventHandler};

impl EventHandler<KeyboardEvent> for KeyboardEventHandler {
    fn listen(&self, receiver: std::sync::mpsc::Receiver<KeyboardEvent>) -> Result<()> {
        let mut buttons = AttributeSet::<Key>::new();

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

        let mut device = VirtualDeviceBuilder::new()?
            .name(KEYBOARD_DEVICE_NAME)
            .with_keys(&buttons)?
            .build()?;

        let mut group1 = KeyboardKeyGroup1::new();

        loop {
            if let Ok(msg) = receiver.recv() {
                let mut events = vec![];
                group1.check_states(msg.key_group1);
                for (key, pressed) in Vec::<(Key, bool)>::from(group1.clone()) {
                    events.push(InputEvent::new(EventType::KEY, key.code(), pressed.into()));
                }
                let _ = device.emit(&events);
            }
        }
    }
}
