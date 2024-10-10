use image::{DynamicImage, GenericImageView};
use ravif::{Encoder as AvifEncoder, Img, RGBA8};
use std::path::Path;
use std::error::Error;

pub fn to_avif(img: &DynamicImage, output_path: &Path, quality: f32) -> Result<(), Box<dyn Error>> {
    let (width, height) = img.dimensions();
    let rgba = img.to_rgba8();
    let encoder = AvifEncoder::new()
        .with_quality(quality)
        .with_alpha_quality(quality)
        .with_speed(5);
    let rgba_vec: Vec<RGBA8> = rgba.pixels().map(|p| RGBA8::new(p[0], p[1], p[2], p[3])).collect();
    let avif = encoder.encode_rgba(Img::new(rgba_vec.as_slice(), width as usize, height as usize))?;
    std::fs::write(output_path, avif.avif_file)?;
    Ok(())
}

pub fn to_webp(img: &DynamicImage, output_path: &Path, quality: f32) -> Result<(), Box<dyn Error>> {
    let rgba = img.to_rgba8();
    let encoder = webp::Encoder::from_rgba(rgba.as_raw(), img.width(), img.height());
    let webp = encoder.encode(quality);
    std::fs::write(output_path, webp.as_ref())?;
    Ok(())
}