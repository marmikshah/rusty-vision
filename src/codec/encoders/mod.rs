pub trait Encoder {
    fn encode(&self, image: &crate::core::image::Image) -> Result<Vec<u8>, crate::error::Error>;
}

pub mod jpeg;
pub mod png;
