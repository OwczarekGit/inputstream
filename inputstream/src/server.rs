use std::{
    io::{BufRead, BufReader},
    net::{TcpListener, TcpStream},
    str::FromStr,
    thread,
};

use lib_inputstream::prelude::*;

use crate::{senders::Senders, Result};

#[derive(Debug)]
pub struct Server {
    port: u16,
    listener: TcpListener,
}

impl Server {
    pub fn new(port: u16) -> Result<Self> {
        let addr = format!("0.0.0.0:{}", port);
        println!("Opening socket on port: {}", port);

        Ok(Self {
            port,
            listener: TcpListener::bind(addr)?,
        })
    }

    pub fn start(&mut self, senders: Senders) -> Result<()> {
        println!("Starting the server on port: {}", self.port);
        for stream in self.listener.incoming().flatten() {
            println!("New connection from: {}", stream.peer_addr()?);
            let senders = senders.clone();
            thread::spawn(move || listen(stream, senders));
        }

        Ok(())
    }
}

fn listen(stream: TcpStream, senders: Senders) -> Result<()> {
    let addr = stream.peer_addr()?;
    let mut reader = BufReader::new(&stream);
    'conn: loop {
        let mut message_raw = String::new();
        if let Ok(bytes) = reader.read_line(&mut message_raw) {
            if bytes == 0 {
                break 'conn;
            }

            if let Ok(ev) = EventType::from_str(&message_raw) {
                match ev {
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
        }
    }
    println!("Device {addr} disconnected.");

    // Perform a cleanup (unpress all the keys).
    let _ = senders.osu_channel.send(OsuEvent::default());
    let _ = senders.keyboard_channel.send(KeyboardEvent::default());
    let _ = senders.mouse_channel.send(MouseEvent::default());
    let _ = senders.gamepad_channel.send(GamepadEvent::default());

    Ok(())
}
