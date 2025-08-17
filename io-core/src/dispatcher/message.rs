use crate::IORemoteResult;

pub type MessageKind = u16;

pub trait Message: Default + Clone + 'static {
    const KIND: MessageKind;

    fn serialize(&self, buffer: &mut Vec<u8>);
    fn deserialize(data: &[u8]) -> IORemoteResult<Self>;
}

pub struct MessageEncoder {
    buffer: Vec<u8>,
}

impl MessageEncoder {
    pub fn new() -> Self {
        Self {
            buffer: Vec::with_capacity(128),
        }
    }

    pub fn encode<M: Message>(&mut self, msg: M, buffer: &mut Vec<u8>) {
        self.buffer.clear();

        let kind = M::KIND;
        msg.serialize(&mut self.buffer);
        let payload_len = self.buffer.len() as u32;

        buffer.extend(kind.to_be_bytes());
        buffer.extend(payload_len.to_be_bytes());
        buffer.extend(&self.buffer);
    }
}
