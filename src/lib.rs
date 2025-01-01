#![allow(dead_code)]
pub mod codec;
pub mod color;
pub mod error;
pub mod geometry;
pub mod image;
pub mod io;
pub mod traits;
pub mod types;

// Re-export for convenience
pub mod prelude {
    pub use crate::codec::*;
    pub use crate::color::*;
    pub use crate::geometry::*;
    pub use crate::image::*;
    pub use crate::io::*;
    pub use crate::traits::*;
}
