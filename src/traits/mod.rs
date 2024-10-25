/**
 * A collection of all the common traits that
 * can be implemented for Image Manipulation.
 *
 * These are intended to be kept as traits to
 * isolate implementation of different type of
 * images.
 * For example, these traits will have different
 * implementations for an Image on CUDA or some
 * other formats.
 *
 */
mod draw;
mod resize;
mod rotate;

// TODO: Find a better way to import.
pub use draw::*;
pub use resize::*;
pub use rotate::*;
