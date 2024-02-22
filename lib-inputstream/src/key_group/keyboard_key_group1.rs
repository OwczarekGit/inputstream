use std::fmt::Display;

use crate::key_state::KeyState;

/// Represents the first group of keys (A-Z).
#[derive(Debug, Clone)]
pub struct KeyboardKeyGroup1(Vec<KeyState>);

impl KeyboardKeyGroup1 {
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

impl Display for KeyboardKeyGroup1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = 0;
        for key in &self.0 {
            if key.pressed() {
                result |= key.bit();
            }
        }

        write!(f, "{result}")
    }
}

impl Default for KeyboardKeyGroup1 {
    fn default() -> Self {
        Self::new()
    }
}

impl From<&str> for KeyboardKeyGroup1 {
    fn from(v: &str) -> Self {
        let mut result = Self::new();
        let mask: u32 = v.parse().unwrap_or(0);
        result.check_states(mask);
        result
    }
}

#[cfg(target_os = "linux")]
impl From<KeyboardKeyGroup1> for Vec<(evdev::Key, bool)> {
    fn from(v: KeyboardKeyGroup1) -> Self {
        use evdev::Key;
        v.0.into_iter()
            .map(|k| match k.bit() {
                0 => (Key::KEY_A, k.pressed()),
                1 => (Key::KEY_B, k.pressed()),
                2 => (Key::KEY_C, k.pressed()),
                3 => (Key::KEY_D, k.pressed()),
                4 => (Key::KEY_E, k.pressed()),
                5 => (Key::KEY_F, k.pressed()),
                6 => (Key::KEY_G, k.pressed()),
                7 => (Key::KEY_H, k.pressed()),
                8 => (Key::KEY_I, k.pressed()),
                9 => (Key::KEY_J, k.pressed()),
                10 => (Key::KEY_K, k.pressed()),
                11 => (Key::KEY_L, k.pressed()),
                12 => (Key::KEY_M, k.pressed()),
                13 => (Key::KEY_N, k.pressed()),
                14 => (Key::KEY_O, k.pressed()),
                15 => (Key::KEY_P, k.pressed()),
                16 => (Key::KEY_Q, k.pressed()),
                17 => (Key::KEY_R, k.pressed()),
                18 => (Key::KEY_S, k.pressed()),
                19 => (Key::KEY_T, k.pressed()),
                20 => (Key::KEY_U, k.pressed()),
                21 => (Key::KEY_V, k.pressed()),
                22 => (Key::KEY_W, k.pressed()),
                23 => (Key::KEY_X, k.pressed()),
                24 => (Key::KEY_Y, k.pressed()),
                25 => (Key::KEY_Z, k.pressed()),
                _ => (Key::KEY_A, false),
            })
            .collect::<Vec<_>>()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn gets_correct_string() {
        let mut mask = 0b0000_0000;
        let mut group = KeyboardKeyGroup1::new();

        let changes = group.check_states(mask);
        assert!(changes.is_empty());
        mask = 0b0000_1010;

        let changes = group.check_states(mask);
        assert_eq!(changes.len(), 2);
        assert_eq!(group.to_string(), "10");

        mask = 0b0001_1010;
        let changes = group.check_states(mask);
        assert_eq!(changes.len(), 1);
        assert_eq!(group.to_string(), "26");
    }

    #[test]
    fn parses_correctly_from_str() {
        let mask = "26";
        let states = KeyboardKeyGroup1::from(mask);
        assert_eq!(states.to_string(), "26");
    }
}
