use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum InputEvent {
    Osu(OsuEvent),
    Keyboard,
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
            "KEYBOARD" => Ok(Self::Keyboard),
            "MOUSE" => {
                let mut data = split.next().ok_or("Missing data.")?.split(';');
                let dx = data.next().ok_or("Missing delta X.")?;
                let dy = data.next().ok_or("Missing delta Y.")?;

                Ok(Self::Mouse(MouseEvent {
                    btn_left_state: true,
                    btn_right_state: true,
                    btn_middle_state: true,
                }))
            }
            "OSU" => {
                let data = split.next().ok_or("Missing data.".to_string())?.trim();
                let data = data
                    .parse::<u8>()
                    .map_err(|_| "Invalid data.".to_string())?;

                Ok(Self::Osu(OsuEvent {
                    key1_state: data & 0b0000_0001 > 0,
                    key2_state: data & 0b0000_0010 > 0,
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
    pub btn_left_state: bool,
    pub btn_right_state: bool,
    pub btn_middle_state: bool,
}
