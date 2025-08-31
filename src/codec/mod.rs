pub mod jpeg;
pub mod png;

pub mod formats {
    // Re-export for convenience
    pub use super::jpeg;
    pub use super::png;
}

pub enum Codec {
    Image(ImageCodec),
    Video(VideoCodec),
}

pub enum ImageCodec {
    PNG,
    JPG,
}

pub enum VideoCodec {
    H264,
    H265,
}
