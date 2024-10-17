use std::io::{self, ErrorKind};

#[derive(Debug)]
pub enum Error {
    IOError(io::Error),
    ImageDecodeError(io::Error),
    ImageEncodeError(io::Error),
    IndexOutOfBounds(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::IOError(error) => write!(f, "I/O error: {}", error),
            Error::ImageDecodeError(error) => write!(f, "Image decode error: {}", error),
            Error::ImageEncodeError(error) => write!(f, "Image encode error: {}", error),
            Error::IndexOutOfBounds(details) => write!(f, "Index out of bounds: {}", details),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::IOError(error) => Some(error),
            Error::ImageDecodeError(error) => Some(error),
            Error::ImageEncodeError(error) => Some(error),
            Error::IndexOutOfBounds(_) => None,
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::IOError(err)
    }
}

impl From<Error> for io::Error {
    fn from(err: Error) -> Self {
        match err {
            Error::IOError(error) => error,
            Error::ImageDecodeError(error) => error,
            Error::ImageEncodeError(error) => error,
            Error::IndexOutOfBounds(details) => io::Error::new(ErrorKind::Other, details),
        }
    }
}