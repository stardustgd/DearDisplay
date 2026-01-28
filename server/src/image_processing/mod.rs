use image::{
    ImageBuffer, ImageError, ImageFormat, ImageResult, Luma,
    imageops::{BiLevel, FilterType},
};

use std::io::Cursor;

pub fn image_to_bin(image_bytes: &[u8]) -> Result<Vec<u8>, ImageError> {
    let image = image::load_from_memory(&image_bytes)?.resize_exact(800, 480, FilterType::Nearest);

    let mut grayscale_image = image.to_luma8();

    image::imageops::dither(&mut grayscale_image, &BiLevel);

    // Write to fs for debugging
    bmp_to_fs(&grayscale_image)?;

    Ok(pack_bytes(&grayscale_image))
}

// Convert 1 byte/pixel to 8 pixels/byte
fn pack_bytes(img: &ImageBuffer<Luma<u8>, Vec<u8>>) -> Vec<u8> {
    let (width, height) = img.dimensions();
    let mut res = Vec::with_capacity((width * height / 8) as usize);

    for y in 0..height {
        let mut byte = 0u8;
        let mut bit = 7;

        for x in 0..width {
            let pixel = img.get_pixel(x, y)[0];

            if pixel < 128 {
                byte |= 1 << bit;
            }

            if bit == 0 {
                res.push(byte);
                byte = 0;
                bit = 7;
            } else {
                bit -= 1;
            }
        }

        if bit != 7 {
            res.push(byte);
        }
    }

    res
}

fn bmp_to_fs(image: &ImageBuffer<Luma<u8>, Vec<u8>>) -> ImageResult<()> {
    let mut buf = Vec::new();

    image.write_to(&mut Cursor::new(&mut buf), ImageFormat::Bmp)?;
    std::fs::write("output.bmp", &buf)?;

    Ok(())
}
