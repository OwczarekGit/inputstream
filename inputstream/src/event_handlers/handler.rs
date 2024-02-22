use std::thread;

use crate::Result;

pub trait EventHandler<T: Send + 'static> {
    fn run_detached(&'static self, receiver: std::sync::mpsc::Receiver<T>)
    where
        Self: 'static + Send + Sync,
    {
        thread::spawn(|| self.listen(receiver));
    }

    fn listen(&self, receiver: std::sync::mpsc::Receiver<T>) -> Result<()>;
}

pub struct OsuEventHandler;

pub struct MouseEventHandler;
pub struct KeyboardEventHandler;
