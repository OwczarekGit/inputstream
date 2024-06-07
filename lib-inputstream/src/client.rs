use std::{
    error::Error,
    fmt::{Debug, Display},
    io::Write,
    net::{IpAddr, SocketAddr, TcpStream},
    str::FromStr,
};

use crate::prelude::EventType;

pub trait MessageSerializer<M> {
    fn serialize(message: M) -> String;
}

impl<T, M> MessageSerializer<M> for T
where
    M: Debug + Default + Display + FromStr,
{
    fn serialize(message: M) -> String {
        let message = message.to_string();
        let len = message.len();
        format!("{:05}{}", len, message)
    }
}

pub trait Client {
    type Error: Debug + Error;
    type Message: Debug + Default + Display + FromStr;
    fn send(&mut self, message: Self::Message) -> Result<(), Self::Error>;
}

#[derive(Debug)]
pub struct TcpClient {
    socket: TcpStream,
}

impl TcpClient {
    pub fn new(addr: impl Into<IpAddr>, port: impl Into<u16>) -> Result<Self, std::io::Error> {
        let socket_addr = SocketAddr::new(addr.into(), port.into());
        Ok(Self {
            socket: TcpStream::connect(socket_addr)?,
        })
    }
}

impl Client for TcpClient {
    type Error = std::io::Error;

    type Message = EventType;

    fn send(&mut self, message: Self::Message) -> Result<(), Self::Error> {
        let message = Self::serialize(message);
        self.socket.write_all(message.as_bytes())
    }
}
