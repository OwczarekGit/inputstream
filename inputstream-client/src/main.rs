use std::{thread, time::Duration};
mod config;

use clap::Parser;
use config::Config;
use lib_inputstream::{
    event::gamepad::absolute_axis::AbsoluteAxis, prelude::*, utils::sdl2_to_dualsense_triggers,
};

use sdl2::{controller::Axis, event::Event, keyboard::Keycode, pixels::Color};

pub fn main() {
    let config = Config::parse();

    let mut client = Client::new(config.address, config.port);

    let ctx = sdl2::init().unwrap();

    // When GameController instance falls off the stack the gamepad stops working
    // so we hold it as an Option
    let _controller = if !config.disable_gamepad {
        let game_controller_subsystem = ctx.game_controller().unwrap();

        let available = game_controller_subsystem
            .num_joysticks()
            .map_err(|e| dbg!(e))
            .unwrap();

        (0..available).find_map(|id| {
            if !game_controller_subsystem.is_game_controller(id) {
                return None;
            }

            match game_controller_subsystem.open(id) {
                Ok(c) => {
                    // We managed to find and open a game controller,
                    // exit the loop
                    // println!("Success: opened \"{}\"", c.name());
                    Some(c)
                }
                Err(e) => {
                    println!("failed: {:?}", e);
                    None
                }
            }
        })
    } else {
        None
    };

    let video_subsystem = ctx.video().unwrap();

    let window = video_subsystem
        .window("InputStream SDL2 client", 400, 300)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    if !config.disable_mouse {
        ctx.mouse().set_relative_mouse_mode(true);
        ctx.mouse().show_cursor(true);
    }

    canvas.set_draw_color(Color::RGB(0x44, 0x44, 0x44));
    canvas.clear();
    canvas.present();

    let mut keyboard_state = KeyboardEvent::default();
    let mut mouse_state = MouseEvent::default();
    let mut osu_state = OsuEvent::default();
    let mut gamepad_state = GamepadState::default();

    let mut event_pump = ctx.event_pump().unwrap();
    'running: loop {
        let prev_keyboard_state = keyboard_state;
        let prev_mouse_state = mouse_state;
        let prev_osu_state = osu_state;
        let prev_gamepad_state = gamepad_state;

        let mut delta_wheel = 0f32;
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,

                // Gamepad
                // Triggers
                Event::ControllerAxisMotion {
                    axis: Axis::TriggerLeft,
                    value,
                    ..
                } => gamepad_state.triggers.0 = sdl2_to_dualsense_triggers(value),
                Event::ControllerAxisMotion {
                    axis: Axis::TriggerRight,
                    value,
                    ..
                } => gamepad_state.triggers.1 = sdl2_to_dualsense_triggers(value),

                // Left stick
                Event::ControllerAxisMotion {
                    axis: Axis::LeftX,
                    value,
                    ..
                } => gamepad_state.left_stick.0 = *AbsoluteAxis::from_sdl2_value_stick(value),
                Event::ControllerAxisMotion {
                    axis: Axis::LeftY,
                    value,
                    ..
                } => gamepad_state.left_stick.1 = *AbsoluteAxis::from_sdl2_value_stick(value),

                // Right stick
                Event::ControllerAxisMotion {
                    axis: Axis::RightX,
                    value,
                    ..
                } => gamepad_state.right_stick.0 = *AbsoluteAxis::from_sdl2_value_stick(value),
                Event::ControllerAxisMotion {
                    axis: Axis::RightY,
                    value,
                    ..
                } => gamepad_state.right_stick.1 = *AbsoluteAxis::from_sdl2_value_stick(value),

                // Buttons
                Event::ControllerButtonDown { button, .. } => {
                    if let Ok(ev) = GamepadButton::try_from(button) {
                        gamepad_state.set_button_state(ev, true);
                    }
                }
                Event::ControllerButtonUp { button, .. } => {
                    if let Ok(ev) = GamepadButton::try_from(button) {
                        gamepad_state.set_button_state(ev, false);
                    }
                }

                // Keyboard
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

                // Mouse
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

            if prev_mouse_state != mouse_state && !config.disable_mouse {
                let _ = client.send_event(EventType::Mouse(mouse_state));
            }

            if prev_osu_state != osu_state {
                let _ = client.send_event(EventType::Osu(osu_state));
            }

            if prev_gamepad_state != gamepad_state && !config.disable_gamepad {
                let _ = client.send_event(EventType::Gamepad(gamepad_state));
            }
        }
        thread::sleep(Duration::from_millis(config.rate));
    }
}
