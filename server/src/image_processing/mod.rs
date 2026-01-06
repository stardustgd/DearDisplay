use image::{
    ImageBuffer, ImageFormat, ImageResult, Luma,
    imageops::{BiLevel, FilterType},
};

use std::io::Cursor;

pub fn image_to_bin(image_bytes: &[u8]) -> Vec<u8> {
    let image =
        image::load_from_memory(&image_bytes)
            .unwrap()
            .resize_exact(800, 480, FilterType::Nearest);
    // .to_luma8();

    let mut grayscale_image = image.to_luma8();

    image::imageops::dither(&mut grayscale_image, &BiLevel);

    // let mut img = image.clone();
    //
    // floyd_steinberg(&mut img);

    // Write to fs for debugging
    bmp_to_fs(&grayscale_image).unwrap_or_else(|e| {
        eprintln!("Failed to write to filesystem: {}", e);
    });

    pack_bytes(&grayscale_image)
}

// Source: https://en.wikipedia.org/wiki/Floyd%E2%80%93Steinberg_dithering
fn floyd_steinberg(img: &mut ImageBuffer<Luma<u8>, Vec<u8>>) {
    let (width, height) = img.dimensions();

    for y in 0..height {
        for x in 0..width {
            let old_pixel = img.get_pixel(x, y)[0] as i16;
            let new_pixel = if old_pixel < 128 { 0 } else { 255 };
            let quant_error = old_pixel - new_pixel;

            img.put_pixel(x, y, Luma([new_pixel as u8]));

            if x + 1 < width {
                let p = img.get_pixel(x + 1, y)[0] as i16 + (quant_error * 7 / 16).clamp(0, 255);
                img.put_pixel(x + 1, y, Luma([p as u8]))
            }
            if x > 0 && y + 1 < height {
                let p =
                    img.get_pixel(x - 1, y + 1)[0] as i16 + (quant_error * 3 / 16).clamp(0, 255);
                img.put_pixel(x - 1, y + 1, Luma([p as u8]))
            }
            if y + 1 < height {
                let p = img.get_pixel(x, y + 1)[0] as i16 + (quant_error * 5 / 16).clamp(0, 255);
                img.put_pixel(x, y + 1, Luma([p as u8]));
            }
            if x + 1 < width && y + 1 < height {
                let p =
                    img.get_pixel(x + 1, y + 1)[0] as i16 + (quant_error * 1 / 16).clamp(0, 255);
                img.put_pixel(x + 1, y + 1, Luma([p as u8]));
            }
        }
    }
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
    }

    res
}

fn bmp_to_fs(image: &ImageBuffer<Luma<u8>, Vec<u8>>) -> ImageResult<()> {
    let mut buf = Vec::new();

    image.write_to(&mut Cursor::new(&mut buf), ImageFormat::Bmp)?;

    std::fs::write("output.bmp", &buf).unwrap();

    Ok(())
}
