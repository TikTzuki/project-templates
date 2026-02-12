<div align="center">

# vibe-generate

**Stop setting up projects. Start building them.**

A single binary that scaffolds production-ready projects from opinionated, best-practice templates — with built-in
AI-assisted development support.

[![Release](https://img.shields.io/github/v/release/tiktzuki/project-templates?style=flat-square&color=blue)](https://github.com/TikTzuki/project-templates/releases/latest)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue?style=flat-square)](LICENSE-APACHE)
[![Stars](https://img.shields.io/github/stars/TikTzuki/project-templates?style=flat-square)](https://github.com/TikTzuki/project-templates/stargazers)

[Install](#install) &bull; [Templates](#templates) &bull; [Usage](#usage) &bull; [Add Your Own](#add-a-template)
&bull; [Claude Code Plugin](#claude-code-plugin)

</div>

---

## Why?

Every new project starts the same way: copy an old project, rip out the business logic, fix the config, update
dependencies, wonder why the build is broken. Repeat.

**vibe-generate** fixes this. One command gives you a fully configured, production-grade project with:

- Modern tooling and latest stable dependencies
- Multi-module/multi-crate architecture that scales
- `CLAUDE.md` files so AI coding assistants understand your stack conventions from the start
- Zero runtime dependencies — templates are embedded in the binary at compile time

```bash
vibe-generate --template nextjs --name my-app
# Done. Start coding.
```

## Templates

| Template       | Stack                                                               | What you get                                                                                       |
|----------------|---------------------------------------------------------------------|----------------------------------------------------------------------------------------------------|
| **`nextjs`**   | Next.js 16 &bull; tRPC v11 &bull; Tailwind CSS 4 &bull; Drizzle ORM | Full-stack TypeScript app with type-safe API layer, App Router, Turbopack, and Zod validation      |
| **`rust-1.9`** | Rust 2024 &bull; clap 4 &bull; tracing &bull; tokio &bull; axum     | Multi-crate workspace with centralized deps, workspace lints, and optional web/crypto/CLI features |
| **`java-25`**  | Spring Boot 4.0 &bull; Gradle 9.3 &bull; Java 25                    | Multi-module project with version catalog, Spotless formatting, and dynamic module discovery       |

Each template is opinionated and battle-tested — not a minimal hello-world, but a real project structure you'd actually
use in production.

## Install

Grab the latest binary from [GitHub Releases](https://github.com/TikTzuki/project-templates/releases/latest):

<details>
<summary><b>macOS</b></summary>

```bash
# Apple Silicon (M1/M2/M3/M4)
curl -fsSL https://github.com/TikTzuki/project-templates/releases/latest/download/vibe-generate-aarch64-apple-darwin.tar.gz | tar xz
sudo mv vibe-generate /usr/local/bin/

# Intel
curl -fsSL https://github.com/TikTzuki/project-templates/releases/latest/download/vibe-generate-x86_64-apple-darwin.tar.gz | tar xz
sudo mv vibe-generate /usr/local/bin/
```

</details>

<details>
<summary><b>Linux</b></summary>

```bash
# x86_64
curl -fsSL https://github.com/TikTzuki/project-templates/releases/latest/download/vibe-generate-x86_64-unknown-linux-gnu.tar.gz | tar xz
sudo mv vibe-generate /usr/local/bin/

# aarch64
curl -fsSL https://github.com/TikTzuki/project-templates/releases/latest/download/vibe-generate-aarch64-unknown-linux-gnu.tar.gz | tar xz
sudo mv vibe-generate /usr/local/bin/
```

</details>

<details>
<summary><b>Windows</b></summary>

Download `vibe-generate-x86_64-pc-windows-msvc.zip` from
the [releases page](https://github.com/TikTzuki/project-templates/releases/latest), extract it, and add the directory to
your `PATH`.

</details>

<details>
<summary><b>Build from source</b></summary>

```bash
git clone https://github.com/TikTzuki/project-templates.git
cd project-templates
cargo install --path vibe-generate
```

Requires Rust 1.70+.

</details>

## Usage

```bash
# Interactive — pick a template from a menu
vibe-generate --name my-project

# Direct — skip the menu
vibe-generate --template nextjs --name my-app

# Custom output directory
vibe-generate --template rust-1.9 --name my-cli --output-dir ~/projects
```

## Claude Code Plugin

vibe-generate ships as a [Claude Code](https://docs.anthropic.com/en/docs/claude-code) plugin, so you can scaffold
projects directly from your AI coding session:

```
/plugin marketplace add TikTzuki/project-templates
/plugin install vibe-generate@TikTzuki/project-templates
```

Then use the `/new` command:

```
/new nextjs my-app
/new rust-1.9 my-cli
/new java-25 my-service ~/projects
```

Every generated project includes a `CLAUDE.md` file tailored to its stack, so Claude Code immediately understands the
project conventions, architecture, and tooling.

## How It Works

1. Templates live in the `templates/` directory, each as a complete project scaffold
2. At compile time, `include_dir` embeds all templates into the binary
3. At runtime, the selected template is extracted and all `{{project-name}}` placeholders are replaced with your project
   name
4. The result is a ready-to-go project — no post-processing needed

The binary is fully self-contained. No internet connection, no package registry, no external dependencies.

## Add a Template

1. Create a directory under `templates/` (e.g., `templates/python-fastapi/`)
2. Build out your project structure using `{{project-name}}` as a placeholder wherever the project name should appear
3. Include a `CLAUDE.md` with stack conventions for AI-assisted development
4. Rebuild: `cargo build -p vibe-generate`

Templates are just directories of files. No special config format, no template language — just real project files with a
single placeholder.

## Project Structure

```
project-templates/
├── vibe-generate/          # CLI tool (Rust)
│   └── src/
│       ├── main.rs         # Entry point + template discovery
│       ├── cli.rs          # Argument parsing (clap)
│       └── scaffold.rs     # Template copying + placeholder replacement
├── templates/
│   ├── nextjs/             # Next.js 16 + tRPC + Tailwind
│   ├── rust-1.9/           # Multi-crate Rust workspace
│   └── java-25/            # Spring Boot 4.0 + Gradle
├── skills/                 # Claude Code skill definitions
└── .github/workflows/      # Cross-platform release CI
```

## Contributing

Contributions are welcome — especially new templates. If you have an opinionated project setup that you keep reusing, it
probably belongs here.

1. Fork the repo
2. Add your template under `templates/`
3. Make sure `{{project-name}}` placeholders work correctly
4. Open a PR

## License

Apache-2.0