# vibe-generate

CLI tool that scaffolds new projects from the boilerplate templates in this
repository.

## Building

```bash
cd generators
cargo build --release
```

The binary is written to `target/release/vibe-generate`.

## Usage

```text
vibe-generate [OPTIONS] --name <NAME>

Options:
  -t, --template <TEMPLATE>      Template to use (e.g. "nextjs"). Omit for interactive selection.
  -n, --name <NAME>              Name of the new project.
  -o, --output-dir <OUTPUT_DIR>  Where to create the project folder (default: current directory).
  -h, --help                     Print help.
  -V, --version                  Print version.
```

### Examples

Interactive mode (presents a menu of available templates):

```bash
vibe-generate --name my-app
```

Specify the template directly:

```bash
vibe-generate --template nextjs --name my-app
```

Create the project in a custom directory:

```bash
vibe-generate --template nextjs --name my-app --output-dir ~/projects
```

## How it works

1. The tool looks for a `templates/` directory next to the `generators/` crate
   (or walks up from the current working directory).
2. Each sub-directory inside `templates/` is treated as an available template.
3. The chosen template is copied into `<output-dir>/<project-name>`.
4. Every occurrence of `{{project-name}}` in the copied files is replaced with
   the actual project name.

## Adding a new template

Drop a new directory under `templates/` at the repository root. Use
`{{project-name}}` anywhere you want the project name to be substituted
(file contents, `package.json`, `Cargo.toml`, etc.).
