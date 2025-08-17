pub mod listener;
pub mod message;

use crate::{Error, IORemoteResult};
use listener::{AnyListener, AnyListenerImpl, Listener};
use message::{Message, MessageKind};
use std::{
    any::TypeId,
    collections::HashMap,
    sync::{Arc, RwLock},
};

#[derive(Debug, Clone)]
pub struct DecodedMessage {
    pub kind: MessageKind,
    pub payload: Vec<u8>,
}

pub struct MessageDecoder;

impl MessageDecoder {
    fn decode(msg: &[u8]) -> IORemoteResult<DecodedMessage> {
        if msg.len() < 6 {
            return Err(Error::MessageTooShort(msg.len()));
        }

        let kind = MessageKind::from_be_bytes([msg[0], msg[1]]);
        let len = u32::from_be_bytes([msg[2], msg[3], msg[4], msg[5]]) as usize;

        if msg.len() < 6 + len {
            return Err(Error::MessagePayloadWrongLength {
                expected: len,
                was: msg.len(),
            });
        }

        let payload = msg[6..6 + len].to_vec();

        Ok(DecodedMessage { kind, payload })
    }
}

type DispatchFn = fn(&[u8], &Box<dyn AnyListener>) -> IORemoteResult<()>;

#[derive(Default)]
pub struct DispatcherState {
    listeners: HashMap<TypeId, Box<dyn AnyListener>>,
    dispatchers: HashMap<MessageKind, (TypeId, DispatchFn)>,
}

#[derive(Default, Clone)]
pub struct Dispatcher {
    state: Arc<RwLock<DispatcherState>>,
}

impl Dispatcher {
    pub fn register_listener<M: Message>(
        &self,
        listener: Box<dyn Listener<M>>,
    ) -> IORemoteResult<()> {
        self.register_message::<M>()?;

        let mut state = self.state.write()?;

        if state.listeners.contains_key(&TypeId::of::<M>()) {
            return Err(Error::MessageHandlerAlreadyRegistered(TypeId::of::<M>()));
        }

        state.listeners.insert(
            TypeId::of::<M>(),
            Box::new(AnyListenerImpl { inner: listener }),
        );

        Ok(())
    }

    pub fn dispatch(&self, msg: &[u8]) -> IORemoteResult<()> {
        let DecodedMessage { kind, payload } = MessageDecoder::decode(msg)?;

        let state = self.state.read()?;

        let Some((type_id, dispatch_fn)) = state.dispatchers.get(&kind) else {
            return Err(Error::MissingDispatcherForMessageKind(kind));
        };

        let Some(listener) = state.listeners.get(type_id) else {
            return Err(Error::MissingListenerForMessageKind(kind));
        };

        dispatch_fn(&payload, listener)
    }

    fn register_message<M: Message + 'static>(&self) -> IORemoteResult<()> {
        fn dispatch_fn<M: Message>(
            data: &[u8],
            listener: &Box<dyn AnyListener>,
        ) -> IORemoteResult<()> {
            let msg = M::deserialize(data)?;

            let Some(conc) = listener.as_any().downcast_ref::<Box<dyn Listener<M>>>() else {
                return Err(Error::RwLockPoisonReadGuard);
            };

            conc.dispatch(msg.clone());
            Ok(())
        }

        let kind = M::KIND;
        let mut state = self.state.write()?;
        state
            .dispatchers
            .insert(kind, (TypeId::of::<M>(), dispatch_fn::<M>));

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{builtin::messages::mouse::Mouse, dispatcher::message::MessageEncoder};

    use super::*;

    #[test]
    fn encoded_message_is_decoded_correctly() {
        let mut buffer = vec![];
        let mut enc = MessageEncoder::new();
        enc.encode(Mouse(11.0, -34.2, 22.2, 123), &mut buffer);
        let dec = MessageDecoder::decode(&buffer).unwrap();
        let decoded = Mouse::deserialize(&dec.payload).unwrap();
        assert_eq!(decoded.0, 11.0);
        assert_eq!(decoded.1, -34.2);
        assert_eq!(decoded.2, 22.2);
        assert_eq!(decoded.3, 123);
    }
}
