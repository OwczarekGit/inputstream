use std::{io::Write, net::TcpStream, thread::sleep, time::Duration};

use io_core::{DEFAULT_PORT, builtin::messages::mouse::Mouse, dispatcher::message::MessageEncoder};

fn main() {
    let addr = format!("0.0.0.0:{DEFAULT_PORT}");
    let mut conn = TcpStream::connect(addr).unwrap();

    let mut mouse = Mouse::default();
    let mut delta = 0f32;
    const RANGE: f32 = 10f32;
    const STEP: f32 = 0.05;

    let mut buffer = vec![];
    let mut encoder = MessageEncoder::new();

    loop {
        buffer.clear();
        mouse.0 = delta.cos() * RANGE;
        mouse.1 = delta.sin() * RANGE;

        encoder.encode(mouse, &mut buffer);
        conn.write(&buffer).unwrap();

        delta = delta + STEP;
        sleep(Duration::from_millis(4));
    }
}
