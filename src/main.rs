use std::path::PathBuf;
use clap::Parser;
use image::GenericImageView;
use ravif::{Img, RGBA8};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Chemin du fichier image d'entrée
    #[arg(short, long)]
    input: PathBuf,

    /// Chemin du fichier de sortie AVIF
    #[arg(short = 'a', long)]
    avif_output: Option<PathBuf>,

    /// Chemin du fichier de sortie WebP
    #[arg(short = 'w', long)]
    webp_output: Option<PathBuf>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let img = image::open(&cli.input)?;

    if let Some(avif_path) = cli.avif_output {
        println!("Conversion en AVIF...");
        let (width, height) = img.dimensions();
        let rgba = img.to_rgba8();
        let encoder = ravif::Encoder::new()
            .with_quality(80.0)
            .with_alpha_quality(80.0)
            .with_speed(5);
        let rgba_vec: Vec<RGBA8> = rgba.pixels().map(|p| RGBA8::new(p[0], p[1], p[2], p[3])).collect();
        let avif = encoder.encode_rgba(Img::new(rgba_vec.as_slice(), width as usize, height as usize))?;
        std::fs::write(avif_path, avif.avif_file)?;
        println!("Image AVIF sauvegardée.");
    }

    if let Some(webp_path) = cli.webp_output {
        println!("Conversion en WebP...");
        let rgba = img.to_rgba8();
        let encoder = webp::Encoder::from_rgba(rgba.as_raw(), img.width(), img.height());
        let webp = encoder.encode(80.0);
        std::fs::write(webp_path, webp.as_ref())?;
        println!("Image WebP sauvegardée.");
    }

    println!("Conversion terminée !");
    Ok(())
}
