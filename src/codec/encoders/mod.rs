mod jpeg;
mod png;

use super::Codec;
use crate::image::Image;

pub trait Encoder {
    fn encode(&self, codec: Codec) -> Result<Vec<u8>, crate::error::Error>;
}

impl Encoder for Image {
    fn encode(&self, codec: Codec) -> Result<Vec<u8>, crate::error::Error> {
        match codec {
            Codec::PNG => png::encode(self),
            Codec::JPG => todo!(),
        }
    }
}
