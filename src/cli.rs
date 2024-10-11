use std::path::PathBuf;
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Input image file path
    #[arg(short, long)]
    pub input: PathBuf,

    /// Output file path (AVIF or WebP)
    #[arg(short, long)]
    pub output: PathBuf,                        

    /// Compression quality (0-100, default 50)
    #[arg(short, long, default_value = "50")]
    pub quality: f32,
}
