use rsmd::directory::{MarkdownFile, list_directory_contents, scan_markdown_files};
use std::fs::File;
use std::io::{ErrorKind, Write};
use std::path::PathBuf;
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

#[test]
fn test_list_directory_contents_root() {
    let files = vec![
        MarkdownFile {
            name: "readme.md".to_string(),
            path: PathBuf::from("/test/readme.md"),
        },
        MarkdownFile {
            name: "docs/api.md".to_string(),
            path: PathBuf::from("/test/docs/api.md"),
        },
        MarkdownFile {
            name: "docs/guide.md".to_string(),
            path: PathBuf::from("/test/docs/guide.md"),
        },
    ];

    let listing = list_directory_contents(&files, "");

    assert_eq!(listing.current_path, "");
    assert_eq!(listing.parent, None);
    assert_eq!(listing.directories.len(), 1);
    assert_eq!(listing.directories[0].name, "docs");
    assert_eq!(listing.directories[0].path, "docs");
    assert_eq!(listing.files.len(), 1);
    assert_eq!(listing.files[0].name, "readme.md");
}

#[test]
fn test_list_directory_contents_nested() {
    let files = vec![
        MarkdownFile {
            name: "docs/api.md".to_string(),
            path: PathBuf::from("/test/docs/api.md"),
        },
        MarkdownFile {
            name: "docs/guide.md".to_string(),
            path: PathBuf::from("/test/docs/guide.md"),
        },
        MarkdownFile {
            name: "docs/advanced/tutorial.md".to_string(),
            path: PathBuf::from("/test/docs/advanced/tutorial.md"),
        },
        MarkdownFile {
            name: "readme.md".to_string(),
            path: PathBuf::from("/test/readme.md"),
        },
    ];

    let listing = list_directory_contents(&files, "docs");

    assert_eq!(listing.current_path, "docs");
    assert_eq!(listing.parent, Some("".to_string()));
    assert_eq!(listing.directories.len(), 1);
    assert_eq!(listing.directories[0].name, "advanced");
    assert_eq!(listing.directories[0].path, "docs/advanced");
    assert_eq!(listing.files.len(), 2);
    assert!(listing.files.iter().any(|f| f.name == "docs/api.md"));
    assert!(listing.files.iter().any(|f| f.name == "docs/guide.md"));
}

#[test]
fn test_list_directory_contents_deep_nested() {
    let files = vec![
        MarkdownFile {
            name: "docs/api/v1/endpoints.md".to_string(),
            path: PathBuf::from("/test/docs/api/v1/endpoints.md"),
        },
        MarkdownFile {
            name: "docs/api/v2/endpoints.md".to_string(),
            path: PathBuf::from("/test/docs/api/v2/endpoints.md"),
        },
    ];

    let listing = list_directory_contents(&files, "docs/api");

    assert_eq!(listing.current_path, "docs/api");
    assert_eq!(listing.parent, Some("docs".to_string()));
    assert_eq!(listing.directories.len(), 2);
    assert!(listing.directories.iter().any(|d| d.name == "v1"));
    assert!(listing.directories.iter().any(|d| d.name == "v2"));
    assert_eq!(listing.files.len(), 0);
}

#[test]
fn test_list_directory_contents_empty_path() {
    let files = vec![MarkdownFile {
        name: "docs/nested/deep/file.md".to_string(),
        path: PathBuf::from("/test/docs/nested/deep/file.md"),
    }];

    let listing = list_directory_contents(&files, "");

    assert_eq!(listing.directories.len(), 1);
    assert_eq!(listing.directories[0].name, "docs");
    assert_eq!(listing.files.len(), 0);
}

#[test]
fn test_list_directory_contents_sorted() {
    let files = vec![
        MarkdownFile {
            name: "z.md".to_string(),
            path: PathBuf::from("/test/z.md"),
        },
        MarkdownFile {
            name: "a.md".to_string(),
            path: PathBuf::from("/test/a.md"),
        },
        MarkdownFile {
            name: "m.md".to_string(),
            path: PathBuf::from("/test/m.md"),
        },
    ];

    let listing = list_directory_contents(&files, "");

    // Files should be sorted alphabetically
    assert_eq!(listing.files[0].name, "a.md");
    assert_eq!(listing.files[1].name, "m.md");
    assert_eq!(listing.files[2].name, "z.md");
}
