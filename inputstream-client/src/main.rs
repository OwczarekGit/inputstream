use std::{thread, time::Duration};
mod config;

use clap::Parser;
use config::Config;
use lib_inputstream::{
    client::Client,
    event::{
        keyboard::{KeyboardEvent, KeyboardEventGroup},
        mouse::{MouseButton, MouseEvent},
        osu::{OsuEvent, OsuKey},
        EventType,
    },
};
use sdl2::{event::Event, keyboard::Keycode, pixels::Color};

pub fn main() {
    let config = Config::parse();

    let mut client = Client::new(config.address, config.port);

    let sdl_context = sdl2::init().unwrap();

    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("InputStream SDL2 client", 400, 300)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    //sdl_context.mouse().set_relative_mouse_mode(true);
    //sdl_context.mouse().show_cursor(true);

    canvas.set_draw_color(Color::RGB(0x44, 0x44, 0x44));
    canvas.clear();
    canvas.present();

    let mut keyboard_state = KeyboardEvent::default();
    let mut mouse_state = MouseEvent::default();
    let mut osu_state = OsuEvent::default();

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        let prev_keyboard_state = keyboard_state;
        let prev_mouse_state = mouse_state;
        let prev_osu_state = osu_state;

        let mut delta_wheel = 0f32;
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::Z),
                    repeat: false,
                    ..
                } => {
                    osu_state.set_key_state(OsuKey::Key1, true);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::X),
                    repeat: false,
                    ..
                } => {
                    osu_state.set_key_state(OsuKey::Key2, true);
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Z),
                    repeat: false,
                    ..
                } => {
                    osu_state.set_key_state(OsuKey::Key1, false);
                }
                Event::KeyUp {
                    keycode: Some(Keycode::X),
                    repeat: false,
                    ..
                } => {
                    osu_state.set_key_state(OsuKey::Key2, false);
                }
                Event::MouseWheel { precise_y, .. } => delta_wheel = precise_y,
                Event::KeyUp {
                    keycode: Some(keycode),
                    repeat: false,
                    ..
                } => {
                    if let Ok(ev) = KeyboardEventGroup::try_from(keycode) {
                        keyboard_state.update(ev, false);
                    }
                }
                Event::KeyDown {
                    keycode: Some(keycode),
                    repeat: false,
                    ..
                } => {
                    if let Ok(ev) = KeyboardEventGroup::try_from(keycode) {
                        keyboard_state.update(ev, true);
                    }
                }
                _ => {}
            };
        }
        let mouse_diff: sdl2::mouse::RelativeMouseState = event_pump.relative_mouse_state();
        let dx = mouse_diff.x() as f32 * config.mouse_accel;
        let dy = mouse_diff.y() as f32 * config.mouse_accel;

        mouse_state.dx = dx;
        mouse_state.dy = dy;
        mouse_state.dw = delta_wheel;
        mouse_state.set_button_state(MouseButton::Left, mouse_diff.left());
        mouse_state.set_button_state(MouseButton::Right, mouse_diff.right());
        mouse_state.set_button_state(MouseButton::Middle, mouse_diff.middle());

        if let Ok(client) = &mut client {
            if prev_keyboard_state != keyboard_state {
                let _ = client.send_event(EventType::Keyboard(keyboard_state));
            }

            if prev_mouse_state != mouse_state {
                let _ = client.send_event(EventType::Mouse(mouse_state));
            }

            if prev_osu_state != osu_state {
                let _ = client.send_event(EventType::Osu(osu_state));
            }
        }
        thread::sleep(Duration::from_millis(config.rate));
    }
}
