//! CLI entry point.

use clap::Parser;

/// {{project-name}} CLI
#[derive(Debug, Parser)]
#[command(version, about)]
struct Cli {
    /// Name to greet
    #[arg(short, long, default_value = "world")]
    name: String,
}

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info".into()),
        )
        .init();

    let cli = Cli::parse();

    tracing::info!(version = project_core::version(), "starting");
    println!("Hello, {}!", cli.name);

    Ok(())
}
