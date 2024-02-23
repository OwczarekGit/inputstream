use std::{
    io::Write,
    net::{IpAddr, SocketAddr, TcpStream},
};

use crate::event::EventType;

#[derive(Debug)]
pub struct Client {
    stream: TcpStream,
}

impl Client {
    pub fn new(addr: impl Into<IpAddr>, port: impl Into<u16>) -> Result<Self, std::io::Error> {
        let socket_addr = SocketAddr::new(addr.into(), port.into());
        Ok(Self {
            stream: TcpStream::connect(socket_addr)?,
        })
    }

    pub fn send_event(&mut self, event: EventType) -> Result<usize, std::io::Error> {
        self.stream.write(event.to_string().as_bytes())
    }
}
