use super::image::Image;

#[derive(PartialEq, Eq)]
pub enum ColorSpace {
    RGB,
    RGBA,
    BGR,
    BRGA,
}

trait ColorSpaceConversion {
    fn bgr_to_rgb(&self) -> Result<&Image, ()>;
    fn rgb_to_bgr(&self) -> Result<&Image, ()>;
    fn rgba_to_bgra(&self) -> Result<&Image, ()>;
    fn bgra_to_rgba(&self) -> Result<&Image, ()>;
}

impl ColorSpace {
    pub fn channels(&self) -> usize {
        match self {
            ColorSpace::RGB => 3,
            ColorSpace::RGBA => 4,
            ColorSpace::BGR => 3,
            ColorSpace::BRGA => 4,
        }
    }

    pub fn can_convert_to(&self, color_space: &ColorSpace) -> bool {
        return color_space != self;
    }

    pub fn convert_to(&self, image: &Image, color_space: &ColorSpace) -> Result<(), ()> {
        if !self.can_convert_to(color_space) {
            Err(())
        } else {
            Ok(())
        }
    }
}

pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: f32,
}

impl Color {
    pub fn new(red: u8, green: u8, blue: u8, alpha: f32) -> Self {
        Color {
            red,
            green,
            blue,
            alpha,
        }
    }
}
