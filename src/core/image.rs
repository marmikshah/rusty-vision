use std::ops::{Index, IndexMut};

use log::debug;

use crate::{codec::Codex, core::color, io::reader::Reader};

use super::color::{Color, ColorSpace};

pub struct Image {
    /*
        Images are stored as row major.
        Formula: (y * self.width + x) * number-of-channels

        For example:
            A 3 x 3 Image is stored as a 1D array of length 27
            Which looks something like:
            [
                R00, G00, B00,
                R01, G01, B01,
                R02, G02, B02
                R10, G10, B10
                R11, G11, B11,
                R12, G12, B12,
                R20, G20, B20,
                R21, G21, B21,
                R22, G22, B22
            ]

        Now if we want to get all 3 channels at index (x, y)
        where
            x = 1,
            y = 1,
            image width = 3
            number-of-channels = 3 (For an RGB)

        index = (1 * 3 + 1) * 3
              = (12)
        So 12 .. (12 + 3) => (R11, G11, B11)
    */
    width: u32,
    height: u32,
    data: Vec<u8>,
    colorspace: ColorSpace,
}

impl Image {
    pub fn new(width: u32, height: u32, colorspace: ColorSpace) -> Self {
        let size = width as usize * height as usize * colorspace.channels();
        let data = vec![0; size];
        debug!("Creating Image of size {}", data.len());
        dbg!(data.len());
        Image {
            width,
            height,
            data,
            colorspace,
        }
    }

    pub fn from_data(data: Vec<u8>, width: u32, height: u32, colorspace: ColorSpace) -> Self {
        let size = width as usize * height as usize * colorspace.channels();
        dbg!(width as usize * height as usize * colorspace.channels());
        assert_eq!(data.len(), size);

        Image {
            width,
            height,
            data,
            colorspace,
        }
    }

    pub fn slice(&self, start: usize, end: usize) -> &[u8] {
        &self.data[start..end]
    }

    pub fn mut_slice(&mut self, start: usize, end: usize) -> &mut [u8] {
        &mut self.data[start..end]
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn size(&self) -> usize {
        self.height as usize * self.width as usize * self.colorspace.channels() as usize
    }

    fn get_index(&self, x: u32, y: u32) -> Option<usize> {
        let index = (y * self.width + x) as usize * self.colorspace.channels();
        if index < self.size() {
            Some(index)
        } else {
            None
            // TODO: ("Decide whether to panic here or not");
        }
    }
}

impl Index<(u32, u32)> for Image {
    type Output = [u8; 3];

    fn index(&self, (x, y): (u32, u32)) -> &Self::Output {
        let index = self.get_index(x, y).expect("Index out of bounds");
        let channels = self.colorspace.channels();
        let slice = &self.data[index..index + channels];
        slice.try_into().expect("Slice has incorrect length")
    }
}

impl IndexMut<(u32, u32)> for Image {
    fn index_mut(&mut self, (x, y): (u32, u32)) -> &mut Self::Output {
        let index = self.get_index(x, y).expect("Index out of bounds");
        let channels = self.colorspace.channels();
        let slice = &mut self.data[index..index + channels];
        slice.try_into().expect("Slice has incorrect length")
    }
}

impl Index<(u32, u32, u32)> for Image {
    type Output = u8;

    fn index(&self, (x, y, c): (u32, u32, u32)) -> &Self::Output {
        if c >= self.colorspace.channels() as u32 {
            panic!("Cannot get channel {c:?}")
        }
        let index = self.get_index(x, y).expect("Index out of bounds");
        let slice = &self.data[index + c as usize];
        slice.try_into().expect("Slice has incorrect length")
    }
}

impl IndexMut<(u32, u32, u32)> for Image {
    fn index_mut(&mut self, (x, y, c): (u32, u32, u32)) -> &mut Self::Output {
        if c >= self.colorspace.channels() as u32 {
            panic!("Cannot get channel {c:?}")
        }
        let index = self.get_index(x, y).expect("Index out of bounds");
        let slice = &mut self.data[index + c as usize];
        slice.try_into().expect("Slice has incorrect length")
    }
}
