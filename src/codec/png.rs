use flate2::Compression;
use flate2::{read::ZlibDecoder, write::ZlibEncoder};

use crate::{color::ColorSpace, image::Image};
use crate::{error::Error, geometry::Shape};
use std::{
    fs::File,
    io::{Read, Write},
};

pub fn encode(image: &Image) -> Result<Vec<u8>, Error> {
    let png_signature = b"\x89PNG\r\n\x1a\n";
    let mut png_data = Vec::new();
    png_data.extend_from_slice(png_signature);

    fn create_chunk(chunk_type: &[u8], data: &[u8]) -> Vec<u8> {
        let mut chunk = Vec::new();
        chunk.extend_from_slice(&(data.len() as u32).to_be_bytes());
        chunk.extend_from_slice(chunk_type);
        chunk.extend_from_slice(data);
        let checksum = crc32fast::hash(&chunk[4..]);
        chunk.extend_from_slice(&checksum.to_be_bytes());
        chunk
    }

    let ihdr = [
        (image.width() >> 24) as u8,
        (image.width() >> 16) as u8,
        (image.width() >> 8) as u8,
        image.width() as u8,
        (image.height() >> 24) as u8,
        (image.height() >> 16) as u8,
        (image.height() >> 8) as u8,
        image.height() as u8,
        8, // Bit depth
        2, // Color type (truecolor)
        0, // Compresison method
        0, // Filter method
        0, // Interlace method
    ];
    let ihdr_chunk = create_chunk(b"IHDR", &ihdr);
    png_data.extend_from_slice(&ihdr_chunk);

    let mut raw_data = Vec::with_capacity((1 + image.width() * 3) * image.height());
    // y = 0, idx = 0 .. 512 * 3 (Gets the first row of all 3 channels)
    for y in 0..image.height() {
        raw_data.push(0);

        let start: usize = y * image.width() * 3;
        let end: usize = (y + 1) * image.width() * 3;

        // Take one row of all three channels.
        raw_data.extend_from_slice(image.slice(start, end))
    }

    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder
        .write_all(&raw_data)
        .map_err(|e| Error::ImageEncodeError(format!("Failed to write PNG data: {}", e)))?;

    let compressed_data = encoder
        .finish()
        .map_err(|e| Error::ImageEncodeError(format!("Failed to compress PNG data: {}", e)))?;

    let idat_chunk = create_chunk(b"IDAT", &compressed_data);
    png_data.extend_from_slice(&idat_chunk);

    let iend_chunk = create_chunk(b"IEND", &[]);
    png_data.extend_from_slice(&iend_chunk);

    Ok(png_data)
}

pub fn decode(file: &mut File) -> Result<Image, Error> {
    let mut signature = [0; 8];
    file.read_exact(&mut signature)
        .map_err(|e| Error::ImageDecodeError(format!("Failed to read PNG signature: {}", e)))?;

    if &signature != b"\x89PNG\r\n\x1a\n" {
        return Err(Error::ImageDecodeError("Invalid PNG Signature".to_string()));
    }

    fn read_chunk<R: Read>(reader: &mut R) -> Result<([u8; 4], Vec<u8>), Error> {
        let mut length_bytes = [0; 4];
        reader
            .read_exact(&mut length_bytes)
            .map_err(|e| Error::ImageDecodeError(format!("Failed to read chunk length: {}", e)))?;

        let length = u32::from_be_bytes(length_bytes);

        let mut chunk_type = [0; 4];
        reader
            .read_exact(&mut chunk_type)
            .map_err(|e| Error::ImageDecodeError(format!("Failed to read chunk type: {}", e)))?;

        let mut data = vec![0; length as usize];
        reader
            .read_exact(&mut data)
            .map_err(|e| Error::ImageDecodeError(format!("Failed to read chunk data: {}", e)))?;

        let mut crc_bytes = [0; 4];
        reader
            .read_exact(&mut crc_bytes)
            .map_err(|e| Error::ImageDecodeError(format!("Failed to read chunk CRC: {}", e)))?;

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
                width = u32::from_be_bytes(chunk_data[0..4].try_into().unwrap());
                height = u32::from_be_bytes(chunk_data[4..8].try_into().unwrap());

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
            value => {
                panic!(
                    "Block type {} decoding has not been implemented",
                    String::from_utf8_lossy(value)
                );
            }
        }
    }

    println!("{width:?}, {height:?}");
    dbg!(palette.len());

    let mut zlib_decoder = ZlibDecoder::new(&image_data[..]);
    let mut decompressed = Vec::new();
    zlib_decoder
        .read_to_end(&mut decompressed)
        .map_err(|e| Error::ImageDecodeError(format!("Failed to decompress PNG data: {}", e)))?;

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

    // TODO: Change to implicit ndim
    let shape = Shape::new(width as usize, height as usize, Some(4));

    Ok(Image::from_data(data, shape, ColorSpace::RGBA))
}
