pub type AppRes<T> = Result<T, Error>;

#[allow(dead_code)]
#[derive(Debug)]
pub enum Error {
    IORemote(io_core::Error),
    Io(std::io::Error),
    Sdl3(sdl3::Error),
    WindowBuild(sdl3::video::WindowBuildError),
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

impl From<sdl3::Error> for Error {
    fn from(value: sdl3::Error) -> Self {
        Self::Sdl3(value)
    }
}

impl From<sdl3::video::WindowBuildError> for Error {
    fn from(value: sdl3::video::WindowBuildError) -> Self {
        Self::WindowBuild(value)
    }
}
