#![allow(dead_code)]
mod codec;
mod core;
mod error;
mod io;

use core::geometry::point::Point;
use core::geometry::shape::Shape;
use core::image::draw::RectParams;
use core::image::traits::*;
use core::image::Image;

use error::Error;
use io::writer::Writer;

fn main() -> Result<(), Error> {
    // Create a new blank (mutable) image

    let shape = Shape::new(1920, 1080, Some(3));
    let mut image = Image::new(shape, core::color::ColorSpace::RGB);

    let topleft = Point::new(100, 100);
    let rectshape = Shape::new(100, 100, None);
    let params = RectParams::new(topleft, rectshape, Some(5));
    image.draw(&params)?;

    image.write("output.png".to_string(), codec::Codex::PNG)?;

    Ok(())
}
