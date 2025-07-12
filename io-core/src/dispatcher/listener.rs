use super::Message;
use std::any::Any;

pub trait Listener<M: Message>: Send + Sync {
    fn dispatch(&self, msg: M);
}

pub(crate) trait AnyListener: Send + Sync {
    fn as_any(&self) -> &dyn Any;
}

pub(crate) struct AnyListenerImpl<M: Message> {
    pub inner: Box<dyn Listener<M>>,
}

impl<M: Message> AnyListener for AnyListenerImpl<M> {
    fn as_any(&self) -> &dyn Any {
        &self.inner
    }
}
