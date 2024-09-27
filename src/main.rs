mod codec;
mod core;
mod error;

use codec::encoders::{png::PngEncoder, Encoder};
use core::image::Image;
use std::{
    fs::File,
    io::{BufWriter, Write},
};

fn main() {
    let encoder = PngEncoder::default();

    let image = Image::new(1920, 1080, core::color::ColorSpace::RGB);
    let data = vec![0; image.size()];
    let mut image = Image::from_data(
        data,
        image.width(),
        image.height(),
        core::color::ColorSpace::RGB,
    );

    // image[(3, 3, 1)] = 255;
    for i in 0..image.width() {
        if i >= image.height() {
            break;
        }
        image[(i, i, 2)] = 255;
    }

    let png_bytes = encoder.encode(&image).unwrap();

    let file = File::create("output.png").unwrap();
    let mut writer = BufWriter::new(file);
    writer.write_all(&png_bytes).unwrap();
}
