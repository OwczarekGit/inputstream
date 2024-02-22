use std::{io::Write, net::TcpStream, thread, time::Duration};
mod config;

use clap::Parser;
use config::Config;
use lib_inputstream::{
    consts::{KEYBOARD_PROTOCOL_NAME, MOUSE_PROTOCOL_NAME, OSU_PROTOCOL_NAME},
    key_group::key_group_bitmask::KeyGroupBitmask,
};
use sdl2::{event::Event, keyboard::Keycode, mouse::MouseButton, pixels::Color};

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

    sdl_context.mouse().set_relative_mouse_mode(true);
    sdl_context.mouse().show_cursor(true);

    canvas.set_draw_color(Color::RGB(0x44, 0x44, 0x44));
    canvas.clear();
    canvas.present();

    let mut m_buttons = 0u32;
    let mut osu_keys_state = 0u32;
    let mut button_group1 = 0u32;
    let mut button_group2 = 0u32;
    let mut button_group3 = 0u32;

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        let prev_osu_keys = osu_keys_state;
        let prev_bg1 = button_group1;
        let prev_bg2 = button_group2;
        let prev_bg3 = button_group3;
        let prev_m_buttons = m_buttons;
        let mut delta_wheel = 0f32;
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::Z),
                    repeat: false,
                    ..
                } => {
                    osu_keys_state |= 0x01;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::X),
                    repeat: false,
                    ..
                } => {
                    osu_keys_state |= 0x02;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Z),
                    repeat: false,
                    ..
                } => {
                    osu_keys_state &= !0x01;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::X),
                    repeat: false,
                    ..
                } => {
                    osu_keys_state &= !0x02;
                }
                Event::MouseWheel { precise_y, .. } => delta_wheel = precise_y,
                Event::KeyUp {
                    keycode: Some(keycode),
                    repeat: false,
                    ..
                } => {
                    if let Ok(bit) = KeyGroupBitmask::try_from(keycode) {
                        button_group1 &= !(1 << *bit);
                    } else if let Ok(bit) = KeyGroupBitmask::try_from(keycode) {
                        button_group2 &= !(1 << *bit);
                    } else if let Ok(bit) = KeyGroupBitmask::try_from(keycode) {
                        button_group3 &= !(1 << *bit);
                    }
                }
                Event::KeyDown {
                    keycode: Some(keycode),
                    repeat: false,
                    ..
                } => {
                    if let Ok(bit) = KeyGroupBitmask::try_from(keycode) {
                        button_group1 |= 1 << *bit;
                    } else if let Ok(bit) = KeyGroupBitmask::try_from(keycode) {
                        button_group2 |= 1 << *bit;
                    } else if let Ok(bit) = KeyGroupBitmask::try_from(keycode) {
                        button_group3 |= 1 << *bit;
                    }
                }
                _ => {}
            };
        }
        let mouse_state: sdl2::mouse::RelativeMouseState = event_pump.relative_mouse_state();
        let dx = mouse_state.x() as f32 * config.mouse_accel;
        let dy = mouse_state.y() as f32 * config.mouse_accel;

        if mouse_state.is_mouse_button_pressed(MouseButton::Left) {
            m_buttons |= 0x01;
        } else {
            m_buttons &= !0x01;
        }

        if mouse_state.is_mouse_button_pressed(MouseButton::Right) {
            m_buttons |= 0x02;
        } else {
            m_buttons &= !0x02;
        }

        if mouse_state.is_mouse_button_pressed(MouseButton::Middle) {
            m_buttons |= 0x04;
        } else {
            m_buttons &= !0x04;
        }

        if let Ok(socket) = &mut socket {
            if prev_bg1 != button_group1 || prev_bg2 != button_group2 || prev_bg3 != button_group3 {
                let _ = socket.write(
                    format!("{KEYBOARD_PROTOCOL_NAME}|{button_group1};{button_group2};{button_group3};0\n")
                        .as_bytes(),
                );
            }

            if dx.abs() > 0.0
                || dy.abs() > 0.0
                || delta_wheel.abs() > 0.0
                || (prev_m_buttons != m_buttons)
            {
                let _ = socket.write(
                    format!("{MOUSE_PROTOCOL_NAME}|{dx};{dy};{delta_wheel};{m_buttons}\n")
                        .as_bytes(),
                );
            }

            if prev_osu_keys != osu_keys_state {
                let _ = socket.write(format!("{OSU_PROTOCOL_NAME}|{osu_keys_state}\n").as_bytes());
            }
        }
        thread::sleep(Duration::from_millis(config.rate));
    }
}
