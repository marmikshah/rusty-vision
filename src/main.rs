mod codec;
mod core;
mod error;

use codec::{
    decoders::png,
    encoders::{png::PngEncoder, Encoder},
};
use core::image::{ColorFormat, Image};
use std::{
    error::Error,
    fs::File,
    io::{BufWriter, Write},
};

fn main() {
    let width = 10;
    let height = 10;
    let red_pixel = [255, 0, 0];

    let data = red_pixel.repeat((width * height) as usize);
    let encoder = PngEncoder::default();

    let image = &Image::new(width, height, data, ColorFormat::RGB);

    let png_bytes = encoder.encode(image).unwrap();

    let file = File::create("output.png").unwrap();
    let mut writer = BufWriter::new(file);
    writer.write_all(&png_bytes).unwrap();
}
