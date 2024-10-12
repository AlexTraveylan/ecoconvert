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

    /// Resize percentage (e.g., 50%)
    #[arg(long)]
    pub resize: Option<String>,

    /// Target width for resizing
    #[arg(short = 'w', long)]
    pub width: Option<u32>,

    /// Target height for resizing
    #[arg(short = 'h', long)]
    pub height: Option<u32>,
}
