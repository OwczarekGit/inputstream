use std::{io::Write, net::TcpStream, thread, time::Duration};

use clap::Parser;
use io_core::{
    builtin::messages::{gamepad::Gamepad, keyboard::Keyboard, mouse::Mouse},
    dispatcher::message::MessageEncoder,
    features::sdl3::{map_sdl3_axis_to_gamepad, map_sdl3_trigger_to_gamepad},
};
use sdl3::{event::Event, gamepad::Axis, pixels::Color};

use crate::arguments::Arguments;

mod arguments;

const WINDOW_TITLE: &str = "IO-Remote Desktop Client";

fn main() {
    let config = Arguments::parse();

    let addr = format!("{}:{}", config.address, config.port);
    let mut client = TcpStream::connect(addr).unwrap();

    let ctx = sdl3::init().unwrap();
    let mouse_util = ctx.mouse();
    let gamepad_sub = ctx.gamepad();

    let _gamepads = gamepad_sub.map(|sub| {
        sub.gamepads()
            .unwrap()
            .iter()
            .filter_map(|c| sub.open(*c).ok())
            .collect::<Vec<_>>()
    });

    let video = ctx.video().unwrap();

    let window = video
        .window(WINDOW_TITLE, 200, 200)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas();

    canvas.window_mut().set_mouse_grab(true);
    mouse_util.set_relative_mouse_mode(canvas.window(), true);

    canvas.set_draw_color(Color::RGB(22, 22, 22));
    canvas.clear();
    canvas.present();

    let mut ev_pump = ctx.event_pump().unwrap();

    let mut mouse = Mouse::default();
    let mut new_mouse = Mouse::default();

    let mut keyboard = Keyboard::default();
    let mut new_keyboard = Keyboard::default();

    let mut gamepad = Gamepad::default();
    let mut new_gamepad = Gamepad::default();

    'running: loop {
        for ev in ev_pump.poll_iter() {
            match ev {
                Event::Quit { .. } => break 'running,
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => {
                    new_keyboard.set_state(keycode, true);
                }
                Event::KeyUp {
                    keycode: Some(keycode),
                    ..
                } => {
                    new_keyboard.set_state(keycode, false);
                }
                Event::MouseMotion { xrel, yrel, .. } => {
                    new_mouse.0 = xrel;
                    new_mouse.1 = yrel;
                }
                Event::MouseButtonDown { mouse_btn, .. } => {
                    new_mouse.set_button(mouse_btn, true);
                }
                Event::MouseButtonUp { mouse_btn, .. } => {
                    new_mouse.set_button(mouse_btn, false);
                }
                Event::MouseWheel { y, .. } => {
                    new_mouse.2 = y;
                }
                Event::ControllerAxisMotion { axis, value, .. } => match axis {
                    Axis::LeftX => new_gamepad.xl = map_sdl3_axis_to_gamepad(value),
                    Axis::LeftY => new_gamepad.yl = map_sdl3_axis_to_gamepad(value),
                    Axis::RightX => new_gamepad.xr = map_sdl3_axis_to_gamepad(value),
                    Axis::RightY => new_gamepad.yr = map_sdl3_axis_to_gamepad(value),
                    Axis::TriggerLeft => new_gamepad.zl = map_sdl3_trigger_to_gamepad(value),
                    Axis::TriggerRight => new_gamepad.zr = map_sdl3_trigger_to_gamepad(value),
                },
                Event::ControllerButtonUp { button, .. } => {
                    new_gamepad.set_button(button, false);
                }
                Event::ControllerButtonDown { button, .. } => {
                    new_gamepad.set_button(button, true);
                }
                _ => {}
            }
        }

        if !new_mouse.eq(&mouse) {
            _ = client.write(&MessageEncoder::encode(new_mouse));
        }

        if !new_keyboard.eq(&keyboard) {
            _ = client.write(&MessageEncoder::encode(new_keyboard));
        }

        if !new_gamepad.eq(&gamepad) {
            _ = client.write(&MessageEncoder::encode(new_gamepad));
        }

        mouse = new_mouse;
        keyboard = new_keyboard;
        gamepad = new_gamepad;

        thread::sleep(Duration::from_millis(4));
    }
}
