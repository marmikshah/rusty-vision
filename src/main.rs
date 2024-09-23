mod codec;
mod core;
mod error;

use codec::encoders::{png::PngEncoder, Encoder};
use core::image::{ColorFormat, Image};
use std::{
    fs::File,
    io::{BufWriter, Write},
};

fn main() {
    let encoder = PngEncoder::default();

    let image = Image::new(512, 512, ColorFormat::RGB);
    let data = vec![0; image.size()];
    let mut image = Image::from_data(data, image.width(), image.height(), ColorFormat::RGB);

    image[(3, 3)] = [255, 255, 255];

    let png_bytes = encoder.encode(&image).unwrap();

    let file = File::create("output.png").unwrap();
    let mut writer = BufWriter::new(file);
    writer.write_all(&png_bytes).unwrap();
}
