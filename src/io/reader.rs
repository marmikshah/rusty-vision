use std::fs::File;

use crate::{
    codec::{decoders::png::decode, Codex},
    error::Error,
    types::Image,
};

pub trait Reader {
    fn read(path: &str, codex: Codex) -> Result<Image, Error>;
}

impl Reader for Image {
    fn read(path: &str, codex: Codex) -> Result<Image, Error> {
        let mut file = File::open(path)?;

        match codex {
            Codex::PNG => Ok(decode(&mut file)?),
            Codex::JPG => todo!(),
        }
    }
}
