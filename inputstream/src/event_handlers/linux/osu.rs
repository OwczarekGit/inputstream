use crate::Result;
use evdev::{uinput::VirtualDeviceBuilder, AttributeSet, EventType, InputEvent, Key};
use lib_inputstream::{consts::OSU_DEVICE_NAME, prelude::*};

use crate::{
    config::config,
    event_handlers::handler::{EventHandler, OsuEventHandler},
};

impl EventHandler<OsuState> for OsuEventHandler {
    fn listen(&self, receiver: std::sync::mpsc::Receiver<OsuState>) -> Result<()> {
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

        let mut osu_state = OsuState::default();

        loop {
            if let Ok(msg) = receiver.recv() {
                let mut ev = vec![];

                let (k1_change, k2_change) = osu_state.get_diff(&msg);

                if let Some((state, _key)) = k1_change {
                    ev.push(InputEvent::new(EventType::KEY, k1.code(), state.into()));
                }

                if let Some((state, _key)) = k2_change {
                    ev.push(InputEvent::new(EventType::KEY, k2.code(), state.into()));
                }

                osu_state = msg;
                let _ = device.emit(&ev);
            }
        }
    }
}
