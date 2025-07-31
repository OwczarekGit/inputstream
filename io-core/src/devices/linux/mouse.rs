use std::sync::mpsc::Receiver;

use evdev::{
    AttributeSet, EventType, InputEvent, KeyCode, RelativeAxisCode,
    uinput::VirtualDevice as EvdevVirtualDevice,
};

use crate::{
    IORemoteResult,
    builtin::messages::{difference::Difference, mouse::Mouse},
    devices::{MOUSE_DEVICE_NAME, VirtualDevice},
};

pub struct MouseDevice(Receiver<Mouse>);

impl VirtualDevice<EvdevVirtualDevice, Mouse> for MouseDevice {
    fn with_receiver(receiver: Receiver<Mouse>) -> Self {
        Self(receiver)
    }

    fn receiver(&self) -> &Receiver<Mouse> {
        &self.0
    }

    fn device(&self) -> IORemoteResult<EvdevVirtualDevice> {
        Ok(EvdevVirtualDevice::builder()?
            .name(MOUSE_DEVICE_NAME)
            .with_relative_axes(&get_relative_axes())?
            .with_keys(&get_keys())?
            .build()?)
    }

    fn on_message(
        device: &mut EvdevVirtualDevice,
        msg: &Mouse,
        prevous_msg: &Mouse,
    ) -> IORemoteResult<()> {
        let mut evs = vec![];

        let (dx, dy, dw, d_btn) = msg.get_diff(prevous_msg);

        for (btn, state) in d_btn {
            evs.push(InputEvent::new(
                EventType::KEY.0,
                KeyCode::from(btn).0,
                state as i32,
            ));
        }

        let mut push_rel_ev = |code, diff: Option<f32>| {
            if let Some(diff) = diff {
                evs.push(InputEvent::new(EventType::RELATIVE.0, code, diff as i32));
            }
        };

        push_rel_ev(RelativeAxisCode::REL_X.0, dx);
        push_rel_ev(RelativeAxisCode::REL_Y.0, dy);
        push_rel_ev(RelativeAxisCode::REL_WHEEL.0, dw);

        if evs.len() > 0 {
            device.emit(&evs)?;
        }
        Ok(())
    }
}

fn get_relative_axes() -> AttributeSet<RelativeAxisCode> {
    let mut axes = AttributeSet::new();

    axes.insert(RelativeAxisCode::REL_X);
    axes.insert(RelativeAxisCode::REL_Y);
    axes.insert(RelativeAxisCode::REL_WHEEL);

    axes
}

fn get_keys() -> AttributeSet<KeyCode> {
    let mut axes = AttributeSet::new();

    axes.insert(KeyCode::BTN_LEFT);
    axes.insert(KeyCode::BTN_RIGHT);
    axes.insert(KeyCode::BTN_MIDDLE);
    axes.insert(KeyCode::BTN_FORWARD);
    axes.insert(KeyCode::BTN_BACK);

    axes
}
