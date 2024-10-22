pub mod jpeg;
pub mod png;

pub trait Decoder {
    fn decode(&self, data: &[u8]) -> Result<crate::types::Image, crate::error::Error>;
}
