mod jpeg;
mod png;

use super::Codex;
use crate::core::image::Image;

pub trait Encoder {
    fn encode(&self, codec: Codex) -> Result<Vec<u8>, crate::error::Error>;
}

impl Encoder for Image {
    fn encode(&self, codec: Codex) -> Result<Vec<u8>, crate::error::Error> {
        match codec {
            Codex::PNG => png::encode(self),
            Codex::JPG => todo!(),
        }
    }
}
