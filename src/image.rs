/**
 * A single page Implementation of all the core
 * functions for an Image type.
 * The Image structure is purely CPU based and
 * has size limitations.
 *  ------------------------------------------------------------
 * Images are stored as a row major vector.
 * Formula: (y * self.width + x) * number-of-channels
 *
 * For example:
 *     A 3 x 3 Image is stored as a 1D array of length 27
 *     Which looks something like: (Displaying a 2D Structure for visualisation)
 *     [
 *         R00, G00, B00,         R01, G01, B01,          R02, G02, B02
 *         R10, G10, B10          R11, G11, B11,          R12, G12, B12,
 *         R20, G20, B20,         R21, G21, B21,          R22, G22, B22,
 *     ]
 * ---------------------------------------------------------------
 *
 * To get 1D index of a (x, y) pixel point,
 *   = (y * image-width + x) * number-of-channels
 *   where,
 *      image-width: The width of Image in pixels
 *      number-of-channels: The total number of color channels.
 *              (For example, 3 for an RGB image and 4 for RGBA)
 */
use std::ops::{Index, IndexMut};

use log::debug;

use crate::color::{Color, ColorSpace};
use crate::error::Error;
use crate::geometry::{Point, Shape};
use crate::traits::*;
use crate::types::*;

pub struct Image {
    shape: Shape,
    data: Vec<u8>,
    colorspace: ColorSpace,
}

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

/* ------------------ DRAWING TRAIT ------------------ */
impl Drawable<RectParams> for Image {
    ///
    /// Draw a rectangle on the Image
    ///
    fn draw(&mut self, params: &RectParams) -> Result<(), Error> {
        let border_width = params.border_width.unwrap_or_default();

        match params.fill_color {
            Some(color) => {
                let top_left = params.topleft;
                let bottom_right = params.topleft + params.shape;

                for i in top_left.x..bottom_right.x + 1 {
                    for j in top_left.y..bottom_right.y + 1 {
                        let point = Point::new(i, j);
                        self.set_pixel(&point, &color)?;
                    }
                }
            }
            None => {
                dbg!("Fill not enabled");
            }
        };

        for i in 0..params.shape.width + border_width {
            let range = match params.border_width {
                Some(value) => (-((value / 2) as i32), ((value / 2) + 1) as i32),
                None => (0, 1),
            };

            dbg!(range);
            for k in range.0..range.1 {
                let x_top_left = params.topleft.x;
                let y_top_left = params.topleft.y;

                // Top Edge
                let top = Point::new(
                    x_top_left + i - (border_width / 2),
                    (y_top_left as i32 + k) as usize,
                );
                dbg!(top);
                self.set_pixel(&top, &params.color)?;

                // Left Side
                let left = Point::new(
                    (x_top_left as i32 + k) as usize,
                    y_top_left + i - (border_width / 2),
                );
                dbg!(left);
                self.set_pixel(&left, &params.color)?;

                // Right Edge
                let right = Point::new(
                    ((x_top_left + params.shape.width) as i32 - k) as usize,
                    y_top_left + i - (border_width / 2),
                );
                dbg!(right);
                self.set_pixel(&right, &params.color)?;

                // Bottom Edge
                let bottom = Point::new(
                    x_top_left + i - (border_width / 2),
                    ((y_top_left + params.shape.height) as i32 - k) as usize,
                );
                dbg!(bottom);
                self.set_pixel(&bottom, &params.color)?;
            }
        }

        Ok(())
    }
}

impl Drawable<CircleParams> for Image {
    ///
    /// Draw a circle on the Image using the Midpoint Circle Algorithm.
    ///
    fn draw(&mut self, params: &CircleParams) -> Result<(), Error> {
        println!("Drawing circle {params:?}");

        let mut x = params.radius;
        let mut y = 0;

        let mut p: f32 = 1.0 - params.radius as f32;

        while x >= y {
            dbg!(x, y, p);

            let symmetric = [
                [
                    Point::new(params.center.x - x, params.center.y + y),
                    Point::new(params.center.x + x, params.center.y + y),
                ],
                [
                    Point::new(params.center.x - x, params.center.y - y),
                    Point::new(params.center.x + x, params.center.y - y),
                ],
                [
                    Point::new(params.center.x - y, params.center.y + x),
                    Point::new(params.center.x + y, params.center.y + x),
                ],
                [
                    Point::new(params.center.x - y, params.center.y - x),
                    Point::new(params.center.x + y, params.center.y - x),
                ],
            ];

            for pair in symmetric {
                if let Some(color) = params.fill_color {
                    let mut start = pair[0].clone();
                    let end = pair[1].clone();
                    while start.x < end.x - 1 {
                        start.x += 1;
                        dbg!(start);
                        self.set_pixel(&start, &color)?;
                    }
                }

                self.set_pixel(&pair[0], &params.color)?;
                self.set_pixel(&pair[1], &params.color)?;
            }

            y += 1;

            if p <= 0.0 {
                p = p + 2.0 * y as f32 + 1.0;
            } else {
                x -= 1;
                p = p + 2.0 * y as f32 - 2.0 * x as f32 + 1.0;
            }
        }

        Ok(())
    }
}
/* ------------------ ------- ----- ------------------ */

/* ------------------ RESIZE TRAIT ------------------ */
impl Resizable<NearestNeighborParams> for Image {
    fn resize(&mut self, shape: Shape) -> Result<(), Error> {
        todo!()
    }
}

impl Resizable<BiCubicParams> for Image {
    fn resize(&mut self, shape: Shape) -> Result<(), Error> {
        todo!()
    }
}

impl Resizable<BiLinearParams> for Image {
    fn resize(&mut self, shape: Shape) -> Result<(), Error> {
        todo!()
    }
}
/* ------------------ ------ ----- ------------------ */

/* ------------------ ROTATION TRAIT ------------------ */

impl Rotatable<i32> for Image {
    fn rotate(&mut self, value: i32) -> Result<(), Error> {
        // Custom rotation by degrees.
        todo!()
    }
}

impl Rotatable<RotationType> for Image {
    fn rotate(&mut self, value: RotationType) -> Result<(), Error> {
        match value {
            RotationType::CLOCKWISE90 => todo!(),
            RotationType::CLOCKWISE180 | RotationType::ANTICLICKWISE180 => {
                for y in 0..self.height() {
                    for x in 0..self.width() / 2 {
                        for c in 0..self.colorspace.channels() {
                            let idx_a = self.get_index_from_xy(x, y).unwrap() + c;
                            let idx_b =
                                self.get_index_from_xy(self.width() - x - 1, y).unwrap() + c;

                            self.swap(idx_a, idx_b);
                        }
                    }
                }
                Ok(())
            }
            RotationType::CLOCKWISE270 => todo!(),
            RotationType::ANTICLOCKWISE90 => todo!(),
            RotationType::ANTICLICKWISE270 => todo!(),
        }
    }
}
/* ------------------ -------- ----- ------------------ */
