use axum::{body::Body, http::Request};
use rsmd::{
    i18n::Language,
    server::{AppState, create_router},
};
use std::sync::Arc;
use tempfile::tempdir;
use tower::util::ServiceExt;

#[tokio::test]
async fn test_single_file_routes_render_content() {
    let temp_dir = tempdir().unwrap();
    let state = Arc::new(AppState::SingleFile {
        markdown_content: "# Heading".to_string(),
        html_content: "<h1>Heading</h1>".to_string(),
        language: Language::English,
        base_dir: temp_dir.path().to_path_buf(),
    });

    let app = create_router(state.clone());

    let html_response = app
        .clone()
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();
    let html_body = axum::body::to_bytes(html_response.into_body(), usize::MAX)
        .await
        .unwrap();
    let html_str = String::from_utf8(html_body.to_vec()).unwrap();
    assert!(html_str.contains("<h1>Heading</h1>"));

    let raw_response = app
        .oneshot(Request::builder().uri("/raw").body(Body::empty()).unwrap())
        .await
        .unwrap();
    let raw_body = axum::body::to_bytes(raw_response.into_body(), usize::MAX)
        .await
        .unwrap();
    let raw_str = String::from_utf8(raw_body.to_vec()).unwrap();
    assert!(raw_str.contains("# Heading"));
}
