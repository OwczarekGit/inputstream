use std::{
    error::Error,
    fmt::{Debug, Display},
    io::Read,
    net::{TcpListener, TcpStream},
    str::FromStr,
};

use crate::{error::MessageDeserialize, prelude::EventType};

pub trait Server {
    type Error: Debug + Error;
    type Message: Debug + Default + Display + FromStr;
}

impl Server for TcpServer {
    type Error = std::io::Error;

    type Message = EventType;
}

#[derive(Debug)]
pub struct TcpServer {
    listener: TcpListener,
}

impl TcpServer {
    pub fn new(port: impl Into<u16>) -> Result<Self, std::io::Error> {
        Ok(Self {
            listener: TcpListener::bind(format!("0.0.0.0:{}", port.into()))?,
        })
    }

    pub fn listen(&mut self, handler: impl Fn(TcpStream)) -> Result<(), std::io::Error> {
        for connection in self.listener.incoming().flatten() {
            println!("New connection from: {}", connection.peer_addr()?);
            handler(connection);
        }
        Ok(())
    }

    pub fn decode_message(conn: &mut TcpStream) -> Result<EventType, MessageDeserialize> {
        let mut len = [0; 5];
        conn.read_exact(&mut len)
            .map_err(|_| MessageDeserialize::CouldNotParseLen)?;
        let len = String::from_utf8_lossy(&len)
            .parse()
            .map_err(|_| MessageDeserialize::CouldNotParseLen)?;
        let mut message = vec![0; len];
        conn.read_exact(&mut message)
            .map_err(|_| MessageDeserialize::CouldNotParseStruct)?;
        String::from_utf8_lossy(&message)
            .parse()
            .map_err(|_| MessageDeserialize::CouldNotParseStruct)
    }
}
