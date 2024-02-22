use std::str::FromStr;

use crate::consts::{KEYBOARD_PROTOCOL_NAME, MOUSE_PROTOCOL_NAME, OSU_PROTOCOL_NAME};

#[derive(Debug, Clone)]
pub enum InputEvent {
    Osu(OsuEvent),
    Keyboard(KeyboardEvent),
    Mouse(MouseEvent),
}

impl FromStr for InputEvent {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split('|');
        let protocol = split
            .next()
            .ok_or("Protocol not specified.".to_string())?
            .trim();

        match protocol {
            KEYBOARD_PROTOCOL_NAME => {
                let mut data = split.next().ok_or("Missing data.")?.split(';');

                let key_group1 = data
                    .next()
                    .ok_or("Missing keyboard group1.")?
                    .trim()
                    .parse::<u32>()
                    .unwrap_or(0);

                let key_group2 = data
                    .next()
                    .ok_or("Missing keyboard group1.")?
                    .trim()
                    .parse::<u32>()
                    .unwrap_or(0);

                let key_group3 = data
                    .next()
                    .ok_or("Missing keyboard group1.")?
                    .trim()
                    .parse::<u32>()
                    .unwrap_or(0);

                let key_group4 = data
                    .next()
                    .ok_or("Missing keyboard group1.")?
                    .trim()
                    .parse::<u32>()
                    .unwrap_or(0);

                Ok(Self::Keyboard(KeyboardEvent {
                    key_group1,
                    key_group2,
                    key_group3,
                    key_group4,
                }))
            }
            MOUSE_PROTOCOL_NAME => {
                let mut data = split.next().ok_or("Missing data.")?.split(';');

                let dx = data
                    .next()
                    .ok_or("Missing delta X.")?
                    .trim()
                    .parse::<f32>()
                    .unwrap_or(0.0);

                let dy = data
                    .next()
                    .ok_or("Missing delta Y.")?
                    .trim()
                    .parse::<f32>()
                    .unwrap_or(0.0);

                let dw = data
                    .next()
                    .ok_or("Missing delta mouse wheel.")?
                    .trim()
                    .parse::<f32>()
                    .unwrap_or(0.0);

                let buttons = data
                    .next()
                    .ok_or("Missing button state.")?
                    .trim()
                    .parse::<u32>()
                    .unwrap_or(0);

                Ok(Self::Mouse(MouseEvent {
                    dx,
                    dy,
                    dw,
                    btn_left_state: buttons & 0x01 > 0,
                    btn_right_state: buttons & 0x02 > 0,
                    btn_middle_state: buttons & 0x04 > 0,
                }))
            }
            OSU_PROTOCOL_NAME => {
                let data = split.next().ok_or("Missing data.".to_string())?.trim();
                let data = data
                    .parse::<u8>()
                    .map_err(|_| "Invalid data.".to_string())?;

                Ok(Self::Osu(OsuEvent {
                    key1_state: data & 0x01 > 0,
                    key2_state: data & 0x02 > 0,
                }))
            }
            unknown => Err(format!("Unknown protocol: {unknown}")),
        }
    }
}

#[derive(Debug, Clone)]
pub struct OsuEvent {
    pub key1_state: bool,
    pub key2_state: bool,
}

#[derive(Debug, Clone)]
pub struct MouseEvent {
    pub dx: f32,
    pub dy: f32,
    pub dw: f32,
    pub btn_left_state: bool,
    pub btn_right_state: bool,
    pub btn_middle_state: bool,
}

#[derive(Debug, Clone)]
pub struct KeyboardEvent {
    pub key_group1: u32,
    pub key_group2: u32,
    pub key_group3: u32,
    pub key_group4: u32,
}
