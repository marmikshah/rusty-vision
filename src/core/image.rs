pub enum ColorFormat {
    RGB,
    RGBA,
    BGR,
    BGRA,
}
pub struct Image {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
    pub color_format: ColorFormat,
}

impl Image {
    pub fn new(width: u32, height: u32, data: Vec<u8>, color_format: ColorFormat) -> Self {
        assert_eq!(data.len(), (width * height * 3) as usize);
        Image {
            width,
            height,
            data,
            color_format,
        }
    }
}
