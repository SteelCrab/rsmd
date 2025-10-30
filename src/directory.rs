use std::collections::BTreeSet;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// Represents a markdown file in a directory
#[derive(Clone, Debug)]
pub struct MarkdownFile {
    pub name: String,
    pub path: PathBuf,
}

/// Scans a directory for markdown files
pub fn scan_markdown_files(dir_path: &str) -> io::Result<Vec<MarkdownFile>> {
    let base_path = Path::new(dir_path);

    if !base_path.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Path is not a directory",
        ));
    }

    let mut md_files = Vec::new();

    fn visit(base: &Path, current: &Path, acc: &mut Vec<MarkdownFile>) -> io::Result<()> {
        for entry in fs::read_dir(current)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                visit(base, &path, acc)?;
            } else if path.is_file() && path.extension().map(|ext| ext == "md").unwrap_or(false) {
                let relative_name = path
                    .strip_prefix(base)
                    .unwrap_or(&path)
                    .to_string_lossy()
                    .replace('\\', "/");

                acc.push(MarkdownFile {
                    name: relative_name,
                    path: path.clone(),
                });
            }
        }

        Ok(())
    }

    visit(base_path, base_path, &mut md_files)?;

    // Sort alphabetically
    md_files.sort_by(|a, b| a.name.cmp(&b.name));

    Ok(md_files)
}

/// Represents a sub-directory entry in the markdown library
#[derive(Clone, Debug)]
pub struct DirectoryEntry {
    pub name: String,
    pub path: String,
}

/// Represents the current directory view (sub-directories + files)
#[derive(Clone, Debug)]
pub struct DirectoryListing {
    pub current_path: String,
    pub parent: Option<String>,
    pub directories: Vec<DirectoryEntry>,
    pub files: Vec<MarkdownFile>,
}

/// Build a directory listing for the provided relative path
pub fn list_directory_contents(all_files: &[MarkdownFile], current_path: &str) -> DirectoryListing {
    let normalized = current_path.trim_matches('/');
    let prefix = if normalized.is_empty() {
        String::new()
    } else {
        format!("{}/", normalized)
    };

    let mut directories = BTreeSet::new();
    let mut files = Vec::new();

    for file in all_files {
        let name = &file.name;
        if !prefix.is_empty() {
            if !name.starts_with(&prefix) {
                continue;
            }

            let remainder = &name[prefix.len()..];
            if remainder.is_empty() {
                continue;
            }

            if let Some(pos) = remainder.find('/') {
                directories.insert(remainder[..pos].to_string());
            } else {
                files.push(file.clone());
            }
        } else if let Some(pos) = name.find('/') {
            directories.insert(name[..pos].to_string());
        } else {
            files.push(file.clone());
        }
    }

    files.sort_by(|a, b| a.name.cmp(&b.name));

    let directories = directories
        .into_iter()
        .map(|name| {
            let path = if normalized.is_empty() {
                name.clone()
            } else {
                format!("{}/{}", normalized, name)
            };
            DirectoryEntry { name, path }
        })
        .collect();

    let parent = if normalized.is_empty() {
        None
    } else if let Some((parent, _)) = normalized.rsplit_once('/') {
        Some(parent.to_string())
    } else {
        Some(String::new())
    };

    DirectoryListing {
        current_path: normalized.to_string(),
        parent,
        directories,
        files,
    }
}
