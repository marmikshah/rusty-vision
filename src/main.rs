mod codec;
mod core;
mod error;
mod imgop;
mod io;

use core::image::Image;
use imgop::draw::{Drawing, RectParams};

use error::Error;
use io::writer::Writer;

fn main() -> Result<(), Error> {
    // Create a new blank (mutable) image
    let mut image = Image::new(1920, 1080, core::color::ColorSpace::RGB);

    let params = RectParams::new(100, 100, 200, 200, Some(5));
    image.rect(&params)?;

    image.write("output.png".to_string(), codec::Codex::PNG)?;

    Ok(())
}
