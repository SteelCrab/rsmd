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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_scan_markdown_files() {
        let temp_dir = tempdir().unwrap();
        let dir_path = temp_dir.path();

        // Create test .md files
        let mut file1 = File::create(dir_path.join("test1.md")).unwrap();
        writeln!(file1, "# Test 1").unwrap();

        let mut file2 = File::create(dir_path.join("test2.md")).unwrap();
        writeln!(file2, "# Test 2").unwrap();

        // Create a non-markdown file
        File::create(dir_path.join("readme.txt")).unwrap();

        let result = scan_markdown_files(dir_path.to_str().unwrap()).unwrap();
        assert_eq!(result.len(), 2);
        assert!(result.iter().any(|f| f.name == "test1.md"));
        assert!(result.iter().any(|f| f.name == "test2.md"));
    }

    #[test]
    fn test_scan_empty_directory() {
        let temp_dir = tempdir().unwrap();
        let result = scan_markdown_files(temp_dir.path().to_str().unwrap()).unwrap();
        assert_eq!(result.len(), 0);
    }
}
