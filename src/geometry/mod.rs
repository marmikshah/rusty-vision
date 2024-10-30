mod point;
mod shape;

use crate::error::Error;
pub use point::Point;
pub use shape::Shape;

/// Compute 1D Index (row-major) when provided with
/// the x, y coordinate, width and channel value.
///
/// # Arguments
///
/// * `x` - The x coordinate (should be between 0 - width)
/// * `y` - The y coordinate (should be between 0 - height)
/// * `width` - The width of the x dimension
/// * `height` - The height of the y dimension
/// * `ndim` - The number of dimensions (ideally same as number of channels)
///
/// # Returns
///
/// * usize containing Index if within bounds,
///     otherwise Error
///
pub fn get_index_from_xywh(
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    ndim: usize,
) -> Result<usize, Error> {
    if x > width || y > height {
        Err(Error::IndexOutOfBounds(
            format!("Invalid coordinates {x:?} x {y:?}").to_string(),
        ))
    } else {
        Ok((y * width + x) * ndim)
    }
}

///
/// Just a wrapper over get_index_from_xywh
///
/// # Arguments
///
/// * `x` - The x coordinate (should be between 0 - width)
/// * `y` - The y coordinate (should be between 0 - height)
/// * `shape` - The shape of the rect this point is currently in
///
/// # Returns
///
/// * usize containing Index if within bounds,
///     otherwise Error
///
pub fn get_index_from_xyshape(x: usize, y: usize, shape: &Shape) -> Result<usize, Error> {
    get_index_from_xywh(x, y, shape.width, shape.height, shape.ndim)
}

///
/// Just a wrapper over get_index_from_xyshape
///
/// # Arguments
///
/// * `point` - The point containing (x, y) coordinate
/// * `shape` - The shape of the rect this point is currently in
///
/// # Returns
///
/// * usize containing Index if within bounds,
///     otherwise Error
///
pub fn get_index_from_point_and_shape(point: Point, shape: &Shape) -> Result<usize, Error> {
    get_index_from_xyshape(point.x, point.y, shape)
}
