use std::vec;

use crate::{config::config, Result};
use evdev::{
    uinput::VirtualDeviceBuilder, AttributeSet, EventType, InputEvent, Key, RelativeAxisType,
};
use lib_inputstream::{
    consts::{MOUSE_DEVICE_NAME, OSU_DEVICE_NAME},
    input_event::{MouseEvent, OsuEvent},
    osu_key_args::get_evdev_key,
};

use super::handler::{EventHandler, MouseEventHandler, OsuEventHandler};

impl EventHandler<OsuEvent> for OsuEventHandler {
    fn listen(&self, receiver: std::sync::mpsc::Receiver<OsuEvent>) -> Result<()> {
        let config = config();
        let k1 = get_evdev_key(&config.osu_key1).unwrap_or(Key::KEY_Z);
        let k2 = get_evdev_key(&config.osu_key2).unwrap_or(Key::KEY_X);

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

impl EventHandler<MouseEvent> for MouseEventHandler {
    fn listen(&self, receiver: std::sync::mpsc::Receiver<MouseEvent>) -> Result<()> {
        let mut buttons = AttributeSet::<Key>::new();
        buttons.insert(Key::BTN_LEFT);
        buttons.insert(Key::BTN_RIGHT);
        buttons.insert(Key::BTN_MIDDLE);

        let mut motion = AttributeSet::<RelativeAxisType>::new();
        motion.insert(RelativeAxisType::REL_X);
        motion.insert(RelativeAxisType::REL_Y);
        motion.insert(RelativeAxisType::REL_WHEEL);

        let mut device = VirtualDeviceBuilder::new()?
            .name(MOUSE_DEVICE_NAME)
            .with_keys(&buttons)?
            .with_relative_axes(&motion)?
            .build()?;

        loop {
            if let Ok(event) = receiver.recv() {
                let mut ev = vec![];

                if event.dx.abs() > 0f32 {
                    ev.push(InputEvent::new(
                        EventType::RELATIVE,
                        RelativeAxisType::REL_X.0,
                        event.dx as i32,
                    ));
                }

                if event.dy.abs() > 0f32 {
                    ev.push(InputEvent::new(
                        EventType::RELATIVE,
                        RelativeAxisType::REL_Y.0,
                        -event.dy as i32,
                    ));
                }

                if event.dw.abs() > 0f32 {
                    ev.push(InputEvent::new(
                        EventType::RELATIVE,
                        RelativeAxisType::REL_WHEEL.0,
                        event.dw as i32,
                    ));
                }

                ev.push(InputEvent::new(
                    EventType::KEY,
                    Key::BTN_LEFT.code(),
                    event.btn_left_state.into(),
                ));

                ev.push(InputEvent::new(
                    EventType::KEY,
                    Key::BTN_RIGHT.code(),
                    event.btn_right_state.into(),
                ));

                ev.push(InputEvent::new(
                    EventType::KEY,
                    Key::BTN_MIDDLE.code(),
                    event.btn_middle_state.into(),
                ));

                let _ = device.emit(&ev);
            }
        }
    }
}
