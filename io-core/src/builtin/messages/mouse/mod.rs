pub mod button;
use button::Button;

use crate::{
    IORemoteResult,
    builtin::messages::{MOUSE_MESSAGE_KIND, difference::Difference},
    dispatcher::message::{Message, MessageKind},
};

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Mouse(pub f32, pub f32, pub f32, pub u8);

impl Mouse {
    pub fn set_button(&mut self, button: impl Into<Button>, state: impl Into<bool>) {
        let state = state.into();
        let bit = button.into().bit();

        if state {
            self.3 |= 1 << bit;
        } else {
            self.3 &= !(1 << bit);
        }
    }

    pub fn get_button(&self, button: impl Into<Button>) -> bool {
        let bit = button.into().bit();
        self.3 & (1 << bit) > 0
    }
}

impl Message for Mouse {
    const KIND: MessageKind = MOUSE_MESSAGE_KIND;

    fn serialize(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(13);
        buf.extend(&self.0.to_bits().to_be_bytes());
        buf.extend(&self.1.to_bits().to_be_bytes());
        buf.extend(&self.2.to_bits().to_be_bytes());
        buf.push(self.3);
        buf
    }

    fn deserialize(data: &[u8]) -> IORemoteResult<Self> {
        let x: f32 = f32::from_bits(u32::from_be_bytes(data[0..4].try_into().unwrap()));
        let y: f32 = f32::from_bits(u32::from_be_bytes(data[4..8].try_into().unwrap()));
        let w: f32 = f32::from_bits(u32::from_be_bytes(data[8..12].try_into().unwrap()));
        let b: u8 = data[12];

        Ok(Self(x, y, w, b))
    }
}

impl Difference for Mouse {
    type Diff = (Option<f32>, Option<f32>, Option<f32>, Vec<(Button, bool)>);

    fn get_diff(&self, other: &Self) -> Self::Diff {
        let mut button_diff = Vec::with_capacity(5);

        let mut check_button = |btn: Button| {
            if !self.get_button(btn).eq(&other.get_button(btn)) {
                button_diff.push((btn, self.get_button(btn)));
            }
        };

        check_button(Button::Left);
        check_button(Button::Right);
        check_button(Button::Middle);
        check_button(Button::Mouse4);
        check_button(Button::Mouse5);

        (
            (!self.0.eq(&other.0)).then_some(self.0),
            (!self.1.eq(&other.1)).then_some(self.1),
            (!self.2.eq(&other.2)).then_some(self.2),
            button_diff,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! t {
        ($case:ident, $x:expr, $y:expr, $w:expr, $b:expr) => {
            #[test]
            fn $case() {
                let m = Mouse($x, $y, $w, $b);
                let serialized = m.serialize();
                let deserialized = Mouse::deserialize(&serialized).unwrap();

                assert_eq!(deserialized.0, $x);
                assert_eq!(deserialized.1, $y);
                assert_eq!(deserialized.2, $w);
                assert_eq!(deserialized.3, $b);
            }
        };
    }

    t!(
        mouse_should_serialized_be_same_as_deserialized_case1,
        2137.69f32,
        420.34,
        321.34,
        0b0110_1010
    );
    t!(
        mouse_should_serialized_be_same_as_deserialized_case2,
        213_151.13_f32,
        1_232_312.5,
        -123.0,
        213
    );
    t!(
        mouse_should_serialized_be_same_as_deserialized_case3,
        1111112f32,
        123123f32,
        123.0,
        22
    );

    #[test]
    fn sets_button_as_expected_case_left() {
        let mut m = Mouse(0.0, 0.0, 0.0, 255);
        m.set_button(Button::Left, false);
        assert_eq!(m.3, 0b1111_1110);
    }

    #[test]
    fn sets_button_as_expected_case_right() {
        let mut m = Mouse(0.0, 0.0, 0.0, 0);
        m.set_button(Button::Right, true);
        assert_eq!(m.3, 0b0000_0010);
    }

    #[test]
    fn sets_button_as_expected_case_middle() {
        let mut m = Mouse(0.0, 0.0, 0.0, 0b1111_0000);
        m.set_button(Button::Middle, true);
        assert_eq!(m.3, 0b1111_0100);
    }
}
