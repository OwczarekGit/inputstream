use crate::{
    IORemoteResult,
    builtin::messages::motion::Motion,
    devices::{DUALSENSE_MOTION_DEVICE_NAME, VirtualDevice},
};
use evdev::{
    AbsInfo, AbsoluteAxisCode, AttributeSet, PropType, UinputAbsSetup,
    uinput::VirtualDevice as EvdevVirtualDevice,
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
        Ok(EvdevVirtualDevice::builder()?
            .name(DUALSENSE_MOTION_DEVICE_NAME)
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
    ) -> crate::IORemoteResult<()> {
        todo!()
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
