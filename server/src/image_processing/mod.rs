use image::{
    ImageBuffer, ImageFormat, ImageResult, Luma,
    imageops::{BiLevel, FilterType},
};

use std::io::Cursor;

pub fn image_to_bmp(image_bytes: &[u8]) -> Vec<u8> {
    let image = image::load_from_memory(&image_bytes)
        .unwrap()
        .resize_to_fill(800, 480, FilterType::Nearest);

    let grayscale_image = image.to_luma8();

    let mut grayscale_image =
        image::imageops::resize(&grayscale_image, 800, 480, FilterType::Nearest);

    image::imageops::dither(&mut grayscale_image, &BiLevel);

    // Write to fs for debugging
    bmp_to_fs(&grayscale_image).unwrap_or_else(|e| {
        eprintln!("Failed to write to filesystem: {}", e);
    });

    grayscale_image.into_raw()
}

fn bmp_to_fs(image: &ImageBuffer<Luma<u8>, Vec<u8>>) -> ImageResult<()> {
    let mut buf = Vec::new();

    image.write_to(&mut Cursor::new(&mut buf), ImageFormat::Bmp)?;

    std::fs::write("output.bmp", &buf).unwrap();

    Ok(())
}
