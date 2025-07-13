use std::{
    io::Read,
    net::{TcpListener, TcpStream},
    sync::{Arc, mpsc::Sender},
};
pub mod error;
pub use error::{AppRes, Error};

use io_core::{
    builtin::messages::{gamepad::Gamepad, keyboard::Keyboard, motion::Motion, mouse::Mouse},
    devices::{
        VirtualDevice, create_channel,
        linux::{
            gamepad::dualsense::DualsenseGamepadDevice, keyboard::KeyboardDevice,
            motion::dualsense::DualsenseMotionDevice, mouse::MouseDevice,
        },
    },
    dispatcher::{Dispatcher, listener::Listener},
};

fn main() -> AppRes<()> {
    let dispatcher = Arc::new(Dispatcher::default());

    let keyboard_channel = create_channel::<Keyboard>();
    let mouse_channel = create_channel::<Mouse>();
    let gamepad_channel = create_channel::<Gamepad>();
    let motion_channel = create_channel::<Motion>();

    dispatcher.register_listener(Box::new(MouseDispatcher(mouse_channel.0)))?;
    dispatcher.register_listener(Box::new(KeyboardDispatcher(keyboard_channel.0)))?;
    dispatcher.register_listener(Box::new(GamepadDispatcher(gamepad_channel.0)))?;
    dispatcher.register_listener(Box::new(MotionDispatcher(motion_channel.0)))?;

    KeyboardDevice::with_receiver(keyboard_channel.1).start_threaded()?;
    MouseDevice::with_receiver(mouse_channel.1).start_threaded()?;
    DualsenseGamepadDevice::with_receiver(gamepad_channel.1).start_threaded()?;
    DualsenseMotionDevice::with_receiver(motion_channel.1).start_threaded()?;

    let addr = "0.0.0.0:2137".to_string();
    let listener = TcpListener::bind(addr)?;

    for conn in listener.incoming().flatten() {
        _ = handle(conn, dispatcher.clone());
    }

    Ok(())
}

fn handle(mut stream: TcpStream, dispatcher: Arc<Dispatcher>) -> AppRes<()> {
    loop {
        let mut kind = [0u8; 2];
        let mut len = [0u8; 4];

        stream.read_exact(&mut kind)?;
        stream.read_exact(&mut len)?;

        let mut msg_buffer = vec![0; u32::from_be_bytes(len) as usize];

        stream.read_exact(&mut msg_buffer)?;

        let mut combined = Vec::with_capacity(kind.len() + len.len() + msg_buffer.len());
        combined.extend_from_slice(&kind);
        combined.extend_from_slice(&len);
        combined.extend_from_slice(&msg_buffer);

        if let Err(err) = dispatcher.dispatch(&combined) {
            dbg!(err);
            stream.shutdown(std::net::Shutdown::Both)?;
            break Ok(());
        }
    }
}

pub struct MouseDispatcher(Sender<Mouse>);

impl Listener<Mouse> for MouseDispatcher {
    fn dispatch(&self, msg: Mouse) {
        _ = self.0.send(msg);
    }
}

struct KeyboardDispatcher(Sender<Keyboard>);

impl Listener<Keyboard> for KeyboardDispatcher {
    fn dispatch(&self, msg: Keyboard) {
        _ = self.0.send(msg);
    }
}

struct GamepadDispatcher(Sender<Gamepad>);

impl Listener<Gamepad> for GamepadDispatcher {
    fn dispatch(&self, msg: Gamepad) {
        _ = self.0.send(msg);
    }
}

struct MotionDispatcher(Sender<Motion>);

impl Listener<Motion> for MotionDispatcher {
    fn dispatch(&self, msg: Motion) {
        _ = self.0.send(msg);
    }
}
