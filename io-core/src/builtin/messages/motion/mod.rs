use crate::{
    IORemoteResult,
    builtin::messages::{MOTION_MESSAGE_KIND, difference::Difference},
    dispatcher::message::{Message, MessageKind},
};

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Motion {
    pub x: i16,
    pub y: i16,
    pub z: i16,
    pub ax: i32,
    pub ay: i32,
    pub az: i32,
}

impl Message for Motion {
    const KIND: MessageKind = MOTION_MESSAGE_KIND;

    fn serialize(&self, buf: &mut Vec<u8>) {
        buf.extend(self.x.to_be_bytes());
        buf.extend(self.y.to_be_bytes());
        buf.extend(self.z.to_be_bytes());
        buf.extend(self.ax.to_be_bytes());
        buf.extend(self.ay.to_be_bytes());
        buf.extend(self.az.to_be_bytes());
    }

    fn deserialize(data: &[u8]) -> IORemoteResult<Self> {
        let x = i16::from_be_bytes(data[0..2].try_into().unwrap());
        let y = i16::from_be_bytes(data[2..4].try_into().unwrap());
        let z = i16::from_be_bytes(data[4..6].try_into().unwrap());
        let ax = i32::from_be_bytes(data[6..10].try_into().unwrap());
        let ay = i32::from_be_bytes(data[10..14].try_into().unwrap());
        let az = i32::from_be_bytes(data[14..18].try_into().unwrap());

        Ok(Self {
            x,
            y,
            z,
            ax,
            ay,
            az,
        })
    }
}

impl Difference for Motion {
    type Diff = (
        Option<i16>,
        Option<i16>,
        Option<i16>,
        Option<i32>,
        Option<i32>,
        Option<i32>,
    );

    fn get_diff(&self, other: &Self) -> Self::Diff {
        (
            (!self.x.eq(&other.x)).then_some(self.x),
            (!self.y.eq(&other.y)).then_some(self.y),
            (!self.z.eq(&other.z)).then_some(self.z),
            (!self.ax.eq(&other.ax)).then_some(self.ax),
            (!self.ax.eq(&other.ay)).then_some(self.ay),
            (!self.ax.eq(&other.az)).then_some(self.az),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! t {
        ($case:ident,
            $x:expr, $y:expr,
            $z:expr, $ax:expr,
            $ay:expr, $az:expr,
        ) => {
            #[test]
            fn $case() {
                let mt = Motion {
                    x: $x,
                    y: $y,
                    z: $z,
                    ax: $ax,
                    ay: $ay,
                    az: $az,
                };
                let mut buff = vec![];
                mt.serialize(&mut buff);
                let deserialized = Motion::deserialize(&buff).unwrap();

                assert_eq!(deserialized.x, $x);
                assert_eq!(deserialized.y, $y);
                assert_eq!(deserialized.z, $z);
                assert_eq!(deserialized.ax, $ax);
                assert_eq!(deserialized.ay, $ay);
                assert_eq!(deserialized.az, $az);
            }
        };
    }

    t!(
        motion_should_serialized_be_same_as_deserialized_case1,
        781,
        -321,
        0,
        -213,
        -1234412,
        211,
    );
    t!(
        motion_should_serialized_be_same_as_deserialized_case2,
        312,
        -31214,
        412,
        -1245,
        412,
        231,
    );
    t!(
        motion_should_serialized_be_same_as_deserialized_case3,
        51,
        -2515,
        -12455,
        2315241,
        123124,
        2123,
    );
}
