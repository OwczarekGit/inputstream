use std::{io::Write, net::TcpStream, time::Duration};
mod config;

use clap::Parser;
use config::Config;
use lib_inputstream::consts::{KEYBOARD_PROTOCOL_NAME, MOUSE_PROTOCOL_NAME};
use sdl2::{event::Event, keyboard::Keycode, pixels::Color};

pub fn main() {
    let config = Config::parse();

    let mut socket = TcpStream::connect(format!("{}:{}", config.address, config.port));

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("InputStream SDL2 client", 400, 300)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0x44, 0x44, 0x44));
    canvas.clear();
    canvas.present();

    let mut button_group1 = 0u32;

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        let prev_bg1 = button_group1;
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyUp {
                    keycode: Some(keycode),
                    repeat: false,
                    ..
                } => {
                    let bit: u32 = match keycode {
                        Keycode::A => 0,
                        Keycode::B => 1,
                        Keycode::C => 2,
                        Keycode::D => 3,
                        Keycode::E => 4,
                        Keycode::F => 5,
                        Keycode::G => 6,
                        Keycode::H => 7,
                        Keycode::I => 8,
                        Keycode::J => 9,
                        Keycode::K => 10,
                        Keycode::L => 11,
                        Keycode::M => 12,
                        Keycode::N => 13,
                        Keycode::O => 14,
                        Keycode::P => 15,
                        Keycode::Q => 16,
                        Keycode::R => 17,
                        Keycode::S => 18,
                        Keycode::T => 19,
                        Keycode::U => 20,
                        Keycode::V => 21,
                        Keycode::W => 22,
                        Keycode::X => 23,
                        Keycode::Y => 24,
                        Keycode::Z => 25,
                        _ => 0,
                    };

                    button_group1 &= !(1 << bit);
                }
                Event::KeyDown {
                    keycode: Some(keycode),
                    repeat: false,
                    ..
                } => {
                    let bit: u32 = match keycode {
                        Keycode::A => 0,
                        Keycode::B => 1,
                        Keycode::C => 2,
                        Keycode::D => 3,
                        Keycode::E => 4,
                        Keycode::F => 5,
                        Keycode::G => 6,
                        Keycode::H => 7,
                        Keycode::I => 8,
                        Keycode::J => 9,
                        Keycode::K => 10,
                        Keycode::L => 11,
                        Keycode::M => 12,
                        Keycode::N => 13,
                        Keycode::O => 14,
                        Keycode::P => 15,
                        Keycode::Q => 16,
                        Keycode::R => 17,
                        Keycode::S => 18,
                        Keycode::T => 19,
                        Keycode::U => 20,
                        Keycode::V => 21,
                        Keycode::W => 22,
                        Keycode::X => 23,
                        Keycode::Y => 24,
                        Keycode::Z => 25,
                        _ => 0,
                    };
                    button_group1 |= 1 << bit;
                }
                _ => {}
            };
        }
        let mouse_state = event_pump.relative_mouse_state();
        let dx = mouse_state.x() as f32 * config.mouse_accel;
        let dy = mouse_state.y() as f32 * config.mouse_accel;
        let mut m_buttons = 0u32;

        if mouse_state.left() {
            m_buttons |= 0x01;
        }

        if mouse_state.right() {
            m_buttons |= 0x02;
        }

        if mouse_state.middle() {
            m_buttons |= 0x04;
        }

        if let Ok(socket) = &mut socket {
            if prev_bg1 != button_group1 {
                let _ = socket
                    .write(format!("{KEYBOARD_PROTOCOL_NAME}|{button_group1};0;0;0\n").as_bytes());
            }

            //let _ =
            //  socket.write(format!("{MOUSE_PROTOCOL_NAME}|{dx};{dy};0;{m_buttons}\n").as_bytes());
        }
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
