pub mod traits;
pub mod draw;

use crate::core::color::{Color, ColorSpace};
use std::ops::{Index, IndexMut};

use crate::error::Error;
use log::debug;

use super::types::*;

pub struct Image {
    width: usize,
    height: usize,
    data: Vec<u8>,
    colorspace: ColorSpace,
}

/// ------------------------------------------------------------------------------
///  Images are stored as a row major vector.
///  Formula: (y * self.width + x) * number-of-channels

///  For example:
///      A 3 x 3 Image is stored as a 1D array of length 27
///      Which looks something like:
///      [
///          R00, G00, B00,
///          R01, G01, B01,
///          R02, G02, B02
///          R10, G10, B10
///          R11, G11, B11,
///          R12, G12, B12,
///          R20, G20, B20,
///          R21, G21, B21,
///          R22, G22, B22
///      ]

///  Now if we want to get all 3 channels at index (x, y)
///  where
///      x = 1,
///      y = 1,
///      image width = 3
///      number-of-channels = 3 (For an RGB)

///  index = (1 * 3 + 1) * 3
///        = (12)
///  So 12 .. (12 + 3) => (R11, G11, B11)
/// ------------------------------------------------------------------------------
impl Image {
    pub fn new(width: usize, height: usize, colorspace: ColorSpace) -> Self {
        let size = width * height * colorspace.channels();
        let data = vec![0; size];
        debug!("Creating Image of size {}", data.len());
        dbg!(data.len());
        Image {
            width,
            height,
            data,
            colorspace,
        }
    }

    pub fn from_data(data: Vec<u8>, width: usize, height: usize, colorspace: ColorSpace) -> Self {
        let size = width as usize * height as usize * colorspace.channels();
        dbg!(width as usize * height as usize * colorspace.channels());
        assert_eq!(data.len(), size);

        Image {
            width,
            height,
            data,
            colorspace,
        }
    }

    pub fn slice(&self, start: usize, end: usize) -> &[u8] {
        &self.data[start..end]
    }

    pub fn mut_slice(&mut self, start: usize, end: usize) -> &mut [u8] {
        &mut self.data[start..end]
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn size(&self) -> usize {
        self.height as usize * self.width as usize * self.colorspace.channels() as usize
    }

    ///
    /// Calculates the 1D Index based on the provided (x, y)
    /// # Arguments
    ///
    /// * `x` - The x coordinate
    /// * `y` - The y coordinate
    ///
    /// # Returns
    ///
    /// * Option<usize> containing Index if within bounds,
    ///     otherwise None
    ///
    pub fn get_index(&self, x: usize, y: usize) -> usize {
        let index = (y * self.width + x) as usize * self.colorspace.channels();
        if index >= self.size() {
            panic!("{x:?}x{y:?} out of bounds!")
        }
        index
    }

    ///
    /// Return a slize of 1 pixel (including all channels)
    ///
    /// # Arguments
    ///
    /// * `x` - The x coordinate
    /// * `y` - The y coordinate
    ///
    /// # Returns
    ///
    /// * An immutable &[u8]
    ///
    pub fn get_pixel(&self, x: usize, y: usize) -> &[u8] {
        let channels = self.colorspace.channels();
        let index = self.get_index(x, y);
        &self.data[index..index + channels]
    }

    /// Same as `get_pixel` but just a mutable reference
    pub fn get_mut_pixel(&mut self, x: usize, y: usize) -> &mut [u8] {
        let channels = self.colorspace.channels();
        let index = self.get_index(x, y);
        &mut self.data[index..index + channels]
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: &Color) -> Result<(), Error> {
        let channels = self.colorspace.channels();
        let pixel = self.get_mut_pixel(x, y);
        for i in 0..channels {
            pixel[i] = color[i];
        }
        Ok(())
    }
}

///
/// Overriding [] for 1D indexing.
///
/// # Returns an immutable reference to the requested Index
///     value in `data` vector
///
impl Index<usize> for Image {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

///
/// Overriding [] for 1D indexing.
///
/// # Returns a mutable reference to the requested Index
///     value in `data` vector
///
impl IndexMut<usize> for Image {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index as usize]
    }
}

///
/// Overriding [] for 2D indexing.
///
/// # Returns an immutable reference to a given pixel
///     The pixel
///
impl Index<Index2D> for Image {
    type Output = [u8];

    fn index(&self, (x, y): Index2D) -> &Self::Output {
        self.get_pixel(x, y)
    }
}

impl IndexMut<Index2D> for Image {
    fn index_mut(&mut self, (x, y): Index2D) -> &mut Self::Output {
        self.get_mut_pixel(x, y)
    }
}

///
/// Overriding [] for 3D indexing.
/// This is useful when you want to modify a specific channel
/// For example, to modify green channel,
/// image[(2, 2, 1)] = 255
/// Where `1` = the channel number.
///
/// See ColorSpace for more information on Channel Numbers
///
impl Index<Index3D> for Image {
    type Output = u8;

    fn index(&self, (x, y, c): Index3D) -> &Self::Output {
        let pixel = self.get_pixel(x, y);
        &pixel[c]
    }
}

impl IndexMut<Index3D> for Image {
    fn index_mut(&mut self, (x, y, c): Index3D) -> &mut Self::Output {
        let pixel = self.get_mut_pixel(x, y);
        &mut pixel[c]
    }
}
