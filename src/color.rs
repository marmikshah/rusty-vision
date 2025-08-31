use std::ops::Index;

use crate::{error::Error, image::Image};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[allow(clippy::upper_case_acronyms)]
pub enum ColorSpace {
    RGB,
    RGBA,
    BGR,
    BGRA,
    YUV420P,
    Gray,
}

impl std::fmt::Display for ColorSpace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
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
            ColorSpace::BGRA => 4,
            ColorSpace::YUV420P => 3, // Special case, 3 Planes Y, U, V
            ColorSpace::Gray => 1,
        }
    }

    pub fn can_convert_to(&self, color_space: &ColorSpace) -> bool {
        color_space != self
    }

    // TODO
    #[allow(unused_variables)]
    pub fn convert_to(&self, image: &Image, color_space: &ColorSpace) -> Result<(), Error> {
        if !self.can_convert_to(color_space) {
            Err(Error::UnsupportedOperation(format!(
                "Cannot convert from {} to {}",
                self, color_space
            )))
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

    pub fn as_rgb_slice(&self) -> [u8; 3] {
        [self.red, self.green, self.blue]
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
