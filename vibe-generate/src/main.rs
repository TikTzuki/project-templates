mod cli;
mod scaffold;

use std::env;
use std::path::PathBuf;
use std::process;

use clap::Parser;
use console::Style;
use dialoguer::Select;
use include_dir::{include_dir, Dir};

use cli::Cli;
use scaffold::{
    list_templates, list_templates_embedded, resolve_template_dir, scaffold, scaffold_embedded,
};

/// All templates are embedded at compile time so the binary is self-contained.
static EMBEDDED_TEMPLATES: Dir = include_dir!("$CARGO_MANIFEST_DIR/../templates");

/// Locate the `templates/` directory on the filesystem (for local development).
fn find_templates_root() -> Option<PathBuf> {
    // During `cargo run` the manifest dir is set.
    if let Ok(manifest) = env::var("CARGO_MANIFEST_DIR") {
        let candidate = PathBuf::from(manifest).join("../templates");
        if candidate.is_dir() {
            return Some(candidate.canonicalize().ok()?);
        }
    }

    // Fallback: walk up from the current working directory.
    let mut dir = env::current_dir().ok()?;
    loop {
        let candidate = dir.join("templates");
        if candidate.is_dir() {
            return Some(candidate);
        }
        if !dir.pop() {
            break;
        }
    }

    None
}

/// Represents where templates come from.
enum TemplateSource {
    Filesystem(PathBuf),
    Embedded,
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    let bold = Style::new().bold();
    let green = Style::new().green().bold();
    let red = Style::new().red().bold();

    // Prefer filesystem templates (local dev), fall back to embedded.
    let source = match find_templates_root() {
        Some(root) => TemplateSource::Filesystem(root),
        None => TemplateSource::Embedded,
    };

    // Discover available templates.
    let available = match &source {
        TemplateSource::Filesystem(root) => list_templates(root)?,
        TemplateSource::Embedded => list_templates_embedded(&EMBEDDED_TEMPLATES),
    };

    if available.is_empty() {
        eprintln!("{} No templates found", red.apply_to("Error:"));
        process::exit(1);
    }

    // Pick a template — either from the CLI flag or via interactive selection.
    let template_name = match args.template {
        Some(t) => {
            if !available.contains(&t) {
                eprintln!(
                    "{} Unknown template \"{}\". Available: {}",
                    red.apply_to("Error:"),
                    t,
                    available.join(", ")
                );
                process::exit(1);
            }
            t
        }
        None => {
            let selection = Select::new()
                .with_prompt("Select a template")
                .items(&available)
                .default(0)
                .interact()?;
            available[selection].clone()
        }
    };

    let output_dir = args
        .output_dir
        .unwrap_or_else(|| env::current_dir().expect("cannot determine current directory"));

    println!(
        "{} Scaffolding project {} from template {}...",
        bold.apply_to("=>"),
        green.apply_to(&args.name),
        green.apply_to(&template_name),
    );

    match &source {
        TemplateSource::Filesystem(root) => {
            let template_dir = resolve_template_dir(root, &template_name);
            scaffold(&template_dir, &output_dir, &args.name)?;
        }
        TemplateSource::Embedded => {
            scaffold_embedded(&EMBEDDED_TEMPLATES, &template_name, &output_dir, &args.name)?;
        }
    }

    println!(
        "\n{} Project {} created at {}/{}",
        green.apply_to("Success!"),
        bold.apply_to(&args.name),
        output_dir.display(),
        &args.name,
    );
    println!(
        "\n  cd {}/{} && get started!",
        output_dir.display(),
        &args.name
    );

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        let red = Style::new().red().bold();
        eprintln!("{} {e}", red.apply_to("Error:"));
        process::exit(1);
    }
}
