use std::fmt::Display;

use crate::key_state::KeyState;

#[derive(Debug, Clone)]
pub struct KeyboardKeyGroup3(Vec<KeyState>);

impl KeyboardKeyGroup3 {
    pub fn new() -> Self {
        Self(vec![
            KeyState::new(0),
            KeyState::new(1),
            KeyState::new(2),
            KeyState::new(3),
            KeyState::new(4),
            KeyState::new(5),
            KeyState::new(6),
            KeyState::new(7),
            KeyState::new(8),
            KeyState::new(9),
            KeyState::new(10),
            KeyState::new(11),
            KeyState::new(12),
            KeyState::new(13),
            KeyState::new(14),
            KeyState::new(15),
            KeyState::new(16),
            KeyState::new(17),
            KeyState::new(18),
            KeyState::new(19),
            KeyState::new(20),
            KeyState::new(21),
            KeyState::new(22),
            KeyState::new(23),
            KeyState::new(24),
            KeyState::new(25),
            KeyState::new(26),
            KeyState::new(27),
            KeyState::new(28),
            KeyState::new(29),
            KeyState::new(30),
            KeyState::new(31),
        ])
    }

    pub fn check_states(&mut self, mask: u32) -> Vec<KeyState> {
        let mut changes = vec![];
        for key in &mut self.0 {
            if key.get_state(mask).is_some() {
                changes.push(key.clone());
            }
        }
        changes
    }
}

impl Display for KeyboardKeyGroup3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = 0;
        for key in &self.0 {
            if key.pressed() {
                result |= 1 << key.bit();
            }
        }

        write!(f, "{result}")
    }
}

impl Default for KeyboardKeyGroup3 {
    fn default() -> Self {
        Self::new()
    }
}

impl From<&str> for KeyboardKeyGroup3 {
    fn from(v: &str) -> Self {
        let mut result = Self::new();
        let mask: u32 = v.parse().unwrap_or(0);
        result.check_states(mask);
        result
    }
}

#[cfg(target_os = "linux")]
impl From<KeyboardKeyGroup3> for Vec<(evdev::Key, bool)> {
    fn from(v: KeyboardKeyGroup3) -> Self {
        use evdev::Key;
        v.0.into_iter()
            .filter(|k| k.should_fire())
            .map(|k| match k.bit() {
                0 => (Key::KEY_F1, k.pressed()),
                1 => (Key::KEY_F2, k.pressed()),
                2 => (Key::KEY_F3, k.pressed()),
                3 => (Key::KEY_F4, k.pressed()),
                4 => (Key::KEY_F5, k.pressed()),
                5 => (Key::KEY_F6, k.pressed()),
                6 => (Key::KEY_F7, k.pressed()),
                7 => (Key::KEY_F8, k.pressed()),
                8 => (Key::KEY_F9, k.pressed()),
                9 => (Key::KEY_F10, k.pressed()),
                10 => (Key::KEY_F11, k.pressed()),
                11 => (Key::KEY_F12, k.pressed()),
                12 => (Key::KEY_LEFT, k.pressed()),
                13 => (Key::KEY_RIGHT, k.pressed()),
                14 => (Key::KEY_UP, k.pressed()),
                15 => (Key::KEY_DOWN, k.pressed()),
                16 => (Key::KEY_INSERT, k.pressed()),
                17 => (Key::KEY_DELETE, k.pressed()),
                18 => (Key::KEY_HOME, k.pressed()),
                19 => (Key::KEY_END, k.pressed()),
                20 => (Key::KEY_PAGEUP, k.pressed()),
                21 => (Key::KEY_PAGEDOWN, k.pressed()),
                22 => (Key::KEY_NUMERIC_0, k.pressed()),
                23 => (Key::KEY_NUMERIC_1, k.pressed()),
                24 => (Key::KEY_NUMERIC_2, k.pressed()),
                25 => (Key::KEY_NUMERIC_3, k.pressed()),
                26 => (Key::KEY_NUMERIC_4, k.pressed()),
                27 => (Key::KEY_NUMERIC_5, k.pressed()),
                28 => (Key::KEY_NUMERIC_6, k.pressed()),
                29 => (Key::KEY_NUMERIC_7, k.pressed()),
                30 => (Key::KEY_NUMERIC_8, k.pressed()),
                31 => (Key::KEY_NUMERIC_9, k.pressed()),
                key => panic!("INVALID KEY: '{key}'"),
            })
            .collect::<Vec<_>>()
    }
}
