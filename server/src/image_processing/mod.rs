use image::{
    ImageBuffer, ImageFormat, ImageResult, Luma,
    imageops::{BiLevel, FilterType},
};

use std::io::Cursor;

pub fn image_to_bin(image_bytes: &[u8]) -> Vec<u8> {
    let image = image::load_from_memory(&image_bytes)
        .unwrap()
        .resize_to_fill(800, 480, FilterType::Nearest);

    let mut grayscale_image = image.to_luma8();

    image::imageops::dither(&mut grayscale_image, &BiLevel);

    // Write to fs for debugging
    bmp_to_fs(&grayscale_image).unwrap_or_else(|e| {
        eprintln!("Failed to write to filesystem: {}", e);
    });

    let raw_image = grayscale_image.into_raw();
    let packed = pack_bytes(&raw_image);

    packed
}

// Convert 1 byte/pixel to 8 pixels/byte
fn pack_bytes(pixels: &[u8]) -> Vec<u8> {
    let mut packed = Vec::with_capacity((pixels.len() + 7) / 8);

    for _ in pixels.chunks(800) {
        for chunk in pixels.chunks(8) {
            let mut byte = 0u8;

            for (i, &p) in chunk.iter().enumerate() {
                if p == 1 {
                    byte |= 1 << (7 - i);
                }
            }

            packed.push(byte);
        }
    }

    packed
}

fn bmp_to_fs(image: &ImageBuffer<Luma<u8>, Vec<u8>>) -> ImageResult<()> {
    let mut buf = Vec::new();

    image.write_to(&mut Cursor::new(&mut buf), ImageFormat::Bmp)?;

    std::fs::write("output.bmp", &buf).unwrap();

    Ok(())
}
