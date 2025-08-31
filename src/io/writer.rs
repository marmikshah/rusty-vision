use std::{
    fs::File,
    io::{BufWriter, Write},
};

use crate::{codec::encoders::Encoder, codec::Codec, error::Error, image::Image};

pub trait Writer {
    fn write(&self, path: String, codec: Codec) -> Result<(), Error>;
}

impl Writer for Image {
    fn write(&self, path: String, codec: Codec) -> Result<(), Error> {
        let data = self
            .encode(codec)
            .map_err(|e| Error::ImageEncodeError(format!("Failed to encode image: {}", e)))?;
        let file = File::create(path).unwrap();
        let mut writer = BufWriter::new(file);
        writer.write_all(&data).map_err(|e| Error::IOError(e))?;

        Ok(())
    }
}
