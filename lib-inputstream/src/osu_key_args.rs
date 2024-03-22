use std::ops::Deref;

#[derive(Debug, Clone)]
pub struct OsuKey(String);

impl Deref for OsuKey {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<String> for OsuKey {
    fn from(value: String) -> Self {
        Self(value)
    }
}

#[cfg(target_os = "linux")]
impl TryFrom<OsuKey> for evdev::Key {
    type Error = String;
    fn try_from(value: OsuKey) -> Result<Self, Self::Error> {
        use evdev::Key;
        match value.0.as_str().trim().to_lowercase().as_str() {
            // Row numbers
            "1" => Ok(Key::KEY_1),
            "2" => Ok(Key::KEY_2),
            "3" => Ok(Key::KEY_3),
            "4" => Ok(Key::KEY_4),
            "5" => Ok(Key::KEY_5),
            "6" => Ok(Key::KEY_6),
            "7" => Ok(Key::KEY_7),
            "8" => Ok(Key::KEY_8),
            "9" => Ok(Key::KEY_9),
            "0" => Ok(Key::KEY_0),
            "-" | "minus" => Ok(Key::KEY_MINUS),
            "=" | "equals" => Ok(Key::KEY_EQUAL),

            // Row 1
            "q" => Ok(Key::KEY_Q),
            "w" => Ok(Key::KEY_W),
            "e" => Ok(Key::KEY_E),
            "r" => Ok(Key::KEY_R),
            "t" => Ok(Key::KEY_T),
            "y" => Ok(Key::KEY_Y),
            "u" => Ok(Key::KEY_U),
            "i" => Ok(Key::KEY_I),
            "o" => Ok(Key::KEY_O),
            "p" => Ok(Key::KEY_P),
            "[" => Ok(Key::KEY_LEFTBRACE),
            "]" => Ok(Key::KEY_RIGHTBRACE),

            // Row 2
            "a" => Ok(Key::KEY_A),
            "s" => Ok(Key::KEY_S),
            "d" => Ok(Key::KEY_D),
            "f" => Ok(Key::KEY_F),
            "g" => Ok(Key::KEY_G),
            "h" => Ok(Key::KEY_H),
            "j" => Ok(Key::KEY_J),
            "k" => Ok(Key::KEY_K),
            "l" => Ok(Key::KEY_L),
            ";" | "semicol" => Ok(Key::KEY_SEMICOLON),
            "'" => Ok(Key::KEY_APOSTROPHE),
            "\\" | "backslash" => Ok(Key::KEY_BACKSLASH),

            // Row 3
            "z" => Ok(Key::KEY_Z),
            "x" => Ok(Key::KEY_X),
            "c" => Ok(Key::KEY_C),
            "v" => Ok(Key::KEY_V),
            "b" => Ok(Key::KEY_B),
            "n" => Ok(Key::KEY_N),
            "m" => Ok(Key::KEY_M),
            "," | "comma" => Ok(Key::KEY_COMMA),
            "." | "dot" => Ok(Key::KEY_DOT),
            "/" | "slash" => Ok(Key::KEY_SLASH),
            _ => Err("Unknown token".to_string()),
        }
    }
}

#[cfg(target_os = "windows")]
impl TryFrom<OsuKey> for winapi::ctypes::c_int {
    type Error = String;
    fn try_from(v: OsuKey) -> Result<Self, Self::Error> {
        use winapi::um::winuser::*;
        match v.0.as_str() {
            // Row numbers
            "1" => Ok(0x31),
            "2" => Ok(0x32),
            "3" => Ok(0x33),
            "4" => Ok(0x34),
            "5" => Ok(0x35),
            "6" => Ok(0x36),
            "7" => Ok(0x37),
            "8" => Ok(0x38),
            "9" => Ok(0x39),
            "0" => Ok(0x30),
            "-" => Ok(VK_OEM_MINUS),
            "=" => Ok(VK_OEM_PLUS),

            // Row 1
            "q" => Ok(0x51),
            "w" => Ok(0x57),
            "e" => Ok(0x45),
            "r" => Ok(0x52),
            "t" => Ok(0x54),
            "y" => Ok(0x59),
            "u" => Ok(0x55),
            "i" => Ok(0x49),
            "o" => Ok(0x4F),
            "p" => Ok(0x50),
            "[" => Ok(VK_OEM_4),
            "]" => Ok(VK_OEM_6),

            // Row 2
            "a" => Ok(0x41),
            "s" => Ok(0x53),
            "d" => Ok(0x44),
            "f" => Ok(0x46),
            "g" => Ok(0x47),
            "h" => Ok(0x48),
            "j" => Ok(0x4A),
            "k" => Ok(0x4B),
            "l" => Ok(0x4C),
            ";" | "semicol" => Ok(VK_OEM_1),
            "'" => Ok(VK_OEM_7),
            "\\" | "backslash" => Ok(VK_OEM_5),

            // Row 3
            "z" => Ok(0x5A),
            "x" => Ok(0x58),
            "c" => Ok(0x43),
            "v" => Ok(0x56),
            "b" => Ok(0x42),
            "n" => Ok(0x4E),
            "m" => Ok(0x4D),
            "," | "comma" => Ok(VK_OEM_COMMA),
            "." | "dot" => Ok(VK_OEM_PERIOD),
            "/" | "slash" => Ok(VK_OEM_2),
            _ => Err("Unknown token".to_string()),
        }
    }
}
