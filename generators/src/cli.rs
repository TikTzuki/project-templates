use clap::Parser;
use std::path::PathBuf;

/// Scaffold a new project from a boilerplate template.
#[derive(Parser, Debug)]
#[command(name = "vibe-generate", version, about)]
pub struct Cli {
    /// Template to use (e.g. "nextjs", "rust-cli"). If omitted, an interactive
    /// selection menu is shown.
    #[arg(short, long)]
    pub template: Option<String>,

    /// Name of the new project (used as the output directory name and for
    /// placeholder replacement).
    #[arg(short, long)]
    pub name: String,

    /// Directory where the project folder will be created. Defaults to the
    /// current working directory.
    #[arg(short, long)]
    pub output_dir: Option<PathBuf>,
}
