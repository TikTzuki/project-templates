---
name: new
description: Scaffold a new project from a template using vibe-generate. Use when the user wants to create a new project.
disable-model-invocation: true
argument-hint: "[template] [project-name] [output-dir]"
allowed-tools: Bash, Read
---

Scaffold a new project using the `vibe-generate` CLI tool.

## Arguments

The user may provide: $ARGUMENTS

Parse the arguments to determine:

- **template**: The template name (e.g., `nextjs`, `rust-1.9`, `java-25`)
- **name**: The project name
- **output-dir** (optional): Where to create the project

## Instructions

1. First, make sure `vibe-generate` is installed. If not, install it:
   ```
   cargo install --git https://github.com/tiktuzki/project-templates vibe-generate
   ```

2. If no arguments were provided, or they're incomplete, first list available templates:
   ```
   vibe-generate --help
   ```
   Then ask the user which template and project name they want.

3. If both template and name are clear from the arguments, run:
   ```
   vibe-generate --template <template> --name <name>
   ```
   If the user specified an output directory, add `--output-dir <path>`.

4. After scaffolding, read the generated project's CLAUDE.md to understand the stack, then give the user a brief summary of what was created and suggest next steps (e.g., `cd <name> && npm install` or `cargo build`).

## Examples

- `/vibe-generate:new nextjs my-app` -> `vibe-generate --template nextjs --name my-app`
- `/vibe-generate:new rust-1.9 my-cli` -> `vibe-generate --template rust-1.9 --name my-cli`
- `/vibe-generate:new java-25 my-service ~/projects` -> `vibe-generate --template java-25 --name my-service --output-dir ~/projects`
- `/vibe-generate:new` (no args) -> list templates and ask user interactively
