#[cfg(target_os = "linux")]
pub fn get_evdev_key(key_str: &str) -> Option<evdev::Key> {
    use evdev::Key;

    match key_str.trim().to_lowercase().as_str() {
        // Row numbers
        "1" => Some(Key::KEY_1),
        "2" => Some(Key::KEY_2),
        "3" => Some(Key::KEY_3),
        "4" => Some(Key::KEY_4),
        "5" => Some(Key::KEY_5),
        "6" => Some(Key::KEY_6),
        "7" => Some(Key::KEY_7),
        "8" => Some(Key::KEY_8),
        "9" => Some(Key::KEY_9),
        "0" => Some(Key::KEY_0),
        "-" | "minus" => Some(Key::KEY_MINUS),
        "=" | "equals" => Some(Key::KEY_EQUAL),

        // Row 1
        "q" => Some(Key::KEY_Q),
        "w" => Some(Key::KEY_W),
        "e" => Some(Key::KEY_E),
        "r" => Some(Key::KEY_R),
        "t" => Some(Key::KEY_T),
        "y" => Some(Key::KEY_Y),
        "u" => Some(Key::KEY_U),
        "i" => Some(Key::KEY_I),
        "o" => Some(Key::KEY_O),
        "p" => Some(Key::KEY_P),
        "[" => Some(Key::KEY_LEFTBRACE),
        "]" => Some(Key::KEY_RIGHTBRACE),

        // Row 2
        "a" => Some(Key::KEY_A),
        "s" => Some(Key::KEY_S),
        "d" => Some(Key::KEY_D),
        "f" => Some(Key::KEY_F),
        "g" => Some(Key::KEY_G),
        "h" => Some(Key::KEY_H),
        "j" => Some(Key::KEY_J),
        "k" => Some(Key::KEY_K),
        "l" => Some(Key::KEY_L),
        ";" | "semicol" => Some(Key::KEY_SEMICOLON),
        "'" => Some(Key::KEY_APOSTROPHE),
        "\\" | "backslash" => Some(Key::KEY_BACKSLASH),

        // Row 3
        "z" => Some(Key::KEY_Z),
        "x" => Some(Key::KEY_X),
        "c" => Some(Key::KEY_C),
        "v" => Some(Key::KEY_V),
        "b" => Some(Key::KEY_B),
        "n" => Some(Key::KEY_N),
        "m" => Some(Key::KEY_M),
        "," | "comma" => Some(Key::KEY_COMMA),
        "." | "dot" => Some(Key::KEY_DOT),
        "/" | "slash" => Some(Key::KEY_SLASH),
        _ => None,
    }
}
