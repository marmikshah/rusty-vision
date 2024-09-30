use flate2::read::ZlibDecoder;

use std::fs::File;
use std::io::{self, Read};
use std::vec;

use crate::core::color::{self, ColorSpace};
use crate::core::image::Image;
use crate::error::Error;

pub fn decode(file: &mut File) -> Result<Image, Error> {
    let mut signature = [0; 8];
    file.read_exact(&mut signature)?;

    if &signature != b"\x89PNG\r\n\x1a\n" {
        return Err(Error::ImageDecodeError(std::io::Error::new(
            io::ErrorKind::InvalidData,
            "Invalid PNG Signature",
        )));
    }

    fn read_chunk<R: Read>(reader: &mut R) -> io::Result<([u8; 4], Vec<u8>)> {
        let mut length_bytes = [0; 4];
        reader.read_exact(&mut length_bytes)?;

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
    let mut colortype = 0;
    let mut palette = Vec::new();
    let mut image_data = Vec::new();
    'outer: loop {
        let (chunk_type, chunk_data) = read_chunk(file).unwrap();
        dbg!(String::from_utf8_lossy(&chunk_type));
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

                println!("Bit Depth: {}", chunk_data[8]);
                colortype = chunk_data[9];

                dbg!(chunk_data);
            }
            b"PLTE" => {
                dbg!(&chunk_data);
                palette = chunk_data;
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

    println!("{width:?}, {height:?}");
    dbg!(palette.len());

    let mut zlib_decoder = ZlibDecoder::new(&image_data[..]);
    let mut decompressed = Vec::new();
    zlib_decoder.read_to_end(&mut decompressed)?;

    println!("Decompressed size:{}", decompressed.len());

    // let bytes_per_pixel = colortype as usize; // RGB

    let row_size = width as usize;
    let mut data = Vec::with_capacity(width as usize * height as usize * 3);

    dbg!(data.capacity());

    for y in 0..height as usize {
        // let filter_type = decompressed[y * (row_size + 1)];

        let start = y * (row_size + 1) + 1;
        let end = start + row_size;

        match colortype {
            3 => {
                for &index in &decompressed[start..end] {
                    // dbg!(index);
                    let p_index = index as usize * 3;
                    data.extend_from_slice(&palette[p_index..p_index + 3]);
                }
            }
            _ => {
                data.extend_from_slice(&decompressed[start..end]);
            }
        }
    }

    Ok(Image::from_data(data, width, height, ColorSpace::RGBA))
}
