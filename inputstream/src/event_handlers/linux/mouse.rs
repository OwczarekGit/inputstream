use crate::Result;
use evdev::{
    uinput::VirtualDeviceBuilder, AttributeSet, EventType, InputEvent, Key, RelativeAxisType,
};
use lib_inputstream::{
    consts::MOUSE_DEVICE_NAME,
    event::mouse::{MouseButton, MouseEvent},
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
                        event.dy as i32,
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
                    event.button_state(MouseButton::Left).into(),
                ));

                ev.push(InputEvent::new(
                    EventType::KEY,
                    Key::BTN_RIGHT.code(),
                    event.button_state(MouseButton::Right).into(),
                ));

                ev.push(InputEvent::new(
                    EventType::KEY,
                    Key::BTN_MIDDLE.code(),
                    event.button_state(MouseButton::Middle).into(),
                ));

                let _ = device.emit(&ev);
            }
        }
    }
}
