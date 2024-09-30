# Rusty Vision

An image processing library.


## Create an image

```rust
use core::image::Image;
use codec::Codex;
use core::color::ColorSpace;
use crate::error::Error;

fn main() -> Result<(), Error> {
    let mut image = Image::new(1920, 1080, ColorSpace::RGB);
    image.write("output.png".to_string(), Codex::PNG);
}

```