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
