use std::io;

#[derive(Debug)]
#[allow(clippy::enum_variant_names)]
pub enum Error {
    IOError(io::Error),
    ImageDecodeError(String),
    ImageEncodeError(String),
    IndexOutOfBounds(String),
    InvalidChannel(String),
    NotImplemented(String),
    UnsupportedOperation(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::IOError(error) => write!(f, "I/O error: {}", error),
            Error::ImageDecodeError(error) => write!(f, "Image decode error: {}", error),
            Error::ImageEncodeError(error) => write!(f, "Image encode error: {}", error),
            Error::IndexOutOfBounds(details) => write!(f, "Index out of bounds: {}", details),
            Error::InvalidChannel(details) => write!(f, "Invalid channel: {}", details),
            Error::NotImplemented(details) => write!(f, "Not implemented: {}", details),
            Error::UnsupportedOperation(details) => write!(f, "Color space error: {}", details),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::IOError(error) => Some(error),
            Error::ImageDecodeError(_) => None,
            Error::ImageEncodeError(_) => None,
            Error::IndexOutOfBounds(_) => None,
            Error::InvalidChannel(_) => None,
            Error::NotImplemented(_) => None,
            Error::UnsupportedOperation(_) => None,
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::IOError(err)
    }
}
