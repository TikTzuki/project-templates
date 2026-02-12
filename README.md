# project-templates

A template-based monorepo for scaffolding new projects. Pick a template, run the generator, and start building.

## Templates

| Template   | Description                                                       |
|------------|-------------------------------------------------------------------|
| `rust-1.9` | Multi-crate Rust workspace with clap, tracing, anyhow + thiserror |
| `nextjs`   | Next.js 16 App Router + tRPC v11 + Tailwind CSS 4 + Drizzle ORM   |
| `java-25`  | Spring Boot 4.0 multi-module project with Gradle 9.3              |

## Installation

Grab the latest binary from [GitHub Releases](https://github.com/tiktuzki/project-templates/releases/latest):

```bash
# macOS (Apple Silicon)
curl -fsSL https://github.com/tiktuzki/project-templates/releases/latest/download/vibe-generate-aarch64-apple-darwin.tar.gz | tar xz
sudo mv vibe-generate /usr/local/bin/

# macOS (Intel)
curl -fsSL https://github.com/tiktuzki/project-templates/releases/latest/download/vibe-generate-x86_64-apple-darwin.tar.gz | tar xz
sudo mv vibe-generate /usr/local/bin/

# Linux (x86_64)
curl -fsSL https://github.com/tiktuzki/project-templates/releases/latest/download/vibe-generate-x86_64-unknown-linux-gnu.tar.gz | tar xz
sudo mv vibe-generate /usr/local/bin/

# Linux (aarch64)
curl -fsSL https://github.com/tiktuzki/project-templates/releases/latest/download/vibe-generate-aarch64-unknown-linux-gnu.tar.gz | tar xz
sudo mv vibe-generate /usr/local/bin/
```

On Windows, download `vibe-generate-x86_64-pc-windows-msvc.zip` from the releases page and add it to your PATH.

## Usage

### CLI

```bash
# Interactive mode — select template from a menu
vibe-generate --name my-project

# Specify template directly
vibe-generate --template nextjs --name my-app

# Custom output directory
vibe-generate --template rust-1.9 --name my-cli --output-dir ~/projects
```

### Claude Code plugin

Install as a [Claude Code](https://docs.anthropic.com/en/docs/claude-code) plugin to get the `/vibe-generate:new` slash
command:

```
/plugin marketplace add tiktuzki/project-templates
/plugin install vibe-generate@tiktuzki/project-templates
```

Then from any Claude Code session:

```
/vibe-generate:new nextjs my-app
/vibe-generate:new rust-1.9 my-cli
/vibe-generate:new java-25 my-service ~/projects
```

## How it works

The generator copies the selected template into a new directory and replaces all `{{project-name}}` placeholders with
your project name. Templates are embedded into the binary at compile time using `include_dir`, so the installed binary
is fully self-contained.

When running from the repo (via `cargo run`), filesystem templates are used directly for faster iteration.

## Adding a new template

1. Create a new directory under `templates/` (e.g., `templates/my-stack/`)
2. Use `{{project-name}}` as a placeholder anywhere you want the project name substituted
3. Include a `CLAUDE.md` with stack conventions for AI-assisted development
4. Rebuild: `cargo build -p vibe-generate` (templates are embedded at compile time)

## Requirements

- **Generator CLI**: Rust 1.70+ and Cargo