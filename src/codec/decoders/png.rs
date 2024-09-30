use flate2::read::ZlibDecoder;
use flate2::Compression;

use std::fs::File;
use std::io::{self, Read};
use std::vec;

use crate::core::color::{Color, ColorSpace};
use crate::core::image::Image;
use crate::error::Error;

pub fn decode(file: &mut File) -> Result<Image, Error> {
    let mut signature = [0; 8];
    file.read_exact(&mut signature);

    if &signature != b"\x89PNG\r\n\x1a\n" {
        return Err(Error::ImageDecodeError(std::io::Error::new(
            io::ErrorKind::InvalidData,
            "Invalid PNG Signature",
        )));
    }

    fn read_chunk<R: Read>(reader: &mut R) -> io::Result<([u8; 4], Vec<u8>)> {
        let mut length_bytes = [0; 4];
        reader.read_exact(&mut length_bytes);

        let length = u32::from_be_bytes(length_bytes);

        let mut chunk_type = [0; 4];
        reader.read_exact(&mut chunk_type)?;

        let mut data = vec![0; length as usize];
        reader.read_exact(&mut data)?;

        let mut crc_bytes = [0; 4];
        reader.read_exact(&mut crc_bytes)?;

        Ok((chunk_type, data))
    }

    let mut width = 0;
    let mut height = 0;
    let mut image_data = Vec::new();
    'outer: loop {
        let (chunk_type, chunk_data) = read_chunk(file).unwrap();

        match &chunk_type {
            b"IHDR" => {
                width = u32::from_be_bytes([
                    chunk_data[0],
                    chunk_data[1],
                    chunk_data[2],
                    chunk_data[3],
                ]);
                height = u32::from_be_bytes([
                    chunk_data[4],
                    chunk_data[5],
                    chunk_data[6],
                    chunk_data[7],
                ]);
            }
            b"IDAT" => {
                image_data.extend(chunk_data);
            }
            b"IEND" => {
                break 'outer;
            }
            _ => {}
        }
    }

    let mut zlib_decoder = ZlibDecoder::new(&image_data[..]);
    let mut decompressed = Vec::new();
    zlib_decoder.read_to_end(&mut decompressed)?;

    let bytes_per_pixel = 3; // RGB
    let row_size = width as usize * bytes_per_pixel;
    let mut data = Vec::with_capacity(row_size * height as usize);

    for y in 0..height as usize {
        let filter_type = decompressed[y * (row_size + 1)];
        if filter_type != 0 {
            println!("Filter type Not handled yet");
        }
        let start = y * (row_size + 1) + 1;
        let end = start + row_size;

        data.extend_from_slice(&decompressed[start..end]);
    }

    Ok(Image::from_data(data, width, height, ColorSpace::RGB))
}
