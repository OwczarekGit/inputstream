use std::{
    any::TypeId,
    sync::{PoisonError, RwLockReadGuard, RwLockWriteGuard, mpsc::RecvError},
};

use crate::dispatcher::message::MessageKind;

pub type IORemoteResult<T> = Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    MessageTooShort(usize),
    MessagePayloadWrongLength { expected: usize, was: usize },
    RwLockPoisonWriteGuard,
    RwLockPoisonReadGuard,
    MessageHandlerAlreadyRegistered(TypeId),
    MissingListenerForMessageKind(MessageKind),
    MissingDispatcherForMessageKind(MessageKind),
    Recv(RecvError),
    Io(std::io::Error),
}

impl<T> From<PoisonError<RwLockWriteGuard<'_, T>>> for Error {
    fn from(_: PoisonError<RwLockWriteGuard<T>>) -> Self {
        Self::RwLockPoisonWriteGuard
    }
}

impl<T> From<PoisonError<RwLockReadGuard<'_, T>>> for Error {
    fn from(_: PoisonError<RwLockReadGuard<T>>) -> Self {
        Self::RwLockPoisonReadGuard
    }
}

impl From<RecvError> for Error {
    fn from(value: RecvError) -> Self {
        Self::Recv(value)
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}
