use std::ops::Deref;

use crate::utils::map_value_to_new_range;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct AbsolutAxis(pub i8);

impl AbsolutAxis {
    pub fn from_normalized(val: impl Into<f32>) -> Self {
        let val: f32 = val.into();
        let val = match val {
            v if v < -1.0 => -1.0,
            v if v > 1.0 => 1.0,
            v => v,
        };
        let val = map_value_to_new_range(val, -1.0, 1.0, i8::MIN as f32, i8::MAX as f32) as i8;
        Self(val)
    }
}

#[cfg(feature = "sdl2")]
impl AbsolutAxis {
    pub fn from_sdl2_value_stick(val: i16) -> Self {
        Self(crate::utils::map_value_to_new_range(
            val as f32,
            -32768.0,
            32767.0,
            i8::MIN as f32,
            i8::MAX as f32,
        ) as i8)
    }
}

impl Deref for AbsolutAxis {
    type Target = i8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn maps_correctly_from_float() {
        let max = AbsolutAxis::from_normalized(1.0);
        let min = AbsolutAxis::from_normalized(-1.0);
        let zero = AbsolutAxis::from_normalized(0.0);

        assert_eq!(*max, 127);
        assert_eq!(*min, -128);
        assert_eq!(*zero, 0);
    }
}
