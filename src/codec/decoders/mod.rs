pub mod jpeg;
pub mod png;

pub trait Decoder {
    fn decode(&self, data: &[u8]) -> Result<crate::core::image::Image, crate::error::Error>;
}
