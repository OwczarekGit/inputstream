use std::{io::Write, net::TcpStream};

use crate::{senders::Senders, Result};
use lib_inputstream::{prelude::*, server::TcpServer};

pub fn listen(mut stream: TcpStream, senders: Senders) -> Result<()> {
    let addr = stream.peer_addr()?;
    'conn: loop {
        let Ok(message) = TcpServer::decode_message(&mut stream) else {
            break 'conn;
        };
        match message {
            EventType::Ping => {
                println!("Got ping from: {addr}.");
                let _ = stream.write(b"PONG");
            }
            EventType::Osu(ev) => {
                let _ = senders.osu_channel.send(ev);
            }
            EventType::Mouse(ev) => {
                let _ = senders.mouse_channel.send(ev);
            }
            EventType::Keyboard(ev) => {
                let _ = senders.keyboard_channel.send(ev);
            }
            EventType::Gamepad(ev) => {
                let _ = senders.gamepad_channel.send(ev);
            }
        }
    }

    println!("Device {addr} disconnected.");

    // Perform a cleanup (unpress all the keys).
    let _ = senders.osu_channel.send(OsuState::default());
    let _ = senders.keyboard_channel.send(KeyboardState::default());
    let _ = senders.mouse_channel.send(MouseState::default());
    let _ = senders.gamepad_channel.send(GamepadState::default());

    Ok(())
}
