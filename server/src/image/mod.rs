use magick_rust::{ColorspaceType, DitherMethod, MagickError, MagickWand, magick_wand_genesis};
use std::sync::Once;

static START: Once = Once::new();

pub fn image_to_bmp(image_bytes: &[u8]) -> Result<Vec<u8>, MagickError> {
    START.call_once(|| {
        magick_wand_genesis();
    });

    let mut wand = MagickWand::new();
    wand.read_image_blob(image_bytes)?;

    wand.fit(800, 480);
    wand.set_option("dither:diffusion-amount", "85%")?;

    wand.quantize_image(
        2,
        ColorspaceType::GRAY,
        8,
        DitherMethod::FloydSteinberg,
        true,
    )?;

    wand.set_image_depth(1)?;

    wand.write_image_blob("BMP3")
}
