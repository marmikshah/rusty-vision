use std::fs::File;

use crate::{
    codec::{decoders::png::decode, Codex},
    core::image::Image,
    error::Error,
};

pub trait Reader {
    fn read(path: &String, codex: Codex) -> Result<Image, Error>;
}

impl Reader for Image {
    fn read(path: &String, codex: Codex) -> Result<Image, Error> {
        let mut file = File::open(path)?;

        match codex {
            Codex::PNG => {
                return Ok(decode(&mut file)?);
            }
            Codex::JPG => todo!(),
        }
    }
}
