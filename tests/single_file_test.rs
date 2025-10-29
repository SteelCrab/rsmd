use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use rsmd::{
    i18n::Language,
    server::{AppState, create_router},
};
use std::path::PathBuf;
use std::sync::Arc;
use tower::util::ServiceExt;

#[tokio::test]
async fn single_file_routes_render_expected_content() {
    let state = Arc::new(AppState::SingleFile {
        markdown_content: "# Title".to_string(),
        html_content: "<h1>Title</h1>".to_string(),
        language: Language::English,
        base_dir: PathBuf::from("."),
    });

    let html_app = create_router(state.clone());

    let html_response = html_app
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(html_response.status(), StatusCode::OK);
    let html_body = String::from_utf8(
        axum::body::to_bytes(html_response.into_body(), usize::MAX)
            .await
            .unwrap()
            .to_vec(),
    )
    .unwrap();
    assert!(html_body.contains("<h1>Title</h1>"));
    assert!(html_body.contains("Markdown Viewer"));

    let raw_app = create_router(state);

    let raw_response = raw_app
        .oneshot(Request::builder().uri("/raw").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(raw_response.status(), StatusCode::OK);
    let raw_body = String::from_utf8(
        axum::body::to_bytes(raw_response.into_body(), usize::MAX)
            .await
            .unwrap()
            .to_vec(),
    )
    .unwrap();
    assert!(raw_body.contains("# Title"));
    assert!(raw_body.contains("Raw Markdown"));
}

#[tokio::test]
async fn single_file_api_endpoints_return_empty_or_error() {
    let state = Arc::new(AppState::SingleFile {
        markdown_content: "# Body".to_string(),
        html_content: "<h1>Body</h1>".to_string(),
        language: Language::English,
        base_dir: PathBuf::from("."),
    });

    let files_app = create_router(state.clone());

    let files_response = files_app
        .oneshot(
            Request::builder()
                .uri("/api/files")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(files_response.status(), StatusCode::NOT_FOUND);

    let partial_app = create_router(state);

    let partial_response = partial_app
        .oneshot(
            Request::builder()
                .uri("/api/content/doc.md")
                .header("X-Requested-With", "XMLHttpRequest")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(partial_response.status(), StatusCode::NOT_FOUND);
}
