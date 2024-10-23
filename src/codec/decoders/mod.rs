pub mod jpeg;
pub mod png;

use crate::image::Image;

pub trait Decoder {
    fn decode(&self, data: &[u8]) -> Result<Image, crate::error::Error>;
}
