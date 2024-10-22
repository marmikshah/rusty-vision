use rusty_vision as rv;

use rv::core::traits::*;
use rv::types::Image;

// Useful structures for geometric operations
use rv::core::geometry::{Point, Shape};

// Structures and Implenetations for Colors and Channels.
use rv::core::color::{Color, ColorSpace};

// Image Encoding/Decoding
use rv::codec::Codex;
use rv::io::writer::Writer;

fn main() {
    let mut image = Image::new(
        Shape {
            width: 1920,
            height: 1080,
            ndim: 3,
        },
        ColorSpace::RGB,
    );

    let config = RectParams::new(
        Point { x: 10, y: 10 },
        Shape {
            width: 100,
            height: 100,
            ndim: 1,
        },
        Color::new(20, 150, 20, 1.0),
        Some(10),
        Some(0.0),
        None,
    );
    image.draw(&config).unwrap();

    image.write("output.png".to_string(), Codex::PNG).unwrap();
}
