use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use rsmd::{
    directory::MarkdownFile, i18n::Language, server::{create_router, AppState},
};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tower::util::ServiceExt;

#[tokio::test]
async fn test_directory_page_renders() {
    let state = Arc::new(AppState::Directory {
        dir_path: "/test".to_string(),
        files: vec![
            MarkdownFile {
                name: "test.md".to_string(),
                path: PathBuf::from("/test/test.md"),
            },
            MarkdownFile {
                name: "another.md".to_string(),
                path: PathBuf::from("/test/another.md"),
            },
        ],
        file_cache: Arc::new(HashMap::new()),
        language: Language::English,
        base_dir: PathBuf::from("/test"),
    });

    let app = create_router(state);

    let response = app
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let body_str = String::from_utf8(body.to_vec()).unwrap();

    // Check that directory page contains file links
    assert!(body_str.contains("test.md"));
    assert!(body_str.contains("another.md"));
    assert!(body_str.contains("data-load"));
}

#[tokio::test]
async fn test_dynamic_loading_with_xhr_header() {
    // Create a temporary test file
    let temp_dir = tempfile::tempdir().unwrap();
    let test_file = temp_dir.path().join("test.md");
    std::fs::write(&test_file, "# Test Header\n\nTest content").unwrap();

    let mut cache = HashMap::new();
    cache.insert(
        "test.md".to_string(),
        ("# Test Header\n\nTest content".to_string(), "<h1>Test Header</h1>\n<p>Test content</p>\n".to_string()),
    );

    let state = Arc::new(AppState::Directory {
        dir_path: temp_dir.path().to_str().unwrap().to_string(),
        files: vec![MarkdownFile {
            name: "test.md".to_string(),
            path: test_file.clone(),
        }],
        file_cache: Arc::new(cache),
        language: Language::English,
        base_dir: temp_dir.path().to_path_buf(),
    });

    let app = create_router(state);

    // Test with X-Requested-With header (fetch request)
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/content/test.md")
                .header("X-Requested-With", "XMLHttpRequest")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let body_str = String::from_utf8(body.to_vec()).unwrap();

    // Check that partial content is returned (not full page with <html> tags)
    assert!(!body_str.contains("<!DOCTYPE html>"));
    assert!(body_str.contains("Test Header"));
}

#[tokio::test]
async fn test_dynamic_loading_without_xhr_header() {
    let temp_dir = tempfile::tempdir().unwrap();
    let test_file = temp_dir.path().join("test.md");
    std::fs::write(&test_file, "# Test").unwrap();

    let state = Arc::new(AppState::Directory {
        dir_path: temp_dir.path().to_str().unwrap().to_string(),
        files: vec![MarkdownFile {
            name: "test.md".to_string(),
            path: test_file.clone(),
        }],
        file_cache: Arc::new(HashMap::new()),
        language: Language::English,
        base_dir: temp_dir.path().to_path_buf(),
    });

    let app = create_router(state);

    // Test without X-Requested-With header (direct browser request)
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/content/test.md")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let body_str = String::from_utf8(body.to_vec()).unwrap();

    // Should return redirect script for non-AJAX requests
    assert!(body_str.contains("window.location.reload"));
}

#[tokio::test]
async fn test_file_not_found() {
    let temp_dir = tempfile::tempdir().unwrap();

    let state = Arc::new(AppState::Directory {
        dir_path: temp_dir.path().to_str().unwrap().to_string(),
        files: vec![],
        file_cache: Arc::new(HashMap::new()),
        language: Language::English,
        base_dir: temp_dir.path().to_path_buf(),
    });

    let app = create_router(state);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/content/nonexistent.md")
                .header("X-Requested-With", "XMLHttpRequest")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let body_str = String::from_utf8(body.to_vec()).unwrap();

    // Should contain error message
    assert!(body_str.contains("not found") || body_str.contains("Not found"));
}

#[tokio::test]
async fn test_korean_language_support() {
    let temp_dir = tempfile::tempdir().unwrap();

    let state = Arc::new(AppState::Directory {
        dir_path: temp_dir.path().to_str().unwrap().to_string(),
        files: vec![],
        file_cache: Arc::new(HashMap::new()),
        language: Language::Korean,
        base_dir: temp_dir.path().to_path_buf(),
    });

    let app = create_router(state);

    let response = app
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let body_str = String::from_utf8(body.to_vec()).unwrap();

    // Check that Korean text is present
    assert!(body_str.contains("마크다운") || body_str.contains("lang=\"ko\""));
}
