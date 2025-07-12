pub type AppRes<T> = Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    IORemote(io_core::Error),
    Io(std::io::Error),
}

impl From<io_core::Error> for Error {
    fn from(value: io_core::Error) -> Self {
        Self::IORemote(value)
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}
