use crate::Result;
use evdev::{uinput::VirtualDeviceBuilder, AttributeSet, EventType, InputEvent, Key};
use lib_inputstream::{consts::OSU_DEVICE_NAME, input_event::OsuEvent};

use crate::{
    config::config,
    event_handlers::handler::{EventHandler, OsuEventHandler},
};

impl EventHandler<OsuEvent> for OsuEventHandler {
    fn listen(&self, receiver: std::sync::mpsc::Receiver<OsuEvent>) -> Result<()> {
        let (k1, k2) = {
            let config = config().clone();
            (
                Key::try_from(config.osu_key1).unwrap_or(Key::KEY_Z),
                Key::try_from(config.osu_key2).unwrap_or(Key::KEY_X),
            )
        };

        let mut keys = AttributeSet::new();
        keys.insert(k1);
        keys.insert(k2);

        let mut device = VirtualDeviceBuilder::new()?
            .name(OSU_DEVICE_NAME)
            .with_keys(&keys)?
            .build()?;

        let mut prev_state: Option<OsuEvent> = None;

        loop {
            if let Ok(msg) = receiver.recv() {
                let mut ev = vec![];

                if let Some(prev) = &prev_state {
                    if prev.key1_state != msg.key1_state {
                        ev.push(InputEvent::new(
                            EventType::KEY,
                            k1.code(),
                            msg.key1_state.into(),
                        ));
                    }
                } else {
                    ev.push(InputEvent::new(
                        EventType::KEY,
                        k1.code(),
                        msg.key1_state.into(),
                    ));
                }

                if let Some(prev) = &prev_state {
                    if prev.key2_state != msg.key2_state {
                        ev.push(InputEvent::new(
                            EventType::KEY,
                            k2.code(),
                            msg.key2_state.into(),
                        ));
                    }
                } else {
                    ev.push(InputEvent::new(
                        EventType::KEY,
                        k2.code(),
                        msg.key2_state.into(),
                    ));
                }

                prev_state = Some(msg);
                let _ = device.emit(&ev);
            }
        }
    }
}
