# project-templates

A template-based monorepo for scaffolding new projects. Pick a template, run the generator, and start building.

## Templates

| Template   | Description                                                       |
|------------|-------------------------------------------------------------------|
| `rust-1.9` | Multi-crate Rust workspace with clap, tracing, anyhow + thiserror |
| `nextjs`   | Next.js 16 App Router + tRPC v11 + Tailwind CSS 4 + Drizzle ORM   |
| `java-25`  | Spring Boot 4.0 multi-module project with Gradle 9.3              |

## Installation

### Install via cargo (from git)

```bash
cargo install --git https://github.com/tiktuzki/project-templates vibe-generate
```

This installs a self-contained `vibe-generate` binary with all templates embedded.

### Build from source

```bash
git clone https://github.com/tiktuzki/project-templates
cd project-templates
cargo install --path generators
```

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

### Claude Code slash command

If you use [Claude Code](https://docs.anthropic.com/en/docs/claude-code), copy the custom command to enable `/scaffold`:

```bash
mkdir -p ~/.claude/commands
cp .claude-commands/scaffold.md ~/.claude/commands/scaffold.md
```

Then from any Claude Code session:

```
/scaffold nextjs my-app
/scaffold rust-1.9 my-cli
/scaffold java-25 my-service ~/projects
```

## How it works

The generator copies the selected template into a new directory and replaces all `{{project-name}}` placeholders with
your project name. Templates are embedded into the binary at compile time using `include_dir`, so the installed binary
is fully self-contained.

When running from the repo (via `cargo run`), filesystem templates are used directly for faster iteration.

## Project Structure

```
├── templates/
│   ├── rust-1.9/          # Rust workspace starter
│   ├── nextjs/            # Next.js + tRPC starter
│   └── java-25/           # Spring Boot multi-module starter
├── generators/            # Rust CLI tool (vibe-generate)
└── README.md
```

## Adding a new template

1. Create a new directory under `templates/` (e.g., `templates/my-stack/`)
2. Use `{{project-name}}` as a placeholder anywhere you want the project name substituted
3. Include a `CLAUDE.md` with stack conventions for AI-assisted development
4. Rebuild: `cargo build -p vibe-generate` (templates are embedded at compile time)

## Requirements

- **Generator CLI**: Rust 1.70+ and Cargo
- **Rust templates**: Rust 1.85+
- **Next.js template**: Node.js 20+ and npm
- **Java template**: Java 25+ and Gradle 9.3+
