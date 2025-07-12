pub mod gamepad_button;

use std::mem::transmute;

use crate::{
    IORemoteResult,
    builtin::messages::{
        GAMEPAD_MESSAGE_KIND, difference::Difference, gamepad::gamepad_button::GamepadButton,
    },
    dispatcher::message::{Message, MessageKind},
};

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Gamepad {
    pub xl: i8,
    pub yl: i8,
    pub xr: i8,
    pub yr: i8,
    pub zl: u8,
    pub zr: u8,
    buttons: u32,
}

impl Gamepad {
    pub fn set_button(&mut self, button: impl Into<GamepadButton>, state: impl Into<bool>) {
        let state = state.into();
        let bit = button.into().bit();

        if state {
            self.buttons |= 1 << bit;
        } else {
            self.buttons &= !(1 << bit);
        }
    }
    pub fn get_button(&self, button: impl Into<GamepadButton>) -> bool {
        let bit = button.into().bit();
        self.buttons & (1 << bit) > 0
    }
}

impl Message for Gamepad {
    const KIND: MessageKind = GAMEPAD_MESSAGE_KIND;

    fn serialize(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(10);
        buf.extend(self.xl.to_be_bytes());
        buf.extend(self.yl.to_be_bytes());
        buf.extend(self.xr.to_be_bytes());
        buf.extend(self.yr.to_be_bytes());
        buf.extend(self.zl.to_be_bytes());
        buf.extend(self.zr.to_be_bytes());
        buf.extend_from_slice(&self.buttons.to_be_bytes());
        buf
    }

    fn deserialize(data: &[u8]) -> IORemoteResult<Self> {
        let xl = u8::cast_signed(data[0]);
        let yl = u8::cast_signed(data[1]);
        let xr = u8::cast_signed(data[2]);
        let yr = u8::cast_signed(data[3]);
        let zl = data[4];
        let zr = data[5];
        let buttons = u32::from_be_bytes(data[6..10].try_into().unwrap());

        Ok(Self {
            xl,
            yl,
            xr,
            yr,
            zl,
            zr,
            buttons,
        })
    }
}

impl Difference for Gamepad {
    type Diff = (
        Option<i8>,
        Option<i8>,
        Option<i8>,
        Option<i8>,
        Option<u8>,
        Option<u8>,
        Vec<(GamepadButton, bool)>,
    );

    fn get_diff(&self, other: &Self) -> Self::Diff {
        let mut results = vec![];

        for key in 0..16 {
            let k = unsafe { transmute::<u8, GamepadButton>(key) };

            if !self.get_button(k).eq(&other.get_button(k)) {
                results.push((k, self.get_button(k)));
            }
        }

        (
            (!self.xl.eq(&other.xl)).then_some(self.xl),
            (!self.yl.eq(&other.yl)).then_some(self.yl),
            (!self.xr.eq(&other.xr)).then_some(self.xr),
            (!self.yr.eq(&other.yr)).then_some(self.yr),
            (!self.zl.eq(&other.zl)).then_some(self.zl),
            (!self.zr.eq(&other.zr)).then_some(self.zr),
            results,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! t {
        ($case:ident,
            $xl:expr, $yl:expr,
            $xr:expr, $yr:expr,
            $zl:expr, $zr:expr,
            $btn:expr
        ) => {
            #[test]
            fn $case() {
                let gp = Gamepad {
                    xl: $xl,
                    yl: $yl,
                    xr: $xr,
                    yr: $yr,
                    zl: $zl,
                    zr: $zr,
                    buttons: $btn,
                };
                let serialized = gp.serialize();
                let deserialized = Gamepad::deserialize(&serialized).unwrap();

                assert_eq!(deserialized.xl, $xl);
                assert_eq!(deserialized.yl, $yl);
                assert_eq!(deserialized.xr, $xr);
                assert_eq!(deserialized.yr, $yr);
                assert_eq!(deserialized.zl, $zl);
                assert_eq!(deserialized.zr, $zr);
                assert_eq!(deserialized.buttons, $btn);
            }
        };
    }

    t!(
        gamepad_should_serialized_be_same_as_deserialized_case1,
        -121,
        123,
        -22,
        71,
        90,
        89,
        32000
    );
    t!(
        gamepad_should_serialized_be_same_as_deserialized_case2,
        1,
        1,
        1,
        -1,
        123,
        21,
        123
    );
    t!(
        gamepad_should_serialized_be_same_as_deserialized_case3,
        72,
        -123,
        -1,
        -0,
        0,
        22,
        0
    );
}
