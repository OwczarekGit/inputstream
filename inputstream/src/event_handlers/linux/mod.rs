use crate::Result;
use evdev::{uinput::VirtualDeviceBuilder, AttributeSet, EventType, InputEvent, Key};
use lib_inputstream::{consts::OSU_DEVICE_NAME, input_event::OsuEvent};

use super::handler::{EventHandler, OsuEventHandler};

impl EventHandler<OsuEvent> for OsuEventHandler {
    fn listen(&self, receiver: std::sync::mpsc::Receiver<OsuEvent>) -> Result<()> {
        let mut keys = AttributeSet::new();
        keys.insert(Key::KEY_Z);
        keys.insert(Key::KEY_X);

        let mut device = VirtualDeviceBuilder::new()?
            .name(OSU_DEVICE_NAME)
            .with_keys(&keys)?
            .build()?;

        loop {
            if let Ok(msg) = receiver.recv() {
                let mut ev = vec![];

                ev.push(InputEvent::new(
                    EventType::KEY,
                    Key::KEY_Z.code(),
                    msg.key1_state.into(),
                ));

                ev.push(InputEvent::new(
                    EventType::KEY,
                    Key::KEY_X.code(),
                    msg.key2_state.into(),
                ));

                let _ = device.emit(&ev);
            }
        }
    }
}
