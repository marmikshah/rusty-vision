#![allow(dead_code)]
mod codec;
mod core;
mod error;
mod io;
mod types;

use core::color::Color;
use core::geometry::{Point, Shape};
use core::traits::*;
use types::Image;

use error::Error;
use io::writer::Writer;

fn main() -> Result<(), Error> {
    let shape = Shape::new(1920, 1080, Some(3));
    let mut image = Image::new(shape, core::color::ColorSpace::RGB);

    let topleft = Point::new(100, 100);
    let rectshape = Shape::new(100, 100, None);
    let params = RectParams::new(
        topleft,
        rectshape,
        Color::new(20, 150, 20, 1.0),
        Some(10),
        Some(0.0),
        None,
    );
    image.draw(&params)?;

    let center = Point::new(500, 500);
    let color = Color::new(10, 250, 20, 1.0);
    let fill = Color::new(132, 18, 28, 1.0);
    let circle = CircleParams::new(center, 20, color, Some(fill));
    // let circle = CircleParams::new(center, 20, color, None);
    image.draw(&circle)?;

    image.write("output.png".to_string(), codec::Codex::PNG)?;

    image.rotate(RotationType::CLOCKWISE180)?;
    image.write("rotated.png".to_string(), codec::Codex::PNG)?;

    let crop = image.crop(topleft, rectshape);
    crop.write("cropped.png".to_string(), codec::Codex::PNG)?;
    Ok(())
}
