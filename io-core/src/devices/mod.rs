use crate::{IORemoteResult, dispatcher::message::Message};
use std::{
    sync::mpsc::{Receiver, Sender, channel},
    thread::spawn,
};

pub mod linux;

pub fn create_channel<T: Message>() -> (Sender<T>, Receiver<T>) {
    channel()
}

pub const KEYBOARD_DEVICE_NAME: &str = "IORemote Virtual Keyboard";
pub const MOUSE_DEVICE_NAME: &str = "IORemote Virtual Mouse";
pub const DUALSENSE_GAMEPAD_DEVICE_NAME: &str = "DualSense Wireless Controller";
pub const DUALSENSE_MOTION_DEVICE_NAME: &str = "DualSense Wireless Controller Motion Sensors";

pub trait VirtualDevice<D, M: Message>: Sized + Send + 'static {
    fn with_receiver(receiver: Receiver<M>) -> Self;

    fn receiver(&self) -> &Receiver<M>;

    fn device(&self) -> IORemoteResult<D>;

    fn start_threaded(self) -> IORemoteResult<()> {
        spawn(move || self.listen());
        Ok(())
    }

    fn listen(self) -> IORemoteResult<()> {
        let receiver = self.receiver();
        let mut device = self.device()?;
        let mut previous_msg: M = M::default();
        loop {
            let current_msg = receiver.recv()?;
            Self::on_message(&mut device, &current_msg, &previous_msg)?;
            previous_msg = current_msg;
        }
    }

    fn on_message(device: &mut D, msg: &M, prevous_msg: &M) -> IORemoteResult<()>;
}
