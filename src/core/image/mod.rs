mod image_impl;
use crate::core::color::{Color, ColorSpace};
use std::ops::{Index, IndexMut};

use crate::error::Error;
use log::debug;

use super::geometry::{Point, Shape};
use super::types::*;

pub struct Image {
    shape: Shape,
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
    pub fn new(shape: Shape, colorspace: ColorSpace) -> Self {
        let data = vec![0; shape.size()];
        assert_eq!(data.len(), shape.size());
        debug!("Creating Image of size {}", data.len());
        dbg!(data.len());
        Image {
            shape,
            data,
            colorspace,
        }
    }

    pub fn from_data(data: Vec<u8>, shape: Shape, colorspace: ColorSpace) -> Self {
        assert_eq!(data.len(), shape.size());
        Image {
            shape,
            data,
            colorspace,
        }
    }

    pub fn swap(&mut self, idx_a: usize, idx_b: usize) {
        self.data.swap(idx_a, idx_b);
    }

    pub fn slice(&self, start: usize, end: usize) -> &[u8] {
        &self.data[start..end]
    }

    pub fn mut_slice(&mut self, start: usize, end: usize) -> &mut [u8] {
        &mut self.data[start..end]
    }

    pub fn width(&self) -> usize {
        self.shape.width
    }

    pub fn height(&self) -> usize {
        self.shape.height
    }

    pub fn size(&self) -> usize {
        self.shape.size()
    }

    pub fn crop(&self, topleft: Point, shape: Shape) -> Self {
        let bottomright = topleft + shape;

        let mut data = Vec::new();
        for hindex in topleft.x..bottomright.x {
            for vindex in topleft.y..bottomright.y {
                for cindex in 0..self.shape.ndim {
                    data.push(self[(hindex, vindex, cindex)]);
                }
            }
        }

        dbg!(data.len());

        let image_shape = Shape::new(shape.width, shape.height, Some(self.colorspace.channels()));
        Self::from_data(data, image_shape, self.colorspace)
    }

    ///
    /// Calculates the 1D Index based on the provided (x, y)
    /// Just wraps `get_index_from_xy` for sake of convenience
    /// # Arguments
    ///
    /// * `Point` - Point containing desired x and y
    ///
    /// # Returns
    ///
    /// * Result<usize> containing Index if within bounds,
    ///     otherwise Error
    ///
    pub fn get_index(&self, point: &Point) -> Result<usize, Error> {
        self.get_index_from_xy(point.x, point.y)
    }

    /// Compute 1D Index (row-major) when provided with
    /// the x, y coordinate, width and channel value.
    ///
    /// # Arguments
    ///
    /// * `x` - The x coordinate (should be between 0 - width of Image)
    /// * `y` - The y coordinate (should be between 0 - height of Image)
    ///
    /// # Returns
    ///
    /// * Result<usize> containing Index if within bounds,
    ///     otherwise Error
    ///
    pub fn get_index_from_xy(&self, x: usize, y: usize) -> Result<usize, Error> {
        if x >= self.width() || y >= self.height() {
            Err(Error::IndexOutOfBounds("Invalid coordinates".to_string()))
        } else {
            let value = (y * self.width() + x) * self.shape.ndim;
            Ok(value)
        }
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
    pub fn get_pixel(&self, point: &Point) -> &[u8] {
        let channels = self.colorspace.channels();
        let index = self.get_index(point).unwrap();
        &self.data[index..index + channels]
    }

    /// Same as `get_pixel` but just a mutable reference
    pub fn get_mut_pixel(&mut self, point: &Point) -> &mut [u8] {
        let channels = self.colorspace.channels();
        let index = self.get_index(point).unwrap();
        &mut self.data[index..index + channels]
    }

    pub fn set_pixel(&mut self, point: &Point, color: &Color) -> Result<(), Error> {
        let channels = self.colorspace.channels();
        let pixel = self.get_mut_pixel(point);
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
        &mut self.data[index]
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
        let point = Point::new(x, y);
        self.get_pixel(&point)
    }
}

impl IndexMut<Index2D> for Image {
    fn index_mut(&mut self, (x, y): Index2D) -> &mut Self::Output {
        let point = Point::new(x, y);
        self.get_mut_pixel(&point)
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
        let point = Point::new(x, y);
        let pixel = self.get_pixel(&point);
        &pixel[c]
    }
}

impl IndexMut<Index3D> for Image {
    fn index_mut(&mut self, (x, y, c): Index3D) -> &mut Self::Output {
        let point = Point::new(x, y);
        let pixel = self.get_mut_pixel(&point);
        &mut pixel[c]
    }
}
