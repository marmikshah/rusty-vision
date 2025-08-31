#![allow(dead_code)]

use rusty_vision as rv;

use rv::color::{Color, ColorSpace};
use rv::geometry::{Point, Shape};
use rv::image::Image;
use rv::traits::*;

use rv::codec::Codec;
use rv::error::Error;
use rv::io::writer::Writer;

fn main() -> Result<(), Error> {
    let shape = Shape::new(1920, 1080, Some(3));
    let mut image = Image::new(shape, ColorSpace::RGB);

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

    image.write("output.png".to_string(), Codec::PNG)?;

    let mut c180 = image.clone();
    c180.rotate(RotationType::Clockwise180)?;
    c180.write("rotated-c180.png".to_string(), Codec::PNG)?;

    // Rotations alter the same Image structure, so copy if needed
    let mut c90 = image.clone();
    c90.rotate(RotationType::Clockwise90)?;
    c90.write("rotated-c90.png".to_string(), Codec::PNG)?;

    let mut c270 = image.clone();
    c270.rotate(RotationType::Clockwise270)?;
    c270.write("rotated-c270.png".to_string(), Codec::PNG)?;

    let crop = image.crop(topleft, rectshape);
    crop.write("cropped.png".to_string(), Codec::PNG)?;
    Ok(())
}
