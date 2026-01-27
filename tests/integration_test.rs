use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use rsmd::{
    directory::MarkdownFile,
    i18n::Language,
    server::{AppState, create_router},
};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use tower::util::ServiceExt;

#[tokio::test]
async fn test_directory_page_renders() {
    let state = Arc::new(AppState::Directory {
        dir_path: "/test".to_string(),
        files: Arc::new(RwLock::new(vec![
            MarkdownFile {
                name: "test.md".to_string(),
                path: PathBuf::from("/test/test.md"),
            },
            MarkdownFile {
                name: "another.md".to_string(),
                path: PathBuf::from("/test/another.md"),
            },
        ])),
        file_cache: Arc::new(RwLock::new(HashMap::new())),
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
    assert!(body_str.contains("directory-body"));
    assert!(body_str.contains("file-list file-entries"));
    assert!(body_str.contains(r#"href="/view/test.md""#));
    assert!(body_str.contains(r#"href="/view/another.md""#));
    assert!(body_str.contains("file-entry__path"));
    assert!(body_str.contains("data-current-path=\"\""));
    assert!(!body_str.contains("data-load"));
}

#[tokio::test]
async fn test_nested_directory_navigation() {
    let state = Arc::new(AppState::Directory {
        dir_path: "/content".to_string(),
        files: Arc::new(RwLock::new(vec![
            MarkdownFile {
                name: "guides/docker.md".to_string(),
                path: PathBuf::from("/content/guides/docker.md"),
            },
            MarkdownFile {
                name: "guides/rust.md".to_string(),
                path: PathBuf::from("/content/guides/rust.md"),
            },
            MarkdownFile {
                name: "guides/workflows/ci.md".to_string(),
                path: PathBuf::from("/content/guides/workflows/ci.md"),
            },
        ])),
        file_cache: Arc::new(RwLock::new(HashMap::new())),
        language: Language::English,
        base_dir: PathBuf::from("/content"),
    });

    let app = create_router(state);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/dir/guides")
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

    assert!(body_str.contains("folder-section"));
    assert!(body_str.contains("folder-card"));
    assert!(body_str.contains(r#"href="/dir/guides/workflows""#));
    assert!(body_str.contains(r#"href="/view/guides/docker.md""#));
    assert!(body_str.contains(r#"href="/view/guides/rust.md""#));
    assert!(body_str.contains("data-current-path=\"guides\""));
    assert!(body_str.contains("data-path=\"guides/workflows\""));
    assert!(body_str.contains("breadcrumbs"));
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
        (
            "# Test Header\n\nTest content".to_string(),
            "<h1>Test Header</h1>\n<p>Test content</p>\n".to_string(),
        ),
    );

    let state = Arc::new(AppState::Directory {
        dir_path: temp_dir.path().to_str().unwrap().to_string(),
        files: Arc::new(RwLock::new(vec![MarkdownFile {
            name: "test.md".to_string(),
            path: test_file.clone(),
        }])),
        file_cache: Arc::new(RwLock::new(cache)),
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
        files: Arc::new(RwLock::new(vec![MarkdownFile {
            name: "test.md".to_string(),
            path: test_file.clone(),
        }])),
        file_cache: Arc::new(RwLock::new(HashMap::new())),
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
        files: Arc::new(RwLock::new(vec![])),
        file_cache: Arc::new(RwLock::new(HashMap::new())),
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
async fn test_raw_route_serves_markdown() {
    let temp_dir = tempfile::tempdir().unwrap();
    let test_file = temp_dir.path().join("article.md");
    std::fs::write(&test_file, "# Article\n\nBody").unwrap();

    let state = Arc::new(AppState::Directory {
        dir_path: temp_dir.path().to_str().unwrap().to_string(),
        files: Arc::new(RwLock::new(vec![MarkdownFile {
            name: "article.md".to_string(),
            path: test_file.clone(),
        }])),
        file_cache: Arc::new(RwLock::new(HashMap::new())),
        language: Language::English,
        base_dir: temp_dir.path().to_path_buf(),
    });

    let app = create_router(state);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/raw/article.md")
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

    assert!(body_str.contains("# Article"));
    assert!(body_str.contains("Body"));
}

#[tokio::test]
async fn test_view_route_serves_markdown() {
    let temp_dir = tempfile::tempdir().unwrap();
    let test_file = temp_dir.path().join("view.md");
    std::fs::write(&test_file, "# View\n\nContent").unwrap();

    let state = Arc::new(AppState::Directory {
        dir_path: temp_dir.path().to_str().unwrap().to_string(),
        files: Arc::new(RwLock::new(vec![MarkdownFile {
            name: "view.md".to_string(),
            path: test_file.clone(),
        }])),
        file_cache: Arc::new(RwLock::new(HashMap::new())),
        language: Language::English,
        base_dir: temp_dir.path().to_path_buf(),
    });

    let app = create_router(state.clone());

    let response = app
        .oneshot(
            Request::builder()
                .uri("/view/view.md")
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

    assert!(body_str.contains("View"));
    assert!(body_str.contains("Content"));

    if let AppState::Directory { file_cache, .. } = state.as_ref() {
        let guard = file_cache.read().await;
        assert!(guard.contains_key("view.md"));
    }
}

#[tokio::test]
async fn test_view_route_serves_nested_markdown() {
    let temp_dir = tempfile::tempdir().unwrap();
    let nested_dir = temp_dir.path().join("docs");
    std::fs::create_dir_all(&nested_dir).unwrap();
    let test_file = nested_dir.join("guide.md");
    std::fs::write(&test_file, "# Guide\n\nDetails").unwrap();

    let state = Arc::new(AppState::Directory {
        dir_path: temp_dir.path().to_str().unwrap().to_string(),
        files: Arc::new(RwLock::new(vec![MarkdownFile {
            name: "docs/guide.md".to_string(),
            path: test_file.clone(),
        }])),
        file_cache: Arc::new(RwLock::new(HashMap::new())),
        language: Language::English,
        base_dir: temp_dir.path().to_path_buf(),
    });

    let app = create_router(state.clone());

    let response = app
        .oneshot(
            Request::builder()
                .uri("/view/docs/guide.md")
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

    assert!(body_str.contains("Guide"));
    assert!(body_str.contains("Details"));

    if let AppState::Directory { file_cache, .. } = state.as_ref() {
        let guard = file_cache.read().await;
        assert!(guard.contains_key("docs/guide.md"));
    }
}

#[tokio::test]
async fn test_partial_content_loads_from_disk_and_caches() {
    let temp_dir = tempfile::tempdir().unwrap();
    let test_file = temp_dir.path().join("live.md");
    std::fs::write(&test_file, "# Live\n\nCache me").unwrap();

    let state = Arc::new(AppState::Directory {
        dir_path: temp_dir.path().to_str().unwrap().to_string(),
        files: Arc::new(RwLock::new(vec![MarkdownFile {
            name: "live.md".to_string(),
            path: test_file.clone(),
        }])),
        file_cache: Arc::new(RwLock::new(HashMap::new())),
        language: Language::English,
        base_dir: temp_dir.path().to_path_buf(),
    });

    let app = create_router(state.clone());

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/content/live.md")
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

    assert!(body_str.contains("Live"));
    assert!(!body_str.contains("<!DOCTYPE html>"));

    if let AppState::Directory { file_cache, .. } = state.as_ref() {
        let guard = file_cache.read().await;
        assert!(guard.contains_key("live.md"));
    }
}

#[tokio::test]
async fn test_view_route_read_error() {
    let temp_dir = tempfile::tempdir().unwrap();
    let missing_path = temp_dir.path().join("broken.md");
    std::fs::write(&missing_path, "# Broken").unwrap();

    let state = Arc::new(AppState::Directory {
        dir_path: temp_dir.path().to_str().unwrap().to_string(),
        files: Arc::new(RwLock::new(vec![MarkdownFile {
            name: "broken.md".to_string(),
            path: missing_path.clone(),
        }])),
        file_cache: Arc::new(RwLock::new(HashMap::new())),
        language: Language::English,
        base_dir: temp_dir.path().to_path_buf(),
    });

    std::fs::remove_file(missing_path).unwrap();

    let app = create_router(state);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/view/broken.md")
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

    assert!(body_str.contains("Error reading file"));
}

#[tokio::test]
async fn test_partial_content_read_error() {
    let temp_dir = tempfile::tempdir().unwrap();
    let missing_path = temp_dir.path().join("missing.md");
    std::fs::write(&missing_path, "# Missing\n\nBody").unwrap();

    let state = Arc::new(AppState::Directory {
        dir_path: temp_dir.path().to_str().unwrap().to_string(),
        files: Arc::new(RwLock::new(vec![MarkdownFile {
            name: "missing.md".to_string(),
            path: missing_path.clone(),
        }])),
        file_cache: Arc::new(RwLock::new(HashMap::new())),
        language: Language::English,
        base_dir: temp_dir.path().to_path_buf(),
    });

    std::fs::remove_file(missing_path).unwrap();

    let app = create_router(state);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/content/missing.md")
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

    assert!(body_str.contains("Error reading file"));
}

#[tokio::test]
async fn test_korean_language_support() {
    let temp_dir = tempfile::tempdir().unwrap();

    let state = Arc::new(AppState::Directory {
        dir_path: temp_dir.path().to_str().unwrap().to_string(),
        files: Arc::new(RwLock::new(vec![])),
        file_cache: Arc::new(RwLock::new(HashMap::new())),
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

#[tokio::test]
async fn test_serve_directory_path_nested_folders() {
    let state = Arc::new(AppState::Directory {
        dir_path: "/test".to_string(),
        files: Arc::new(RwLock::new(vec![
            MarkdownFile {
                name: "docs/api.md".to_string(),
                path: PathBuf::from("/test/docs/api.md"),
            },
            MarkdownFile {
                name: "docs/guide.md".to_string(),
                path: PathBuf::from("/test/docs/guide.md"),
            },
            MarkdownFile {
                name: "readme.md".to_string(),
                path: PathBuf::from("/test/readme.md"),
            },
        ])),
        file_cache: Arc::new(RwLock::new(HashMap::new())),
        language: Language::English,
        base_dir: PathBuf::from("/test"),
    });

    let app = create_router(state);

    // Test navigating to /dir/docs
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/dir/docs")
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

    // Should show files in docs folder
    assert!(body_str.contains("api.md"));
    assert!(body_str.contains("guide.md"));
    // Should not show root level files
    assert!(!body_str.contains("readme.md"));
}

#[tokio::test]
async fn test_directory_navigation_with_path_traversal_attempt() {
    let state = Arc::new(AppState::Directory {
        dir_path: "/test".to_string(),
        files: Arc::new(RwLock::new(vec![MarkdownFile {
            name: "test.md".to_string(),
            path: PathBuf::from("/test/test.md"),
        }])),
        file_cache: Arc::new(RwLock::new(HashMap::new())),
        language: Language::English,
        base_dir: PathBuf::from("/test"),
    });

    let app = create_router(state);

    // Attempt path traversal
    let response = app
        .oneshot(
            Request::builder()
                .uri("/dir/../etc/passwd")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Should reject with BAD_REQUEST
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_directory_navigation_nonexistent_path() {
    let state = Arc::new(AppState::Directory {
        dir_path: "/test".to_string(),
        files: Arc::new(RwLock::new(vec![MarkdownFile {
            name: "test.md".to_string(),
            path: PathBuf::from("/test/test.md"),
        }])),
        file_cache: Arc::new(RwLock::new(HashMap::new())),
        language: Language::English,
        base_dir: PathBuf::from("/test"),
    });

    let app = create_router(state);

    // Navigate to non-existent directory
    let response = app
        .oneshot(
            Request::builder()
                .uri("/dir/nonexistent")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Should return NOT_FOUND
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_directory_mode_handles_file_requests() {
    // Create directory state
    let temp_dir = tempfile::tempdir().unwrap();
    let test_file = temp_dir.path().join("test.md");
    std::fs::write(&test_file, "# Test Content").unwrap();

    let state = Arc::new(AppState::Directory {
        dir_path: temp_dir.path().to_str().unwrap().to_string(),
        files: Arc::new(RwLock::new(vec![MarkdownFile {
            name: "test.md".to_string(),
            path: test_file.clone(),
        }])),
        file_cache: Arc::new(RwLock::new(HashMap::new())),
        language: Language::English,
        base_dir: temp_dir.path().to_path_buf(),
    });

    let app = create_router(state);

    // Directory mode should handle /view/filename requests
    let response = app
        .oneshot(
            Request::builder()
                .uri("/view/test.md")
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

    assert!(body_str.contains("Test Content"));
}

#[tokio::test]
async fn test_serve_raw_with_directory_state() {
    let state = Arc::new(AppState::Directory {
        dir_path: "/test".to_string(),
        files: Arc::new(RwLock::new(vec![MarkdownFile {
            name: "test.md".to_string(),
            path: PathBuf::from("/test/test.md"),
        }])),
        file_cache: Arc::new(RwLock::new(HashMap::new())),
        language: Language::English,
        base_dir: PathBuf::from("/test"),
    });

    let app = create_router(state);

    // Accessing /raw in directory mode should work via /raw/{filename}
    let response = app
        .oneshot(
            Request::builder()
                .uri("/raw/test.md")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_directory_navigation_root_path() {
    let state = Arc::new(AppState::Directory {
        dir_path: "/test".to_string(),
        files: Arc::new(RwLock::new(vec![
            MarkdownFile {
                name: "root.md".to_string(),
                path: PathBuf::from("/test/root.md"),
            },
            MarkdownFile {
                name: "docs/nested.md".to_string(),
                path: PathBuf::from("/test/docs/nested.md"),
            },
        ])),
        file_cache: Arc::new(RwLock::new(HashMap::new())),
        language: Language::English,
        base_dir: PathBuf::from("/test"),
    });

    let app = create_router(state);

    // Navigate to root directory via main page
    let response = app
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let body_str = String::from_utf8(body.to_vec()).unwrap();

    // Should show root level file and docs folder
    assert!(body_str.contains("root.md"));
    assert!(body_str.contains("docs"));
}

#[tokio::test]
async fn test_compare_route_serves_comparison_page() {
    let state = Arc::new(AppState::SingleFile {
        markdown_content: "# Test\n\nThis is a test.".to_string(),
        html_content: "<h1>Test</h1><p>This is a test.</p>".to_string(),
        language: Language::English,
        base_dir: PathBuf::from("/tmp"),
    });

    let app = create_router(state);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/compare")
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

    // Check that comparison page contains both rendered and raw content
    assert!(body_str.contains("<h1>Test</h1>"));
    assert!(body_str.contains("# Test"));
    assert!(body_str.contains("compare-container"));
    assert!(body_str.contains("compare-panel"));
    assert!(body_str.contains("rendered-content"));
    assert!(body_str.contains("raw-content"));
    assert!(body_str.contains("Markdown Comparison"));
    assert!(body_str.contains("Rendered View"));
    assert!(body_str.contains("Raw Markdown"));
}

#[tokio::test]
async fn test_compare_route_korean_language() {
    let state = Arc::new(AppState::SingleFile {
        markdown_content: "# 테스트\n\n이것은 테스트입니다.".to_string(),
        html_content: "<h1>테스트</h1><p>이것은 테스트입니다.</p>".to_string(),
        language: Language::Korean,
        base_dir: PathBuf::from("/tmp"),
    });

    let app = create_router(state);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/compare")
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

    // Check Korean language content
    assert!(body_str.contains("마크다운 비교"));
    assert!(body_str.contains("렌더링된 보기"));
    assert!(body_str.contains("원본 마크다운"));
    assert!(body_str.contains("lang=\"ko\""));
}
