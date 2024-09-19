pub enum ColorFormat {
    RGB,
    RGBA,
}

impl ColorFormat {
    pub fn get_index(&self, width: u32, height: u32, x: u32, y: u32) -> usize {
        match self {
            ColorFormat::RGB => return (y * width + x) as usize,
            ColorFormat::RGBA => return ((y * width + 3) * 4) as usize,
        };
    }
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

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn channels(&self) -> usize {
        match self.color_format {
            ColorFormat::RGB => 3,
            ColorFormat::RGBA => 4,
        }
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, color: &[u8]) -> Result<(), ()> {
        let index = self.color_format.get_index(self.width, self.height, x, y);

        for (i, &value) in color.iter().enumerate() {
            self.data[index + i] = value;
        }

        Ok(())
    }
    pub fn get_pixel(&self, x: u32, y: u32) -> &[u8] {
        let index = self.color_format.get_index(self.width, self.height, x, y);
        &self.data[index..index + self.channels()]
    }
}
