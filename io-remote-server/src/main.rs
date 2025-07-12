use std::{
    io::Read,
    net::{TcpListener, TcpStream},
    sync::{Arc, mpsc::Sender},
};
pub mod error;
pub use error::{AppRes, Error};

use io_core::{
    builtin::messages::{gamepad::Gamepad, keyboard::Keyboard, mouse::Mouse},
    devices::{
        VirtualDevice, create_channel,
        linux::{
            gamepad::dualsense::DualsenseGamepadDevice, keyboard::KeyboardDevice,
            mouse::MouseDevice,
        },
    },
    dispatcher::{Dispatcher, listener::Listener},
};

fn main() -> AppRes<()> {
    let dispatcher = Arc::new(Dispatcher::default());

    let keyboard_channel = create_channel::<Keyboard>();
    let mouse_channel = create_channel::<Mouse>();
    let gamepad_channel = create_channel::<Gamepad>();

    dispatcher.register_listener(Box::new(MouseLogger(mouse_channel.0)))?;
    dispatcher.register_listener(Box::new(KeyboardLogger(keyboard_channel.0)))?;
    dispatcher.register_listener(Box::new(GamepadLogger(gamepad_channel.0)))?;

    KeyboardDevice::with_receiver(keyboard_channel.1).start_threaded()?;
    MouseDevice::with_receiver(mouse_channel.1).start_threaded()?;
    DualsenseGamepadDevice::with_receiver(gamepad_channel.1).start_threaded()?;

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

pub struct MouseLogger(Sender<Mouse>);

impl Listener<Mouse> for MouseLogger {
    fn dispatch(&self, msg: Mouse) {
        _ = self.0.send(msg);
    }
}

struct KeyboardLogger(Sender<Keyboard>);

impl Listener<Keyboard> for KeyboardLogger {
    fn dispatch(&self, msg: Keyboard) {
        _ = self.0.send(msg);
    }
}

struct GamepadLogger(Sender<Gamepad>);

impl Listener<Gamepad> for GamepadLogger {
    fn dispatch(&self, msg: Gamepad) {
        _ = self.0.send(msg);
    }
}
