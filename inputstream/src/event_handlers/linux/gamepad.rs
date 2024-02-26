use crate::event_handlers::handler::{EventHandler, GamepadEventHandler};
use evdev::{
    uinput::{VirtualDevice, VirtualDeviceBuilder},
    AbsInfo, AbsoluteAxisType, AttributeSet, BusType, EventType, InputEvent, InputId, Key,
    UinputAbsSetup,
};
use lib_inputstream::{consts::GAMEPAD_DEVICE_NAME, prelude::*};

impl EventHandler<GamepadEvent> for GamepadEventHandler {
    fn listen(&self, receiver: std::sync::mpsc::Receiver<GamepadEvent>) -> crate::Result<()> {
        let mut device = create_device()?;

        let mut gamepad_state = GamepadEvent::default();

        loop {
            if let Ok(msg) = receiver.recv() {
                let mut ev = vec![];

                let (l_stick, r_stick, triggers, buttons) = gamepad_state.get_diff(&msg);

                if let Some(x) = l_stick.0 {
                    ev.push(InputEvent::new(
                        EventType::ABSOLUTE,
                        AbsoluteAxisType::ABS_X.0,
                        x as i32,
                    ));
                }
                if let Some(y) = l_stick.1 {
                    ev.push(InputEvent::new(
                        EventType::ABSOLUTE,
                        AbsoluteAxisType::ABS_Y.0,
                        y as i32,
                    ));
                }

                if let Some(x) = r_stick.0 {
                    ev.push(InputEvent::new(
                        EventType::ABSOLUTE,
                        AbsoluteAxisType::ABS_RX.0,
                        x as i32,
                    ));
                }
                if let Some(y) = r_stick.1 {
                    ev.push(InputEvent::new(
                        EventType::ABSOLUTE,
                        AbsoluteAxisType::ABS_RY.0,
                        y as i32,
                    ));
                }

                if let Some(l) = triggers.0 {
                    ev.push(InputEvent::new(
                        EventType::ABSOLUTE,
                        AbsoluteAxisType::ABS_Z.0,
                        l as i32,
                    ));
                }
                if let Some(r) = triggers.1 {
                    ev.push(InputEvent::new(
                        EventType::ABSOLUTE,
                        AbsoluteAxisType::ABS_RZ.0,
                        r as i32,
                    ));
                }

                for (state, btn) in buttons {
                    ev.push(InputEvent::new(
                        EventType::KEY,
                        Key::from(btn).code(),
                        state.into(),
                    ));
                }

                gamepad_state = msg;
                let _ = device.emit(&ev);
            }
        }
    }
}

fn create_device() -> crate::Result<VirtualDevice> {
    let mut buttons = AttributeSet::new();
    buttons.insert(Key::BTN_NORTH);
    buttons.insert(Key::BTN_SOUTH);
    buttons.insert(Key::BTN_WEST);
    buttons.insert(Key::BTN_EAST);
    buttons.insert(Key::BTN_TL);
    buttons.insert(Key::BTN_TR);

    buttons.insert(Key::BTN_SELECT);
    buttons.insert(Key::BTN_START);
    buttons.insert(Key::BTN_MODE);

    buttons.insert(Key::BTN_THUMBL);
    buttons.insert(Key::BTN_THUMBR);

    buttons.insert(Key::BTN_TRIGGER_HAPPY1);
    buttons.insert(Key::BTN_TRIGGER_HAPPY2);
    buttons.insert(Key::BTN_TRIGGER_HAPPY3);
    buttons.insert(Key::BTN_TRIGGER_HAPPY4);

    const MIN: i32 = -32768;
    const MAX: i32 = 32767;
    const FUZZ: i32 = 16;
    const FLAT: i32 = 128;
    const RES: i32 = 1;

    const Z_MIN: i32 = 0;
    const Z_MAX: i32 = 255;
    const Z_RES: i32 = 1;
    const Z_FUZZ: i32 = 1;
    const Z_FLAT: i32 = 1;

    let x_axis = UinputAbsSetup::new(
        AbsoluteAxisType::ABS_X,
        AbsInfo::new(0, MIN, MAX, FUZZ, FLAT, RES),
    );
    let y_axis = UinputAbsSetup::new(
        AbsoluteAxisType::ABS_Y,
        AbsInfo::new(0, MIN, MAX, FUZZ, FLAT, RES),
    );

    let rx_axis = UinputAbsSetup::new(
        AbsoluteAxisType::ABS_RX,
        AbsInfo::new(0, MIN, MAX, FUZZ, FLAT, RES),
    );
    let ry_axis = UinputAbsSetup::new(
        AbsoluteAxisType::ABS_RY,
        AbsInfo::new(0, MIN, MAX, FUZZ, FLAT, RES),
    );

    let z_axis = UinputAbsSetup::new(
        AbsoluteAxisType::ABS_Z,
        AbsInfo::new(0, Z_MIN, Z_MAX, Z_FUZZ, Z_FLAT, Z_RES),
    );
    let rz_axis = UinputAbsSetup::new(
        AbsoluteAxisType::ABS_RZ,
        AbsInfo::new(0, Z_MIN, Z_MAX, Z_FUZZ, Z_FLAT, Z_RES),
    );

    let hat_x = UinputAbsSetup::new(AbsoluteAxisType::ABS_HAT0X, AbsInfo::new(0, -1, 1, 1, 1, 1));
    let hat_y = UinputAbsSetup::new(AbsoluteAxisType::ABS_HAT0Y, AbsInfo::new(0, -1, 1, 1, 1, 1));

    Ok(VirtualDeviceBuilder::new()?
        .name(GAMEPAD_DEVICE_NAME)
        .input_id(InputId::new(BusType::BUS_USB, 0x45E, 0x2A1, 0x100))
        .with_keys(&buttons)?
        .with_absolute_axis(&hat_x)?
        .with_absolute_axis(&hat_y)?
        .with_absolute_axis(&x_axis)?
        .with_absolute_axis(&y_axis)?
        .with_absolute_axis(&rx_axis)?
        .with_absolute_axis(&ry_axis)?
        .with_absolute_axis(&z_axis)?
        .with_absolute_axis(&rz_axis)?
        .build()?)
}
