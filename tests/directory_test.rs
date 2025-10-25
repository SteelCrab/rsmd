use rsmd::directory::scan_markdown_files;
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
