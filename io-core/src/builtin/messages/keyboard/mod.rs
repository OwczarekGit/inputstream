pub mod key;

use std::mem::transmute;

use crate::{
    IORemoteResult,
    builtin::messages::{KEYBOARD_MESSAGE_KIND, difference::Difference, keyboard::key::Key},
    dispatcher::message::{Message, MessageKind},
};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Keyboard(pub u32, pub u32, pub u32, pub u32);

impl Keyboard {
    pub fn set_state(&mut self, key: impl Into<Key>, state: impl Into<bool>) {
        let key = key.into() as u8;
        let key_wrapped = key % 32;

        let mask = |bit: u8, key_ref: &mut u32| {
            if state.into() {
                *key_ref |= 1 << bit;
            } else {
                *key_ref &= !(1 << bit);
            };
        };
        match key {
            0..32 => mask(key_wrapped, &mut self.0),
            32..64 => mask(key_wrapped, &mut self.1),
            64..96 => mask(key_wrapped, &mut self.2),
            96..128 => mask(key_wrapped, &mut self.3),
            other => panic!("Outside expected range: {other}."),
        }
    }

    pub fn get_state(&self, key: impl Into<Key>) -> bool {
        let key = key.into() as u8;
        let key_wrapped = key % 32;

        let is_pressed = |bit: u8, key_ref: &u32| -> bool { key_ref & (1 << bit) > 0 };

        match key {
            0..32 => is_pressed(key_wrapped, &self.0),
            32..64 => is_pressed(key_wrapped, &self.1),
            64..96 => is_pressed(key_wrapped, &self.2),
            96..128 => is_pressed(key_wrapped, &self.3),
            other => panic!("Outside expected range: {other}."),
        }
    }
}

impl Message for Keyboard {
    const KIND: MessageKind = KEYBOARD_MESSAGE_KIND;

    fn serialize(&self) -> Vec<u8> {
        let mut buf = Vec::<u8>::with_capacity(16);
        buf.extend_from_slice(&self.0.to_be_bytes());
        buf.extend_from_slice(&self.1.to_be_bytes());
        buf.extend_from_slice(&self.2.to_be_bytes());
        buf.extend_from_slice(&self.3.to_be_bytes());

        buf
    }

    fn deserialize(data: &[u8]) -> IORemoteResult<Self> {
        let b0 = u32::from_be_bytes(data[0..4].try_into().unwrap());
        let b1 = u32::from_be_bytes(data[4..8].try_into().unwrap());
        let b2 = u32::from_be_bytes(data[8..12].try_into().unwrap());
        let b3 = u32::from_be_bytes(data[12..16].try_into().unwrap());

        Ok(Self(b0, b1, b2, b3))
    }
}

impl Difference for Keyboard {
    type Diff = Vec<(Key, bool)>;

    fn get_diff(&self, other: &Self) -> Self::Diff {
        let mut results = vec![];

        for key in 0..87u8 {
            let k = unsafe { transmute::<u8, Key>(key) };

            if !self.get_state(k).eq(&other.get_state(k)) {
                results.push((k, self.get_state(k)));
            }
        }

        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! t {
        ($case:ident, $b1:expr, $b2:expr, $b3:expr, $b4:expr) => {
            #[test]
            fn $case() {
                let kbd = Keyboard($b1, $b2, $b3, $b4);
                let serialized = kbd.serialize();
                let deserialized = Keyboard::deserialize(&serialized).unwrap();

                assert_eq!(deserialized.0, $b1);
                assert_eq!(deserialized.1, $b2);
                assert_eq!(deserialized.2, $b3);
                assert_eq!(deserialized.3, $b4);
            }
        };
    }

    t!(
        keyboard_should_serialized_be_same_as_deserialized_case1,
        4123,
        45125162,
        1231256,
        1236673546
    );
    t!(
        keyboard_should_serialized_be_same_as_deserialized_case2,
        586790342,
        312907,
        14289774,
        34902785
    );
    t!(
        keyboard_should_serialized_be_same_as_deserialized_case3,
        489127,
        4901278,
        6590201,
        0
    );

    #[test]
    fn should_set_correct_bit_true() {
        let mut kbd = Keyboard(8, 8, 8, 8);
        kbd.set_state(Key::A, true);
        assert_eq!(kbd.0, 0b0000_0000_0000_1001);
        assert_eq!(kbd.1, 0b0000_0000_0000_1000);
        assert_eq!(kbd.2, 0b0000_0000_0000_1000);
        assert_eq!(kbd.3, 0b0000_0000_0000_1000);
    }

    #[test]
    fn should_set_correct_bit_false() {
        let mut kbd = Keyboard(8, 8, 8, 8);
        kbd.set_state(Key::D, false);
        assert_eq!(kbd.0, 0b0000_0000_0000_0000);
        assert_eq!(kbd.1, 0b0000_0000_0000_1000);
        assert_eq!(kbd.2, 0b0000_0000_0000_1000);
        assert_eq!(kbd.3, 0b0000_0000_0000_1000);
    }
}
