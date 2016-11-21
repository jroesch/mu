use std::io;

#[derive(Debug)]
pub enum Error {
    TypeMismatch,
    MissingKey(String),
    Io(io::Error)
}

impl From<io::Error> for Error {
    fn from(x: io::Error) -> Error {
        Error::Io(x)
    }
}
