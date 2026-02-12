Scaffold a new project using vibe-generate.

The user wants to create a new project. Use the `vibe-generate` CLI tool (installed via cargo) to scaffold it.

## Arguments

The user may provide: $ARGUMENTS

Parse the arguments to determine:

- **template**: The template name (e.g., `nextjs`, `rust-1.9`, `java-25`)
- **name**: The project name

## Instructions

1. If no arguments were provided, or they're incomplete, first list available templates:
   ```
   vibe-generate --help
   ```
   Then ask the user which template and project name they want.

2. If both template and name are clear from the arguments, run:
   ```
   vibe-generate --template <template> --name <name>
   ```
   By default this scaffolds into the current directory. If the user specified an output directory, add
   `--output-dir <path>`.

3. After scaffolding, read the generated project's CLAUDE.md to understand the stack, then give the user a brief summary
   of what was created and suggest next steps (e.g., `cd <name> && npm install` or `cargo build`).

## Examples

- `/new nextjs my-app` → `vibe-generate --template nextjs --name my-app`
- `/new rust-1.9 my-cli` → `vibe-generate --template rust-1.9 --name my-cli`
- `/new java-25 my-service ~/projects` → `vibe-generate --template java-25 --name my-service --output-dir ~/projects`
- `/new` (no args) → list templates and ask user interactively
