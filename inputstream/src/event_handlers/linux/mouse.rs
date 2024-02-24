use crate::Result;
use evdev::{
    uinput::VirtualDeviceBuilder, AttributeSet, EventType, InputEvent, Key, RelativeAxisType,
};
use lib_inputstream::{
    consts::MOUSE_DEVICE_NAME,
    event::{difference::Difference, mouse::MouseEvent},
};

use crate::event_handlers::handler::{EventHandler, MouseEventHandler};

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

        let mut mouse_state = MouseEvent::default();

        loop {
            if let Ok(event) = receiver.recv() {
                let mut ev = vec![];

                let (dx_diff, dy_diff, dw_diff, buttons_diff) = mouse_state.get_diff(&event);

                if let Some(dx) = dx_diff {
                    ev.push(InputEvent::new(
                        EventType::RELATIVE,
                        RelativeAxisType::REL_X.0,
                        dx as i32,
                    ));
                }

                if let Some(dy) = dy_diff {
                    ev.push(InputEvent::new(
                        EventType::RELATIVE,
                        RelativeAxisType::REL_Y.0,
                        dy as i32,
                    ));
                }

                if let Some(dw) = dw_diff {
                    ev.push(InputEvent::new(
                        EventType::RELATIVE,
                        RelativeAxisType::REL_WHEEL.0,
                        dw as i32,
                    ));
                }

                for (state, button) in buttons_diff {
                    ev.push(InputEvent::new(
                        EventType::KEY,
                        Key::from(button).code(),
                        state.into(),
                    ));
                }

                mouse_state = event;
                let _ = device.emit(&ev);
            }
        }
    }
}
