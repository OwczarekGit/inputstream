use std::{
    io::{BufRead, BufReader},
    net::{TcpListener, TcpStream},
    thread,
};

use lib_inputstream::input_event::InputEvent;

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

            match message_raw.parse::<InputEvent>() {
                Ok(ev) => match ev {
                    InputEvent::Osu(event) => {
                        let _ = senders.osu_channel.send(event);
                    }
                    InputEvent::Keyboard(event) => {
                        let _ = senders.keyboard_channel.send(event);
                    }
                    InputEvent::Mouse(event) => {
                        let _ = senders.mouse_channel.send(event);
                    }
                },
                Err(err) => println!("{}", err),
            }
        }
    }
    println!("Device {addr} disconnected.");

    Ok(())
}
