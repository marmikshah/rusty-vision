use std::fs::File;

use crate::error::Error;
use crate::image::Image;

pub fn encode(image: &Image) -> Result<Vec<u8>, Error> {
    todo!()
}

pub fn decode(file: &mut File) -> Result<Image, Error> {
    todo!()
}
