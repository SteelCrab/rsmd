use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use rsmd::{
    directory::MarkdownFile,
    server::{AppState, FilesResponse, MarkdownResponse, create_router},
};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tower::util::ServiceExt;

#[tokio::test]
async fn test_api_get_files_empty() {
    let state = Arc::new(AppState::Directory {
        dir_path: "/test".to_string(),
        files: vec![],
        file_cache: Arc::new(HashMap::new()),
        language: rsmd::i18n::Language::English,
        base_dir: PathBuf::from("/test"),
    });

    let app = create_router(state);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/files")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: FilesResponse = serde_json::from_slice(&body).unwrap();

    assert_eq!(json.files.len(), 0);
}

#[tokio::test]
async fn test_api_get_files_with_data() {
    let state = Arc::new(AppState::Directory {
        dir_path: "/test".to_string(),
        files: vec![
            MarkdownFile {
                name: "test1.md".to_string(),
                path: PathBuf::from("/test/test1.md"),
            },
            MarkdownFile {
                name: "test2.md".to_string(),
                path: PathBuf::from("/test/test2.md"),
            },
        ],
        file_cache: Arc::new(HashMap::new()),
        language: rsmd::i18n::Language::English,
        base_dir: PathBuf::from("/test"),
    });

    let app = create_router(state);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/files")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: FilesResponse = serde_json::from_slice(&body).unwrap();

    assert_eq!(json.files.len(), 2);
    assert!(json.files.contains(&"test1.md".to_string()));
    assert!(json.files.contains(&"test2.md".to_string()));
}

#[tokio::test]
async fn test_api_get_markdown_from_cache() {
    let mut cache = HashMap::new();
    cache.insert(
        "test.md".to_string(),
        (
            "# Test Header\n\nContent".to_string(),
            "<h1>Test Header</h1>".to_string(),
        ),
    );

    let state = Arc::new(AppState::Directory {
        dir_path: "/test".to_string(),
        files: vec![MarkdownFile {
            name: "test.md".to_string(),
            path: PathBuf::from("/test/test.md"),
        }],
        file_cache: Arc::new(cache),
        language: rsmd::i18n::Language::English,
        base_dir: PathBuf::from("/test"),
    });

    let app = create_router(state);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/markdown/test.md")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: MarkdownResponse = serde_json::from_slice(&body).unwrap();

    assert_eq!(json.markdown, "# Test Header\n\nContent");
}

#[tokio::test]
async fn test_api_get_markdown_not_found() {
    let state = Arc::new(AppState::Directory {
        dir_path: "/test".to_string(),
        files: vec![],
        file_cache: Arc::new(HashMap::new()),
        language: rsmd::i18n::Language::English,
        base_dir: PathBuf::from("/test"),
    });

    let app = create_router(state);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/markdown/nonexistent.md")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: MarkdownResponse = serde_json::from_slice(&body).unwrap();

    assert!(json.markdown.contains("Error"));
    assert!(json.markdown.contains("not found"));
}

#[tokio::test]
async fn test_api_get_markdown_from_file() {
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    let temp_dir = tempdir().unwrap();
    let test_file = temp_dir.path().join("test.md");
    let mut file = File::create(&test_file).unwrap();
    writeln!(file, "# Real File\n\nThis is content").unwrap();

    let state = Arc::new(AppState::Directory {
        dir_path: temp_dir.path().to_str().unwrap().to_string(),
        files: vec![MarkdownFile {
            name: "test.md".to_string(),
            path: test_file.clone(),
        }],
        file_cache: Arc::new(HashMap::new()),
        language: rsmd::i18n::Language::English,
        base_dir: temp_dir.path().to_path_buf(),
    });

    let app = create_router(state);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/markdown/test.md")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: MarkdownResponse = serde_json::from_slice(&body).unwrap();

    assert!(json.markdown.contains("# Real File"));
    assert!(json.markdown.contains("This is content"));
}

#[tokio::test]
async fn test_api_get_markdown_read_error() {
    use tempfile::tempdir;

    let temp_dir = tempdir().unwrap();
    let missing_path = temp_dir.path().join("missing.md");
    std::fs::write(&missing_path, "# Missing\n\nContent").unwrap();

    let state = Arc::new(AppState::Directory {
        dir_path: temp_dir.path().to_str().unwrap().to_string(),
        files: vec![MarkdownFile {
            name: "missing.md".to_string(),
            path: missing_path.clone(),
        }],
        file_cache: Arc::new(HashMap::new()),
        language: rsmd::i18n::Language::English,
        base_dir: temp_dir.path().to_path_buf(),
    });

    // Remove the file so the server encounters a read error.
    std::fs::remove_file(missing_path).unwrap();

    let app = create_router(state);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/markdown/missing.md")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: MarkdownResponse = serde_json::from_slice(&body).unwrap();

    assert!(json.markdown.contains("Failed to read file"));
}
