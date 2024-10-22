use rusty_vision as rv;

use rv::core::image::Image;
use rv::core::traits::*;

// Useful structures for geometric operations
use rv::core::geometry::{Point, Shape};

// Structures and Implenetations for Colors and Channels.
use rv::core::color::{Color, ColorSpace};

// Image Encoding/Decoding
use rv::codec::Codex;
use rv::io::writer::Writer;

fn main() {
    let shape = Shape::new(1920, 1080, Some(3));
    let mut image = Image::new(shape, ColorSpace::RGB);

    let rect = Shape::new(100, 100, None);
    let topleft = Point::new(10, 10);

    let config = RectParams::new(
        topleft,
        rect,
        Color::new(20, 150, 20, 1.0),
        Some(10),
        Some(0.0),
        None,
    );
    image.draw(&config).unwrap();

    image.write("output.png".to_string(), Codex::PNG).unwrap();
}
