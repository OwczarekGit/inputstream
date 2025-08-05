use crate::IORemoteResult;

pub type MessageKind = u16;

pub trait Message: Default + Clone + 'static {
    const KIND: MessageKind;

    fn serialize(&self) -> Vec<u8>;
    fn deserialize(data: &[u8]) -> IORemoteResult<Self>;
}

pub struct MessageEncoder;

impl MessageEncoder {
    pub fn encode<M: Message>(msg: M, buffer: &mut Vec<u8>) {
        let kind = M::KIND;
        let payload = msg.serialize();
        let payload_len = payload.len() as u32;

        buffer.extend(kind.to_be_bytes());
        buffer.extend(payload_len.to_be_bytes());
        buffer.extend(payload);
    }
}
