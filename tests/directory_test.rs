use rsmd::directory::scan_markdown_files;
use std::fs::File;
use std::io::{ErrorKind, Write};
use tempfile::{NamedTempFile, tempdir};

#[test]
fn test_scan_markdown_files() {
    let temp_dir = tempdir().unwrap();
    let dir_path = temp_dir.path();

    // Create test .md files
    let mut file1 = File::create(dir_path.join("test1.md")).unwrap();
    writeln!(file1, "# Test 1").unwrap();

    let mut file2 = File::create(dir_path.join("test2.md")).unwrap();
    writeln!(file2, "# Test 2").unwrap();

    // Create nested directory with markdown file
    let nested_dir = dir_path.join("nested");
    std::fs::create_dir(&nested_dir).unwrap();
    let mut nested_file = File::create(nested_dir.join("inner.md")).unwrap();
    writeln!(nested_file, "# Nested").unwrap();

    // Create a non-markdown file
    File::create(dir_path.join("readme.txt")).unwrap();

    let result = scan_markdown_files(dir_path.to_str().unwrap()).unwrap();
    assert_eq!(result.len(), 3);
    assert!(result.iter().any(|f| f.name == "test1.md"));
    assert!(result.iter().any(|f| f.name == "test2.md"));
    assert!(result.iter().any(|f| f.name == "nested/inner.md"));
}

#[test]
fn test_scan_empty_directory() {
    let temp_dir = tempdir().unwrap();
    let result = scan_markdown_files(temp_dir.path().to_str().unwrap()).unwrap();
    assert_eq!(result.len(), 0);
}

#[test]
fn test_scan_non_directory_returns_error() {
    let tempfile = NamedTempFile::new().unwrap();
    let err = scan_markdown_files(tempfile.path().to_str().unwrap()).unwrap_err();
    assert_eq!(err.kind(), ErrorKind::NotFound);
}
