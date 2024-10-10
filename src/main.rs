mod cli;
mod conversion;

use cli::Cli;
use clap::Parser;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    let img = image::open(&cli.input)?;

    if let Some(avif_path) = cli.avif_output.as_ref() {
        println!("Conversion en AVIF avec une qualité de {}...", cli.quality);
        conversion::to_avif(&img, avif_path, cli.quality)?;
        println!("Image AVIF sauvegardée.");
    }

    if let Some(webp_path) = cli.webp_output.as_ref() {
        println!("Conversion en WebP avec une qualité de {}...", cli.quality);
        conversion::to_webp(&img, webp_path, cli.quality)?;
        println!("Image WebP sauvegardée.");
    }

    println!("Conversion terminée !");
    Ok(())
}
