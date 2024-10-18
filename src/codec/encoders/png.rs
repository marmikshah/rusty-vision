use flate2::write::ZlibEncoder;
use flate2::Compression;

use std::io::Write;

pub fn encode(image: &crate::core::image::Image) -> Result<Vec<u8>, crate::error::Error> {
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
    encoder.write_all(&raw_data)?;

    let compressed_data = encoder.finish()?;

    let idat_chunk = create_chunk(b"IDAT", &compressed_data);
    png_data.extend_from_slice(&idat_chunk);

    let iend_chunk = create_chunk(b"IEND", &[]);
    png_data.extend_from_slice(&iend_chunk);

    Ok(png_data)
}
