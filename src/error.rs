#[derive(Debug)]
pub enum Error {
    ImageDecodeError(std::io::Error),
    ImageEncodeError(std::io::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ImageDecodeError(_) => todo!(),
            Error::ImageEncodeError(_) => todo!(),
        }
    }
}
impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}
