use crate::IORemoteResult;

pub type MessageKind = u16;

pub trait Message: Default + Clone + 'static {
    const KIND: MessageKind;

    fn serialize(&self) -> Vec<u8>;
    fn deserialize(data: &[u8]) -> IORemoteResult<Self>;
}

pub struct MessageEncoder;

impl MessageEncoder {
    pub fn encode<M: Message>(msg: M) -> Vec<u8> {
        let kind = M::KIND;
        let payload = msg.serialize();
        let payload_len = payload.len() as u32;

        let mut out = Vec::with_capacity(6 + payload.len());
        out.extend(kind.to_be_bytes());
        out.extend(payload_len.to_be_bytes());
        out.extend(payload);

        out
    }
}
