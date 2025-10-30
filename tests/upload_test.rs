use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use rsmd::{
    directory::MarkdownFile,
    i18n::Language,
    server::{AppState, create_router},
};
use serde_json::Value;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use tower::util::ServiceExt;

fn directory_state(temp_dir: &tempfile::TempDir) -> Arc<AppState> {
    Arc::new(AppState::Directory {
        dir_path: temp_dir.path().display().to_string(),
        files: Arc::new(RwLock::new(vec![])),
        file_cache: Arc::new(RwLock::new(HashMap::new())),
        language: Language::English,
        base_dir: temp_dir.path().to_path_buf(),
    })
}

#[tokio::test]
async fn upload_requires_file_name_header() {
    let temp_dir = tempfile::tempdir().unwrap();
    let state = directory_state(&temp_dir);
    let app = create_router(state);

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/upload")
                .body(Body::from("# Missing header"))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let payload: Value = serde_json::from_slice(
        &axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap(),
    )
    .unwrap();

    assert_eq!(payload["success"], Value::Bool(false));
    assert_eq!(
        payload["message"],
        Value::String("Failed to upload file.".to_string())
    );
}

#[tokio::test]
async fn upload_rejects_empty_file_name() {
    let temp_dir = tempfile::tempdir().unwrap();
    let state = directory_state(&temp_dir);
    let app = create_router(state);

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/upload")
                .header("x-file-name", "   ")
                .body(Body::from("# Empty name"))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let payload: Value = serde_json::from_slice(
        &axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap(),
    )
    .unwrap();

    assert_eq!(payload["success"], Value::Bool(false));
    assert_eq!(
        payload["message"],
        Value::String("Failed to upload file.".to_string())
    );
}

#[tokio::test]
async fn upload_rejects_invalid_extension() {
    let temp_dir = tempfile::tempdir().unwrap();
    let state = directory_state(&temp_dir);
    let app = create_router(state);

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/upload")
                .header("x-file-name", "notes.txt")
                .body(Body::from("# Invalid extension"))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let payload: Value = serde_json::from_slice(
        &axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap(),
    )
    .unwrap();

    assert_eq!(payload["success"], Value::Bool(false));
    assert_eq!(
        payload["message"],
        Value::String("Only markdown (.md) files are supported.".to_string())
    );
}

#[tokio::test]
async fn upload_rejects_empty_body() {
    let temp_dir = tempfile::tempdir().unwrap();
    let state = directory_state(&temp_dir);
    let app = create_router(state);

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/upload")
                .header("x-file-name", "empty.md")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let payload: Value = serde_json::from_slice(
        &axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap(),
    )
    .unwrap();

    assert_eq!(payload["success"], Value::Bool(false));
    assert_eq!(
        payload["message"],
        Value::String("Failed to upload file.".to_string())
    );
}

#[tokio::test]
async fn upload_succeeds_and_updates_state() {
    let temp_dir = tempfile::tempdir().unwrap();
    let base_dir = temp_dir.path().to_path_buf();

    let files = Arc::new(RwLock::new(vec![MarkdownFile {
        name: "existing.md".to_string(),
        path: base_dir.join("existing.md"),
    }]));

    let mut cache_map = HashMap::new();
    cache_map.insert(
        "guides/notes.md".to_string(),
        ("# old".to_string(), "<p>old</p>".to_string()),
    );

    let state = Arc::new(AppState::Directory {
        dir_path: base_dir.display().to_string(),
        files: files.clone(),
        file_cache: Arc::new(RwLock::new(cache_map)),
        language: Language::English,
        base_dir: base_dir.clone(),
    });

    let app = create_router(state.clone());

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/upload")
                .header("x-file-name", "../notes.md")
                .header("x-directory-path", "guides")
                .body(Body::from("# New content"))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let payload: Value = serde_json::from_slice(
        &axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap(),
    )
    .unwrap();

    assert_eq!(payload["success"], Value::Bool(true));
    assert_eq!(
        payload["file"],
        Value::String("guides/notes.md".to_string())
    );

    let expected_path = base_dir.join("guides").join("notes.md");
    let contents = tokio::fs::read_to_string(&expected_path).await.unwrap();
    assert!(contents.contains("New content"));

    if let AppState::Directory { file_cache, .. } = state.as_ref() {
        let cache_guard = file_cache.read().await;
        assert!(!cache_guard.contains_key("guides/notes.md"));
    } else {
        panic!("expected directory state");
    }

    let guard = files.read().await;
    let uploaded = guard
        .iter()
        .find(|item| item.name == "guides/notes.md")
        .expect("uploaded file should be present");
    assert_eq!(uploaded.path, expected_path);
}

#[tokio::test]
async fn upload_rejected_in_single_file_mode() {
    let state = Arc::new(AppState::SingleFile {
        markdown_content: "# Single".to_string(),
        html_content: "<h1>Single</h1>".to_string(),
        language: Language::English,
        base_dir: PathBuf::from("."),
    });

    let app = create_router(state);

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/upload")
                .header("x-file-name", "notes.md")
                .body(Body::from("# Should fail"))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn upload_rejects_invalid_directory_path() {
    let temp_dir = tempfile::tempdir().unwrap();
    let state = directory_state(&temp_dir);
    let app = create_router(state);

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/upload")
                .header("x-file-name", "notes.md")
                .header("x-directory-path", "../outside")
                .body(Body::from("# Should fail"))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}
