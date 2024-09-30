use std::{
    fs::File,
    io::{BufWriter, Write},
};

use crate::{codec::encoders::Encoder, codec::Codex, core::image::Image, error};

pub trait Writer {
    fn write(&self, path: String, codec: Codex) -> Result<(), error::Error>;
}

impl Writer for Image {
    fn write(&self, path: String, codec: Codex) -> Result<(), error::Error> {
        let data = self.encode(codec)?;
        let file = File::create(path).unwrap();
        let mut writer = BufWriter::new(file);
        writer.write_all(&data)?;

        Ok(())
    }
}
