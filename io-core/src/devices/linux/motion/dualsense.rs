use crate::{
    IORemoteResult,
    builtin::messages::{difference::Difference, motion::Motion},
    devices::{DUALSENSE_MOTION_DEVICE_NAME, VirtualDevice},
};
use evdev::{
    AbsInfo, AbsoluteAxisCode, AttributeSet, BusType, EventType, InputEvent, InputId, PropType,
    UinputAbsSetup, uinput::VirtualDevice as EvdevVirtualDevice,
};
use std::sync::mpsc::Receiver;

pub struct DualsenseMotionDevice(Receiver<Motion>);

impl VirtualDevice<EvdevVirtualDevice, Motion> for DualsenseMotionDevice {
    fn with_receiver(receiver: Receiver<Motion>) -> Self {
        Self(receiver)
    }

    fn receiver(&self) -> &Receiver<Motion> {
        &self.0
    }

    fn device(&self) -> IORemoteResult<EvdevVirtualDevice> {
        // DualSense motion controls.
        const VENDOR: u16 = 0x54C;
        const PRODUCT: u16 = 0xCE6;
        const VERSION: u16 = 0x8100;

        Ok(EvdevVirtualDevice::builder()?
            .name(DUALSENSE_MOTION_DEVICE_NAME)
            .input_id(InputId::new(BusType::BUS_USB, VENDOR, PRODUCT, VERSION))
            .with_properties(&get_prop())?
            .with_absolute_axis(&get_axis_pos(AbsoluteAxisCode::ABS_X))?
            .with_absolute_axis(&get_axis_pos(AbsoluteAxisCode::ABS_Y))?
            .with_absolute_axis(&get_axis_pos(AbsoluteAxisCode::ABS_Z))?
            .with_absolute_axis(&get_axis_accel(AbsoluteAxisCode::ABS_RX))?
            .with_absolute_axis(&get_axis_accel(AbsoluteAxisCode::ABS_RY))?
            .with_absolute_axis(&get_axis_accel(AbsoluteAxisCode::ABS_RZ))?
            .build()?)
    }

    fn on_message(
        device: &mut EvdevVirtualDevice,
        msg: &Motion,
        prevous_msg: &Motion,
    ) -> IORemoteResult<()> {
        let mut evs = vec![];

        let send_motion_pos = |evs: &mut Vec<InputEvent>, value, axis| {
            if let Some(value) = value {
                evs.push(InputEvent::new(EventType::ABSOLUTE.0, axis, value as i32));
            }
        };

        let send_motion_acc = |evs: &mut Vec<InputEvent>, value, axis| {
            if let Some(value) = value {
                evs.push(InputEvent::new(EventType::ABSOLUTE.0, axis, value));
            }
        };

        let (x, y, z, ax, ay, az) = msg.get_diff(prevous_msg);

        send_motion_pos(&mut evs, x, AbsoluteAxisCode::ABS_X.0);
        send_motion_pos(&mut evs, y, AbsoluteAxisCode::ABS_Y.0);
        send_motion_pos(&mut evs, z, AbsoluteAxisCode::ABS_Z.0);

        send_motion_acc(&mut evs, ax, AbsoluteAxisCode::ABS_RX.0);
        send_motion_acc(&mut evs, ay, AbsoluteAxisCode::ABS_RY.0);
        send_motion_acc(&mut evs, az, AbsoluteAxisCode::ABS_RZ.0);

        device.emit(&evs)?;
        Ok(())
    }
}

fn get_prop() -> AttributeSet<PropType> {
    let mut props = AttributeSet::new();
    props.insert(PropType::ACCELEROMETER);
    props
}

fn get_axis_pos(axis: AbsoluteAxisCode) -> UinputAbsSetup {
    const MIN: i32 = -32768;
    const MAX: i32 = 32768;

    // Minimal changes to fire event
    const FUZZ: i32 = 16;

    // Deadzone
    const FLAT: i32 = 1;

    const RES: i32 = 8192;

    UinputAbsSetup::new(axis, AbsInfo::new(0, MIN, MAX, FUZZ, FLAT, RES))
}

fn get_axis_accel(axis: AbsoluteAxisCode) -> UinputAbsSetup {
    const MIN: i32 = -2097152;
    const MAX: i32 = 2097152;

    // Minimal changes to fire event
    const FUZZ: i32 = 16;

    // Deadzone
    const FLAT: i32 = 1;

    const RES: i32 = 1024;

    UinputAbsSetup::new(axis, AbsInfo::new(0, MIN, MAX, FUZZ, FLAT, RES))
}
