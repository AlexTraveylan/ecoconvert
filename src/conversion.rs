use image::{DynamicImage, GenericImageView};
use ravif::{Encoder as AvifEncoder, Img, RGBA8};
use std::path::Path;
use std::error::Error;

pub trait ImageConverter {
    fn convert(&self, img: &DynamicImage, output_path: &Path, quality: f32) -> Result<(), Box<dyn Error>>;
}

struct AvifConverter;
struct WebPConverter;

impl ImageConverter for AvifConverter {
    fn convert(&self, img: &DynamicImage, output_path: &Path, quality: f32) -> Result<(), Box<dyn Error>> {
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
}

impl ImageConverter for WebPConverter {
    fn convert(&self, img: &DynamicImage, output_path: &Path, quality: f32) -> Result<(), Box<dyn Error>> {
        let rgba = img.to_rgba8();
        let encoder = webp::Encoder::from_rgba(rgba.as_raw(), img.width(), img.height());
        let webp = encoder.encode(quality);
        std::fs::write(output_path, webp.as_ref())?;
        Ok(())
    }
}

pub fn get_converter(extension: &str) -> Result<Box<dyn ImageConverter>, Box<dyn Error>> {
    match extension.to_lowercase().as_str() {
        "avif" => Ok(Box::new(AvifConverter)),
        "webp" => Ok(Box::new(WebPConverter)),
        _ => Err("Format de fichier non pris en charge".into()),
    }
}
