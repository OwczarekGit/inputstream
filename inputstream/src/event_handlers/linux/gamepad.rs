use crate::event_handlers::handler::{EventHandler, GamepadEventHandler};
use evdev::{
    uinput::{VirtualDevice, VirtualDeviceBuilder},
    AbsInfo, AbsoluteAxisType, AttributeSet, BusType, EventType, InputEvent, InputId, Key,
    UinputAbsSetup,
};
use lib_inputstream::{consts::GAMEPAD_DEVICE_NAME, prelude::*};

impl EventHandler<GamepadState> for GamepadEventHandler {
    fn listen(&self, receiver: std::sync::mpsc::Receiver<GamepadState>) -> crate::Result<()> {
        let mut device = create_device()?;

        let mut gamepad_state = GamepadState::default();

        loop {
            if let Ok(msg) = receiver.recv() {
                let mut ev = vec![];

                let to_dualsense_range = |v: i8| (v as u8) + 128u8;

                let (l_stick, r_stick, triggers, buttons) = gamepad_state.get_diff(&msg);

                if let Some(x) = l_stick.0 {
                    ev.push(InputEvent::new(
                        EventType::ABSOLUTE,
                        AbsoluteAxisType::ABS_X.0,
                        to_dualsense_range(x) as i32,
                    ));
                }
                if let Some(y) = l_stick.1 {
                    ev.push(InputEvent::new(
                        EventType::ABSOLUTE,
                        AbsoluteAxisType::ABS_Y.0,
                        to_dualsense_range(y) as i32,
                    ));
                }

                if let Some(x) = r_stick.0 {
                    ev.push(InputEvent::new(
                        EventType::ABSOLUTE,
                        AbsoluteAxisType::ABS_RX.0,
                        to_dualsense_range(x) as i32,
                    ));
                }
                if let Some(y) = r_stick.1 {
                    ev.push(InputEvent::new(
                        EventType::ABSOLUTE,
                        AbsoluteAxisType::ABS_RY.0,
                        to_dualsense_range(y) as i32,
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
                    match btn {
                        GamepadButton::Down => {
                            ev.push(InputEvent::new(
                                EventType::ABSOLUTE,
                                AbsoluteAxisType::ABS_HAT0Y.0,
                                if state { 1 } else { 0 },
                            ));
                        }
                        GamepadButton::Up => {
                            ev.push(InputEvent::new(
                                EventType::ABSOLUTE,
                                AbsoluteAxisType::ABS_HAT0Y.0,
                                if state { -1 } else { 0 },
                            ));
                        }
                        GamepadButton::Left => {
                            ev.push(InputEvent::new(
                                EventType::ABSOLUTE,
                                AbsoluteAxisType::ABS_HAT0X.0,
                                if state { -1 } else { 0 },
                            ));
                        }
                        GamepadButton::Right => {
                            ev.push(InputEvent::new(
                                EventType::ABSOLUTE,
                                AbsoluteAxisType::ABS_HAT0X.0,
                                if state { 1 } else { 0 },
                            ));
                        }
                        other => {
                            ev.push(InputEvent::new(
                                EventType::KEY,
                                Key::from(other).code(),
                                state.into(),
                            ));
                        }
                    }
                }

                gamepad_state = msg;
                let _ = device.emit(&ev);
            }
        }
    }
}

// NOTE: Maybe we could have multiple different controllers?
//       This one maps to Dualsense.
fn create_device() -> crate::Result<VirtualDevice> {
    let mut buttons = AttributeSet::new();
    buttons.insert(Key::BTN_NORTH);
    buttons.insert(Key::BTN_SOUTH);
    buttons.insert(Key::BTN_WEST);
    buttons.insert(Key::BTN_EAST);
    buttons.insert(Key::BTN_TL);
    buttons.insert(Key::BTN_TR);

    // Not sure why but Dualsense has theese.
    // Don't really seem to be used anywhere.
    // Maybe some accessory.
    buttons.insert(Key::BTN_TL2);
    buttons.insert(Key::BTN_TR2);

    buttons.insert(Key::BTN_SELECT);
    buttons.insert(Key::BTN_START);
    buttons.insert(Key::BTN_MODE);

    buttons.insert(Key::BTN_THUMBL);
    buttons.insert(Key::BTN_THUMBR);

    const MIN: i32 = 0;
    const MAX: i32 = 255;

    // Minimal changes to fire event
    const FUZZ: i32 = 1;

    // Deadzone
    const FLAT: i32 = 1;
    const RES: i32 = 1;

    // Left stick
    let x_axis = UinputAbsSetup::new(
        AbsoluteAxisType::ABS_X,
        AbsInfo::new(0, MIN, MAX, FUZZ, FLAT, RES),
    );
    let y_axis = UinputAbsSetup::new(
        AbsoluteAxisType::ABS_Y,
        AbsInfo::new(0, MIN, MAX, FUZZ, FLAT, RES),
    );

    // Right stick
    let rx_axis = UinputAbsSetup::new(
        AbsoluteAxisType::ABS_RX,
        AbsInfo::new(0, MIN, MAX, FUZZ, FLAT, RES),
    );
    let ry_axis = UinputAbsSetup::new(
        AbsoluteAxisType::ABS_RY,
        AbsInfo::new(0, MIN, MAX, FUZZ, FLAT, RES),
    );

    // Triggers
    let z_axis = UinputAbsSetup::new(
        AbsoluteAxisType::ABS_Z,
        AbsInfo::new(0, MIN, MAX, FUZZ, FLAT, RES),
    );
    let rz_axis = UinputAbsSetup::new(
        AbsoluteAxisType::ABS_RZ,
        AbsInfo::new(0, MIN, MAX, FUZZ, FLAT, RES),
    );

    // Dpad
    let hat_x = UinputAbsSetup::new(AbsoluteAxisType::ABS_HAT0X, AbsInfo::new(0, -1, 1, 0, 0, 1));
    let hat_y = UinputAbsSetup::new(AbsoluteAxisType::ABS_HAT0Y, AbsInfo::new(0, -1, 1, 0, 0, 1));

    // NOTE: Some programs don't like virtual devices,
    //       so we just pretend it's using USB.
    Ok(VirtualDeviceBuilder::new()?
        .name(GAMEPAD_DEVICE_NAME)
        .input_id(InputId::new(BusType::BUS_USB, 0x54C, 0xCE6, 0x8111))
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
