use std::fs::File;

use crate::{
    codec::{decoders::png::decode, Codec},
    error::Error,
    image::Image,
};

pub trait Reader {
    fn read(path: &str, codec: Codec) -> Result<Image, Error>;
}

impl Reader for Image {
    fn read(path: &str, codec: Codec) -> Result<Image, Error> {
        let mut file = File::open(path)?;

        match codec {
            Codec::PNG => Ok(decode(&mut file)?),
            Codec::JPG => todo!(),
        }
    }
}
