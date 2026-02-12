use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use fs_extra::dir::{self, CopyOptions};
use include_dir::Dir;
use walkdir::WalkDir;

/// Copy the template directory into `output_dir/project_name` and replace every
/// occurrence of the `{{project-name}}` placeholder in file contents with the
/// real project name.
pub fn scaffold(template_dir: &Path, output_dir: &Path, project_name: &str) -> io::Result<()> {
    let dest = output_dir.join(project_name);

    if dest.exists() {
        return Err(io::Error::new(
            io::ErrorKind::AlreadyExists,
            format!("Destination already exists: {}", dest.display()),
        ));
    }

    // --- copy the whole template tree ----------------------------------------
    let mut opts = CopyOptions::new();
    opts.copy_inside = true;

    dir::copy(template_dir, &dest, &opts).map_err(|e| {
        io::Error::new(
            io::ErrorKind::Other,
            format!("Failed to copy template directory: {e}"),
        )
    })?;

    // --- replace placeholders in every file ----------------------------------
    replace_placeholders(&dest, project_name)?;

    Ok(())
}

/// Scaffold from embedded (compile-time) templates.
pub fn scaffold_embedded(
    embedded: &Dir,
    template_name: &str,
    output_dir: &Path,
    project_name: &str,
) -> io::Result<()> {
    let dest = output_dir.join(project_name);

    if dest.exists() {
        return Err(io::Error::new(
            io::ErrorKind::AlreadyExists,
            format!("Destination already exists: {}", dest.display()),
        ));
    }

    let template_dir = embedded.get_dir(template_name).ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::NotFound,
            format!("Embedded template not found: {template_name}"),
        )
    })?;

    // Extract embedded files to disk.
    extract_dir(template_dir, &dest)?;

    // Replace placeholders.
    replace_placeholders(&dest, project_name)?;

    Ok(())
}

/// Recursively extract an embedded directory to disk.
fn extract_dir(dir: &Dir, dest: &Path) -> io::Result<()> {
    fs::create_dir_all(dest)?;

    for file in dir.files() {
        let file_name = file
            .path()
            .file_name()
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "invalid embedded file path"))?;
        let out_path = dest.join(file_name);
        fs::write(&out_path, file.contents())?;
    }

    for subdir in dir.dirs() {
        let dir_name = subdir
            .path()
            .file_name()
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "invalid embedded dir path"))?;
        extract_dir(subdir, &dest.join(dir_name))?;
    }

    Ok(())
}

/// Walk `root` recursively and replace `{{project-name}}` in every regular
/// file.
fn replace_placeholders(root: &Path, project_name: &str) -> io::Result<()> {
    for entry in WalkDir::new(root).into_iter().filter_map(Result::ok) {
        let path = entry.path();
        if !path.is_file() {
            continue;
        }

        // Only process files that look like text (skip binary blobs).
        if let Ok(contents) = fs::read_to_string(path) {
            if contents.contains("{{project-name}}") {
                let replaced = contents.replace("{{project-name}}", project_name);
                fs::write(path, replaced)?;
            }
        }
    }

    Ok(())
}

/// Discover available templates by listing sub-directories of `templates_root`.
pub fn list_templates(templates_root: &Path) -> io::Result<Vec<String>> {
    let mut templates: Vec<String> = Vec::new();

    for entry in fs::read_dir(templates_root)? {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            if let Some(name) = entry.file_name().to_str() {
                templates.push(name.to_string());
            }
        }
    }

    templates.sort();
    Ok(templates)
}

/// List templates from embedded (compile-time) data.
pub fn list_templates_embedded(embedded: &Dir) -> Vec<String> {
    let mut templates: Vec<String> = embedded
        .dirs()
        .filter_map(|d| d.path().file_name()?.to_str().map(String::from))
        .collect();
    templates.sort();
    templates
}

/// Resolve the absolute path to a template directory.
pub fn resolve_template_dir(templates_root: &Path, template_name: &str) -> PathBuf {
    templates_root.join(template_name)
}
