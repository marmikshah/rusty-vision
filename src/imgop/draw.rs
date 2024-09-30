use crate::core::image::Image;

pub fn rect(image: &Image, x: u32, y: u32, width: u32, height: u32) -> Result<(), ()> {

    Ok(())
}

pub fn square(image: &Image, x: u32, y: u32, width: u32, height: u32) -> Result<(), ()> {
    rect(image, x, y, width, height)
}

pub fn circle(image: &Image, center: u32, radius: u32) -> Result<(), ()> {
    Ok(())
}
