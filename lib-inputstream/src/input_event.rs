use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum InputEvent {
    Osu(OsuEvent),
    Keyboard,
    Mouse,
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
            "MOUSE" => Ok(Self::Mouse),
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
