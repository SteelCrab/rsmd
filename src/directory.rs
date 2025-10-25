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
    let path = Path::new(dir_path);

    if !path.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Path is not a directory",
        ));
    }

    let mut md_files = Vec::new();

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file()
            && let Some(ext) = path.extension()
            && ext == "md"
            && let Some(name) = path.file_name()
        {
            md_files.push(MarkdownFile {
                name: name.to_string_lossy().to_string(),
                path: path.clone(),
            });
        }
    }

    // Sort alphabetically
    md_files.sort_by(|a, b| a.name.cmp(&b.name));

    Ok(md_files)
}
