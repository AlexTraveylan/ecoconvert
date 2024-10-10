use std::path::PathBuf;
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Chemin du fichier image d'entrée
    #[arg(short, long)]
    pub input: PathBuf,

    /// Chemin du fichier de sortie AVIF
    #[arg(short = 'a', long)]
    pub avif_output: Option<PathBuf>,

    /// Chemin du fichier de sortie WebP
    #[arg(short = 'w', long)]
    pub webp_output: Option<PathBuf>,

    /// Qualité de compression (0-100, par défaut 50)
    #[arg(short = 'q', long, default_value = "50")]
    pub quality: f32,
}