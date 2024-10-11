mod cli;
mod conversion;

use cli::Cli;
use clap::Parser;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    let img = image::open(&cli.input)?;

    let converter = conversion::get_converter(cli.output.extension()
        .and_then(|ext| ext.to_str())
        .ok_or("Invalid file extension")?)?;

    println!("Converting with quality {}...", cli.quality);
    converter.convert(&img, &cli.output, cli.quality)?;
    println!("Image converted and saved.");

    println!("Conversion completed!");
    Ok(())
}
