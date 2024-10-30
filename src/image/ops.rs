use super::Image;
use crate::geometry::Point;
use crate::types::*;
use std::ops::{Add, BitAnd, BitOr, BitXor, Index, IndexMut, Sub};

///
/// 1-D Indexing.
/// Use this only if you know what you're doing.
///
/// # Returns
///
/// * A reference to the value at requested index
///
impl Index<usize> for Image {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

///
/// 1-D Indexing.
/// Same as IndexMut, but mutable.
///
impl IndexMut<usize> for Image {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

///
/// 2-D Indexing.
/// Get all channel values for a requested pixel.
/// When asked for pixel (x, y) the output will be
/// a slice of [idx .. idx + ndim] where idx, is
/// the computed row major index from the (x, y).
/// See 'geometry::get_index_from_xywh'
///
/// # Returns
///
/// * [u8]
///
impl Index<Index2D> for Image {
    type Output = [u8];

    fn index(&self, (x, y): Index2D) -> &Self::Output {
        let point = Point::new(x, y);
        self.get_pixel(&point)
    }
}

///
/// 2-D Indexing.
/// Same as `IndexMut<Index2D>`, but mutable.
///
/// # Returns
/// * [u8]
///
impl IndexMut<Index2D> for Image {
    fn index_mut(&mut self, (x, y): Index2D) -> &mut Self::Output {
        let point = Point::new(x, y);
        self.get_mut_pixel(&point)
    }
}

///
/// 3-D Indexing
/// This is useful when you want to modify a specific channel
/// For example, to modify green channel in an RGB or RGBA image,
/// image[(2, 2, 1)] = 255
/// Where `1` = the channel number.
///
/// See ColorSpace for more information on Channel Numbers
///
/// # Returns
/// * u8
///
impl Index<Index3D> for Image {
    type Output = u8;

    fn index(&self, (x, y, c): Index3D) -> &Self::Output {
        let point = Point::new(x, y);
        let pixel = self.get_pixel(&point);
        &pixel[c]
    }
}

///
/// 3-D Indexing
/// Same as `IndexMut<Index3D>` but mutable.
///
/// # Returns
/// * u8
///
impl IndexMut<Index3D> for Image {
    fn index_mut(&mut self, (x, y, c): Index3D) -> &mut Self::Output {
        let point = Point::new(x, y);
        let pixel = self.get_mut_pixel(&point);
        &mut pixel[c]
    }
}

// --------------------- Operator Implementations -------------------- \\
// ------- All the operations here are performed in-place. --------- \\

impl Add<Image> for Image {
    type Output = Image;

    fn add(mut self, rhs: Image) -> Self::Output {
        self.combine(&rhs, |a, b| a + b);
        self
    }
}

impl Sub<Image> for Image {
    type Output = Image;

    fn sub(mut self, rhs: Image) -> Self::Output {
        self.combine(&rhs, |a, b| if a <= b { 0 } else { a - b });
        self
    }
}

impl BitAnd<Image> for Image {
    type Output = Image;

    fn bitand(mut self, rhs: Image) -> Self::Output {
        self.combine(&rhs, |a, b| a & b);
        self
    }
}

impl BitOr<Image> for Image {
    type Output = Image;

    fn bitor(mut self, rhs: Image) -> Self::Output {
        self.combine(&rhs, |a, b| a | b);
        self
    }
}

impl BitXor<Image> for Image {
    type Output = Image;

    fn bitxor(mut self, rhs: Image) -> Self::Output {
        self.combine(&rhs, |a, b| a ^ b);
        self
    }
}
