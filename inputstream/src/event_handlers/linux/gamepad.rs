use std::{thread, time::Duration};

use evdev::{
    uinput::{VirtualDevice, VirtualDeviceBuilder},
    AbsInfo, AbsoluteAxisType, AttributeSet, BusType, EventType, InputEvent, InputId, Key,
    UinputAbsSetup,
};
use lib_inputstream::{consts::GAMEPAD_DEVICE_NAME, event::gamepad::GamepadEvent};

use crate::event_handlers::handler::{EventHandler, GamepadEventHandler};

impl EventHandler<GamepadEvent> for GamepadEventHandler {
    fn listen(&self, receiver: std::sync::mpsc::Receiver<GamepadEvent>) -> crate::Result<()> {
        let mut device = create_device()?;
        thread::sleep(Duration::from_secs(10));

        let res = device.emit(&[InputEvent::new(
            EventType::KEY,
            Key::BTN_TRIGGER_HAPPY1.code(),
            1,
        )]);

        dbg!(res);

        loop {
            thread::sleep(Duration::from_secs(10));
            if let Ok(_msg) = receiver.recv() {
                let _ = device.emit(&[]);
            }
        }
    }
}

fn create_device() -> crate::Result<VirtualDevice> {
    let mut buttons = AttributeSet::new();
    /* buttons.insert(Key::BTN_0);
    buttons.insert(Key::BTN_1);
    buttons.insert(Key::BTN_2);
    buttons.insert(Key::BTN_3);
    buttons.insert(Key::BTN_4);
    buttons.insert(Key::BTN_5);
    buttons.insert(Key::BTN_6);
    buttons.insert(Key::BTN_7);
    buttons.insert(Key::BTN_8);
    buttons.insert(Key::BTN_9);
    buttons.insert(Key::BTN_DPAD_UP);
    buttons.insert(Key::BTN_DPAD_DOWN);
    buttons.insert(Key::BTN_DPAD_LEFT);
    buttons.insert(Key::BTN_DPAD_RIGHT);
    buttons.insert(Key::BTN_THUMB);
    buttons.insert(Key::BTN_THUMB2);
    buttons.insert(Key::BTN_START);
    buttons.insert(Key::BTN_SELECT);
    buttons.insert(Key::BTN_LEFT);
    buttons.insert(Key::BTN_RIGHT);
    buttons.insert(Key::BTN_EXTRA); */
    buttons.insert(Key::BTN_TRIGGER_HAPPY1);
    buttons.insert(Key::BTN_TRIGGER_HAPPY2);
    buttons.insert(Key::BTN_TRIGGER_HAPPY3);
    buttons.insert(Key::BTN_TRIGGER_HAPPY4);

    let x_axis = UinputAbsSetup::new(
        AbsoluteAxisType::ABS_X,
        AbsInfo::new(0, -32767, 32767, 20, 20, 128),
    );

    let y_axis = UinputAbsSetup::new(
        AbsoluteAxisType::ABS_Y,
        AbsInfo::new(0, -32767, 32767, 20, 20, 1),
    );

    let z_axis = UinputAbsSetup::new(
        AbsoluteAxisType::ABS_Z,
        AbsInfo::new(0, -32767, 32767, 20, 20, 128),
    );

    Ok(VirtualDeviceBuilder::new()?
        .name(GAMEPAD_DEVICE_NAME)
        .input_id(InputId::new(BusType::BUS_USB, 0x045E, 0x02EA, 0))
        .with_keys(&buttons)?
        .with_absolute_axis(&x_axis)?
        .with_absolute_axis(&y_axis)?
        .with_absolute_axis(&z_axis)?
        .build()?)
}
