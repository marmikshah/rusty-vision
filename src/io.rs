use std::fs::File;
use std::io::{BufWriter, Write};

use crate::{
    codec::{formats, Codec, ImageCodec},
    error::Error,
    image::Image,
};

pub struct ImageReader;

pub struct VideoReader;

pub trait Reader<T> {
    fn read(&self, path: &str, codec: Codec) -> Result<T, Error>;
}

pub struct ImageWriter;
pub struct VideoWriter;
pub trait Writer<T> {
    fn write(&self, path: &str, codec: Codec, data: T) -> Result<(), Error>;
}

impl Reader<Image> for ImageReader {
    fn read(&self, path: &str, codec: Codec) -> Result<Image, Error> {
        let mut file = File::open(path).map_err(|e| Error::IOError(e))?;
        let image =
            match codec {
                Codec::Image(ImageCodec::PNG) => formats::png::decode(&mut file)
                    .map_err(|e| Error::ImageDecodeError(e.to_string())),
                Codec::Image(ImageCodec::JPG) => formats::jpeg::decode(&mut file)
                    .map_err(|e| Error::ImageDecodeError(e.to_string())),
                Codec::Video(_) => panic!("Cannot call ImageReader for Video codec"),
            }?;
        Ok(image)
    }
}

impl Writer<Image> for ImageWriter {
    fn write(&self, path: &str, codec: Codec, data: Image) -> Result<(), Error> {
        let bytes = match codec {
            Codec::Image(ImageCodec::PNG) => {
                formats::png::encode(&data).map_err(|e| Error::ImageEncodeError(e.to_string()))
            }
            Codec::Image(ImageCodec::JPG) => {
                formats::jpeg::encode(&data).map_err(|e| Error::ImageEncodeError(e.to_string()))
            }
            Codec::Video(_) => panic!("Cannot call ImageWriter for Video codec"),
        }?;

        let file = File::create(path).map_err(|e| Error::IOError(e))?;
        let mut writer = BufWriter::new(file);
        writer.write_all(&bytes).map_err(|e| Error::IOError(e))?;
        Ok(())
    }
}
