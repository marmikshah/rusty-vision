use std::ops::Index;

use super::image::Image;

#[derive(Debug, PartialEq, Eq)]
pub enum ColorSpace {
    RGB,
    RGBA,
    BGR,
    BRGA,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

trait Convert {
    fn convert(to_colorspace: ColorSpace) -> Self;
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

impl Color {
    pub fn new(red: u8, green: u8, blue: u8, alpha: f32) -> Self {
        let alpha = (alpha * 255.0) as u8;
        Color {
            red,
            green,
            blue,
            alpha,
        }
    }

    pub fn as_vec(&self) -> Vec<u8> {
        vec![self.red, self.green, self.blue, self.alpha]
    }
}

impl Index<usize> for Color {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.red,
            1 => &self.green,
            2 => &self.blue,
            _ => &self.red, // todo fix this
        }
    }
}
