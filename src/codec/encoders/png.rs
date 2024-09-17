use super::Encoder;

struct PngEncoder;

impl Encoder for PngEncoder {
    fn encode(&self, image: &crate::core::image::Image) -> Result<Vec<u8>, crate::error::Error> {
        todo!()
    }
}

