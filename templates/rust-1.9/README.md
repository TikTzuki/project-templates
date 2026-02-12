# {{project-name}}

Rust multi-crate workspace.

## Prerequisites

- [Rust](https://rustup.rs/) (stable toolchain)

## Getting started

```bash
cargo build
cargo run -p {{project-name}}-cli
cargo run -p {{project-name}}-cli -- --name Rust
```

## Project structure

```
crates/
├── core/       Shared library (error types, utilities)
└── cli/        CLI binary
examples/
└── hello/      Example using the core library
```

## Adding a crate

Create a directory under `crates/` with a `Cargo.toml` that inherits workspace settings:

```toml
[package]
name = "{{project-name}}-mycrate"
version.workspace = true
edition.workspace = true

[dependencies]
# use workspace = true for shared deps

[lints]
workspace = true
```

It will be auto-discovered via `members = ["crates/*"]`.

## Workspace commands

| Command                             | Description            |
|-------------------------------------|------------------------|
| `cargo build`                       | Build all crates       |
| `cargo test`                        | Run all tests          |
| `cargo clippy`                      | Lint all crates        |
| `cargo run -p {{project-name}}-cli` | Run the CLI            |
| `cargo doc --open`                  | Generate and open docs |

## Environment variables

| Variable   | Default | Description      |
|------------|---------|------------------|
| `RUST_LOG` | `info`  | Log level filter |
