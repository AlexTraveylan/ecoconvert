mod cli;
mod conversion;
mod resize;

use cli::Cli;
use clap::Parser;
use std::error::Error;
use resize::{ResizeOption, resize_image};

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    let mut img = image::open(&cli.input)?;

    if let Some(resize_option) = get_resize_option(&cli)? {
        img = resize_image(&img, resize_option);
    }

    let converter = conversion::get_converter(cli.output.extension()
        .and_then(|ext| ext.to_str())
        .ok_or("Invalid file extension")?)?;

    converter.convert(&img, &cli.output, cli.quality)?;
    Ok(())
}

fn get_resize_option(cli: &Cli) -> Result<Option<ResizeOption>, Box<dyn Error>> {
    if let Some(resize) = &cli.resize {
        if resize.ends_with('%') {
            let percent = resize.trim_end_matches('%').parse::<f32>()?;
            Ok(Some(ResizeOption::Percentage(percent)))
        } else {
            Err("Invalid resize format. Use percentage (e.g., 50%)".into())
        }
    } else if let (Some(width), Some(height)) = (cli.width, cli.height) {
        Ok(Some(ResizeOption::Dimensions(width, height)))
    } else if let Some(width) = cli.width {
        Ok(Some(ResizeOption::Width(width)))
    } else if let Some(height) = cli.height {
        Ok(Some(ResizeOption::Height(height)))
    } else {
        Ok(None)
    }
}
