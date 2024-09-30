mod codec;
mod core;
mod error;
mod io;

use core::image::Image;

use codec::Codex;
use error::Error;
use io::{reader::Reader, writer::Writer};

fn main() -> Result<(), Error> {
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

    image.write("output.png".to_string(), codec::Codex::PNG)?;

    let path = String::from("output.png");
    let mut image2 = Image::read(&path, Codex::PNG)?;

    assert_eq!(image2.width(), 1920);
    assert_eq!(image2.height(), 1080);

    for i in 0..image.height() {
        image2[(i + 20, i, 1)] = 255;
    }

    let output = String::from("out.png");
    image2.write(output, Codex::PNG)?;

    Ok(())
}
