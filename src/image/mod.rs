mod draw;
mod ops;
mod resize;
mod rotate;
use log::debug;

use crate::color::{Color, ColorSpace};
use crate::error::Error;
use crate::geometry::{self, Point, Shape};

#[derive(Debug, Clone)]
pub struct Image {
    /// Core Image struct.
    ///
    /// This Image structure is purely CPU based and
    /// has size limitations.
    ///  ------------------------------------------------------------
    /// Images are stored as a row major vector.
    /// Formula: (y * self.width + x) * number-of-channels
    ///
    /// For example:
    ///     A 3 x 3 RGB Image is stored as a 1D array of length 27 (3 width x 3 height x 3 channels)
    ///     Which looks something like: (Displaying a 2D Structure for visualization)
    ///     [
    ///         R00, G00, B00,         R01, G01, B01,          R02, G02, B02
    ///         R10, G10, B10          R11, G11, B11,          R12, G12, B12,
    ///         R20, G20, B20,         R21, G21, B21,          R22, G22, B22,
    ///     ]
    /// ---------------------------------------------------------------
    ///
    /// To get 1D index of a (x, y) pixel point,
    ///   = (y * image-width + x) * number-of-channels
    ///   where,
    ///      image-width: The width of Image in pixels
    ///      number-of-channels: The total number of color channels.
    ///              (For example, 3 for an RGB image and 4 for RGBA)
    ///
    shape: Shape,
    data: Vec<u8>,
    colorspace: ColorSpace,
}

impl Image {
    pub fn new(shape: Shape, colorspace: ColorSpace) -> Self {
        let data = vec![0; shape.size()];
        assert_eq!(data.len(), shape.size());
        debug!("Creating Image of size {}", data.len());
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

        let image_shape = Shape::new(shape.width, shape.height, Some(self.colorspace.channels()));
        Self::from_data(data, image_shape, self.colorspace)
    }

    pub fn combine<F>(&mut self, rhs: &Image, op: F)
    where
        F: Fn(u32, u32) -> u32,
    {
        assert_eq!(self.size(), rhs.size());
        // TODO: Change clam to support max of the types later
        self.data
            .iter_mut()
            .zip(rhs.data.iter())
            .for_each(|(a, &b)| {
                *a = op(*a as u32, b as u32).clamp(0, 255) as u8;
            });
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
    /// * usize containing Index if within bounds,
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
    /// * usize containing Index if within bounds,
    ///     otherwise Error
    ///
    pub fn get_index_from_xy(&self, x: usize, y: usize) -> Result<usize, Error> {
        geometry::get_index_from_xywh(
            x,
            y,
            self.width(),
            self.height(),
            self.colorspace.channels(),
        )
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
