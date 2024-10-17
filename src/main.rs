#![allow(dead_code)]
mod codec;
mod core;
mod error;
mod io;

use core::image::draw::RectParams;
use core::image::traits::*;
use core::image::Image;

use error::Error;
use io::writer::Writer;

fn main() -> Result<(), Error> {
    // Create a new blank (mutable) image
    let mut image = Image::new(1920, 1080, core::color::ColorSpace::RGB);

    let params = RectParams::new(100, 100, 200, 200, Some(5));
    image.draw(&params)?;

    image.write("output.png".to_string(), codec::Codex::PNG)?;

    Ok(())
}
