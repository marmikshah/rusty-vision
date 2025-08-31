use rusty_vision as rv;

use rv::prelude::*;

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

    image.write("output.png".to_string(), Codec::PNG).unwrap();
}
