use std::sync::mpsc::Receiver;

use evdev::{
    AbsInfo, AbsoluteAxisCode, AttributeSet, BusType, EventType, InputEvent, InputId, KeyCode,
    UinputAbsSetup, uinput::VirtualDevice as EvdevVirtualDevice,
};

use crate::{
    IORemoteResult,
    builtin::messages::{
        difference::Difference,
        gamepad::{Gamepad, gamepad_button::GamepadButton},
    },
    devices::{DUALSENSE_GAMEPAD_DEVICE_NAME, VirtualDevice},
};

pub struct DualsenseGamepadDevice(Receiver<Gamepad>);

impl VirtualDevice<EvdevVirtualDevice, Gamepad> for DualsenseGamepadDevice {
    fn with_receiver(receiver: Receiver<Gamepad>) -> Self {
        Self(receiver)
    }

    fn receiver(&self) -> &Receiver<Gamepad> {
        &self.0
    }

    fn device(&self) -> IORemoteResult<EvdevVirtualDevice> {
        // DualSense controller.
        const VENDOR: u16 = 0x54C;
        const PRODUCT: u16 = 0xCE6;
        const VERSION: u16 = 0x8111;

        Ok(EvdevVirtualDevice::builder()?
            .name(DUALSENSE_GAMEPAD_DEVICE_NAME)
            // NOTE: Some programs don't like virtual devices,
            //       so we just pretend it's using USB.
            .input_id(InputId::new(BusType::BUS_USB, VENDOR, PRODUCT, VERSION))
            .with_keys(&get_buttons())?
            .with_absolute_axis(&get_axis(AbsoluteAxisCode::ABS_X))?
            .with_absolute_axis(&get_axis(AbsoluteAxisCode::ABS_Y))?
            .with_absolute_axis(&get_axis(AbsoluteAxisCode::ABS_RX))?
            .with_absolute_axis(&get_axis(AbsoluteAxisCode::ABS_RY))?
            .with_absolute_axis(&get_axis(AbsoluteAxisCode::ABS_Z))?
            .with_absolute_axis(&get_axis(AbsoluteAxisCode::ABS_RZ))?
            .with_absolute_axis(&get_dpad_axis(AbsoluteAxisCode::ABS_HAT0X))?
            .with_absolute_axis(&get_dpad_axis(AbsoluteAxisCode::ABS_HAT0X))?
            .with_absolute_axis(&get_dpad_axis(AbsoluteAxisCode::ABS_HAT0Y))?
            .build()?)
    }

    fn on_message(
        device: &mut EvdevVirtualDevice,
        msg: &Gamepad,
        prevous_msg: &Gamepad,
    ) -> IORemoteResult<()> {
        let mut evs = vec![];

        let (xl, yl, xr, yr, zl, zr, buttons) = msg.get_diff(prevous_msg);

        let to_dualsense_stick_range = |v: i8| (v as u8) + 128u8;

        let send_motion_stick = |evs: &mut Vec<InputEvent>, value, axis| {
            if let Some(value) = value {
                evs.push(InputEvent::new(
                    EventType::ABSOLUTE.0,
                    axis,
                    to_dualsense_stick_range(value) as i32,
                ));
            }
        };

        let send_motion_trigger = |evs: &mut Vec<InputEvent>, value, axis| {
            if let Some(value) = value {
                evs.push(InputEvent::new(EventType::ABSOLUTE.0, axis, value as i32));
            }
        };

        // Left stick
        send_motion_stick(&mut evs, xl, AbsoluteAxisCode::ABS_X.0);
        send_motion_stick(&mut evs, yl, AbsoluteAxisCode::ABS_Y.0);

        // Right stick
        send_motion_stick(&mut evs, xr, AbsoluteAxisCode::ABS_RX.0);
        send_motion_stick(&mut evs, yr, AbsoluteAxisCode::ABS_RY.0);

        // Triggers stick
        send_motion_trigger(&mut evs, zl, AbsoluteAxisCode::ABS_Z.0);
        send_motion_trigger(&mut evs, zr, AbsoluteAxisCode::ABS_RZ.0);

        for (button, state) in buttons {
            match button {
                GamepadButton::Up => {
                    evs.push(InputEvent::new(
                        EventType::ABSOLUTE.0,
                        AbsoluteAxisCode::ABS_HAT0Y.0,
                        if state { -1 } else { 0 },
                    ));
                }
                GamepadButton::Right => {
                    evs.push(InputEvent::new(
                        EventType::ABSOLUTE.0,
                        AbsoluteAxisCode::ABS_HAT0X.0,
                        if state { 1 } else { 0 },
                    ));
                }
                GamepadButton::Down => {
                    evs.push(InputEvent::new(
                        EventType::ABSOLUTE.0,
                        AbsoluteAxisCode::ABS_HAT0Y.0,
                        if state { 1 } else { 0 },
                    ));
                }
                GamepadButton::Left => {
                    evs.push(InputEvent::new(
                        EventType::ABSOLUTE.0,
                        AbsoluteAxisCode::ABS_HAT0X.0,
                        if state { -1 } else { 0 },
                    ));
                }
                other => {
                    evs.push(InputEvent::new(
                        EventType::KEY.0,
                        KeyCode::from(other).code(),
                        state.into(),
                    ));
                }
            }
        }

        device.emit(&evs)?;
        Ok(())
    }
}

fn get_axis(axis: AbsoluteAxisCode) -> UinputAbsSetup {
    const MIN: i32 = 0;
    const MAX: i32 = 255;

    // Minimal changes to fire event
    const FUZZ: i32 = 1;

    // Deadzone
    const FLAT: i32 = 1;

    const RES: i32 = 1;

    UinputAbsSetup::new(axis, AbsInfo::new(0, MIN, MAX, FUZZ, FLAT, RES))
}

fn get_dpad_axis(axis: AbsoluteAxisCode) -> UinputAbsSetup {
    const MIN: i32 = -1;
    const MAX: i32 = 1;

    // Minimal changes to fire event
    const FUZZ: i32 = 0;

    // Deadzone
    const FLAT: i32 = 0;

    const RES: i32 = 1;

    UinputAbsSetup::new(axis, AbsInfo::new(0, MIN, MAX, FUZZ, FLAT, RES))
}

fn get_buttons() -> AttributeSet<KeyCode> {
    let mut buttons = AttributeSet::new();
    buttons.insert(KeyCode::BTN_NORTH);
    buttons.insert(KeyCode::BTN_SOUTH);
    buttons.insert(KeyCode::BTN_WEST);
    buttons.insert(KeyCode::BTN_EAST);
    buttons.insert(KeyCode::BTN_TL);
    buttons.insert(KeyCode::BTN_TR);

    // Not sure why but Dualsense has theese.
    // Don't really seem to be used anywhere.
    // Maybe some accessory.
    buttons.insert(KeyCode::BTN_TL2);
    buttons.insert(KeyCode::BTN_TR2);

    buttons.insert(KeyCode::BTN_SELECT);
    buttons.insert(KeyCode::BTN_START);
    buttons.insert(KeyCode::BTN_MODE);
    buttons.insert(KeyCode::BTN_THUMBL);
    buttons.insert(KeyCode::BTN_THUMBR);

    buttons
}
