use axum::{
    Json, Router, body,
    extract::{Path, Request, State},
    http::{HeaderMap, StatusCode},
    response::{Html, IntoResponse},
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::fs;
use tokio::sync::RwLock;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;

use crate::{
    ajax,
    directory::{self, MarkdownFile},
    html,
    i18n::Language,
    markdown::MarkdownParser,
};

// JSON response structures
#[derive(Serialize, Deserialize)]
pub struct FilesResponse {
    pub files: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct MarkdownResponse {
    pub markdown: String,
}

#[derive(Clone)]
pub enum AppState {
    SingleFile {
        markdown_content: String,
        html_content: String,
        language: Language,
        base_dir: PathBuf,
    },
    Directory {
        dir_path: String,
        files: Arc<RwLock<Vec<MarkdownFile>>>,
        file_cache: Arc<RwLock<HashMap<String, (String, String)>>>, // filename -> (markdown, html)
        language: Language,
        base_dir: PathBuf,
    },
}

pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 3000,
        }
    }
}

impl ServerConfig {
    pub fn new(host: String, port: u16) -> Self {
        Self { host, port }
    }

    pub fn address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

/// Create the application router with all routes
pub fn create_router(state: Arc<AppState>) -> Router {
    let base_dir = match state.as_ref() {
        AppState::SingleFile { base_dir, .. } => base_dir.clone(),
        AppState::Directory { base_dir, .. } => base_dir.clone(),
    };

    match state.as_ref() {
        AppState::SingleFile { .. } => Router::new()
            .route("/", get(serve_html))
            .route("/raw", get(serve_raw))
            .route("/compare", get(serve_comparable_file))
            .nest_service("/static", ServeDir::new(base_dir))
            .with_state(state)
            .layer(TraceLayer::new_for_http()),
        AppState::Directory { .. } => Router::new()
            .route("/", get(serve_directory))
            .route("/dir/{*path}", get(serve_directory_path))
            .route("/view/{*filename}", get(serve_file_html))
            .route("/raw/{*filename}", get(serve_file_raw))
            .route("/api/content/{*filename}", get(serve_partial_content))
            .route("/api/files", get(api_get_files))
            .route("/api/markdown/{*filename}", get(api_get_markdown))
            .route("/api/upload", post(handle_upload))
            .nest_service("/static", ServeDir::new("static"))
            .with_state(state)
            .layer(TraceLayer::new_for_http()),
    }
}

/// Handler for rendering markdown as HTML (single file mode)
async fn serve_html(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    match state.as_ref() {
        AppState::SingleFile {
            html_content,
            language,
            ..
        } => Html(html::render_page(html_content, language)),
        _ => Html("<h1>Error: Invalid mode</h1>".to_string()),
    }
}

/// Handler for displaying raw markdown (single file mode)
async fn serve_raw(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    match state.as_ref() {
        AppState::SingleFile {
            markdown_content,
            language,
            ..
        } => Html(html::render_raw_page(markdown_content, language)),
        _ => Html("<h1>Error: Invalid mode</h1>".to_string()),
    }
}

/// Handler for directory listing
async fn serve_directory(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    match state.as_ref() {
        AppState::Directory {
            dir_path,
            files,
            language,
            ..
        } => {
            let current_files = {
                let guard = files.read().await;
                guard.clone()
            };
            let listing = directory::list_directory_contents(&current_files, "");
            Html(html::render_directory_page(
                &listing, dir_path, language, true,
            ))
        }
        _ => Html("<h1>Error: Invalid mode</h1>".to_string()),
    }
}

/// Handler for directory navigation within nested folders
async fn serve_directory_path(
    State(state): State<Arc<AppState>>,
    Path(path): Path<String>,
) -> impl IntoResponse {
    match state.as_ref() {
        AppState::Directory {
            dir_path,
            files,
            language,
            ..
        } => {
            let requested = path.trim_matches('/');
            let segments: Vec<&str> = requested.split('/').filter(|seg| !seg.is_empty()).collect();
            if segments.contains(&"..") {
                return (
                    StatusCode::BAD_REQUEST,
                    Html(language.text("error_not_found").to_string()),
                )
                    .into_response();
            }

            let normalized = segments.join("/");

            let snapshot = {
                let guard = files.read().await;
                guard.clone()
            };

            let exists = if normalized.is_empty() {
                true
            } else {
                let prefix = format!("{}/", normalized);
                snapshot
                    .iter()
                    .any(|file| file.name == normalized || file.name.starts_with(&prefix))
            };

            if !exists {
                return (
                    StatusCode::NOT_FOUND,
                    Html(html::render_page(
                        &format!("<h1>{}</h1>", language.text("error_not_found")),
                        language,
                    )),
                )
                    .into_response();
            }

            let listing = directory::list_directory_contents(&snapshot, &normalized);

            Html(html::render_directory_page(
                &listing, dir_path, language, true,
            ))
            .into_response()
        }
        _ => Html("<h1>Error: Invalid mode</h1>".to_string()).into_response(),
    }
}

/// Handler for partial content (dynamic AJAX/fetch requests)
async fn serve_partial_content(
    State(state): State<Arc<AppState>>,
    Path(filename): Path<String>,
    headers: HeaderMap,
) -> impl IntoResponse {
    let is_dynamic = ajax::is_dynamic_request(
        headers.get("hx-request").and_then(|v| v.to_str().ok()),
        headers
            .get("x-requested-with")
            .and_then(|v| v.to_str().ok()),
    );

    if !is_dynamic {
        // If not dynamic request, redirect to full page
        return Html("<script>window.location.reload()</script>".to_string());
    }

    match state.as_ref() {
        AppState::Directory {
            files,
            file_cache,
            language,
            ..
        } => {
            let files_snapshot = {
                let guard = files.read().await;
                guard.clone()
            };
            // Check if file exists in the directory
            if !files_snapshot.iter().any(|f| f.name == filename) {
                return Html(format!("<h1>{}</h1>", language.text("error_not_found")));
            }

            // Get from cache or load
            if let Some((_, html_content)) = {
                let cache = file_cache.read().await;
                cache.get(&filename).cloned()
            } {
                return Html(ajax::render_partial_content(&html_content));
            }

            // If not in cache, load it
            if let Some(file) = files_snapshot.iter().find(|f| f.name == filename) {
                match MarkdownParser::from_file(file.path.to_str().unwrap()) {
                    Ok(parser) => {
                        let html_content = parser.to_html();
                        let markdown_content = parser.raw_content().to_string();
                        let mut cache = file_cache.write().await;
                        cache.insert(filename.clone(), (markdown_content, html_content.clone()));
                        Html(ajax::render_partial_content(&html_content))
                    }
                    Err(_) => Html(format!("<h1>{}</h1>", language.text("error_reading_file"))),
                }
            } else {
                Html(format!("<h1>{}</h1>", language.text("error_not_found")))
            }
        }
        _ => Html("<h1>Error: Invalid mode</h1>".to_string()),
    }
}

/// Handler for viewing a specific file in directory mode
async fn serve_file_html(
    State(state): State<Arc<AppState>>,
    Path(filename): Path<String>,
) -> impl IntoResponse {
    match state.as_ref() {
        AppState::Directory {
            files,
            file_cache,
            language,
            ..
        } => {
            let files_snapshot = {
                let guard = files.read().await;
                guard.clone()
            };
            // Check if file exists in the directory
            if !files_snapshot.iter().any(|f| f.name == filename) {
                return Html(format!("<h1>{}</h1>", language.text("error_not_found")));
            }

            // Get from cache or load
            if let Some((_, html_content)) = {
                let cache = file_cache.read().await;
                cache.get(&filename).cloned()
            } {
                return Html(html::render_page(&html_content, language));
            }

            // If not in cache, load it
            if let Some(file) = files_snapshot.iter().find(|f| f.name == filename) {
                match MarkdownParser::from_file(file.path.to_str().unwrap()) {
                    Ok(parser) => {
                        let html_content = parser.to_html();
                        let markdown_content = parser.raw_content().to_string();
                        let mut cache = file_cache.write().await;
                        cache.insert(filename.clone(), (markdown_content, html_content.clone()));
                        Html(html::render_page(&html_content, language))
                    }
                    Err(_) => Html(format!("<h1>{}</h1>", language.text("error_reading_file"))),
                }
            } else {
                Html(format!("<h1>{}</h1>", language.text("error_not_found")))
            }
        }
        _ => Html("<h1>Error: Invalid mode</h1>".to_string()),
    }
}

/// Handler for viewing raw markdown of a specific file in directory mode
async fn serve_file_raw(
    State(state): State<Arc<AppState>>,
    Path(filename): Path<String>,
) -> impl IntoResponse {
    match state.as_ref() {
        AppState::Directory {
            files,
            file_cache,
            language,
            ..
        } => {
            let files_snapshot = {
                let guard = files.read().await;
                guard.clone()
            };
            // Check if file exists in the directory
            if !files_snapshot.iter().any(|f| f.name == filename) {
                return Html(format!("<h1>{}</h1>", language.text("error_not_found")));
            }

            // Get from cache or load
            if let Some((markdown_content, _)) = {
                let cache = file_cache.read().await;
                cache.get(&filename).cloned()
            } {
                return Html(html::render_raw_page(&markdown_content, language));
            }

            // If not in cache, load it
            if let Some(file) = files_snapshot.iter().find(|f| f.name == filename) {
                match MarkdownParser::from_file(file.path.to_str().unwrap()) {
                    Ok(parser) => {
                        let html_content = parser.to_html();
                        let markdown_content = parser.raw_content().to_string();
                        let mut cache = file_cache.write().await;
                        cache.insert(filename.clone(), (markdown_content.clone(), html_content));
                        Html(html::render_raw_page(markdown_content.as_str(), language))
                    }
                    Err(_) => Html(format!("<h1>{}</h1>", language.text("error_reading_file"))),
                }
            } else {
                Html(format!("<h1>{}</h1>", language.text("error_not_found")))
            }
        }
        _ => Html("<h1>Error: Invalid mode</h1>".to_string()),
    }
}

//for serving both rendered and the raw file to the user
async fn serve_comparable_file(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    match state.as_ref() {
        AppState::SingleFile {
            markdown_content,
            html_content,
            language,
            ..
        } => Html(html::render_compare_page(
            html_content,
            markdown_content,
            language,
        )),
        _ => Html("<h1>Fuck You</h1>".to_string()),
    }
}

/// API: Get list of markdown files
async fn api_get_files(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    match state.as_ref() {
        AppState::Directory { files, .. } => {
            let file_names: Vec<String> = {
                let guard = files.read().await;
                guard.iter().map(|f| f.name.clone()).collect()
            };
            Json(FilesResponse { files: file_names })
        }
        _ => Json(FilesResponse { files: vec![] }),
    }
}

/// API: Get markdown content for a specific file
async fn api_get_markdown(
    State(state): State<Arc<AppState>>,
    Path(filename): Path<String>,
) -> impl IntoResponse {
    match state.as_ref() {
        AppState::Directory {
            files, file_cache, ..
        } => {
            let files_snapshot = {
                let guard = files.read().await;
                guard.clone()
            };
            // Check if file exists in the directory
            if !files_snapshot.iter().any(|f| f.name == filename) {
                return Json(MarkdownResponse {
                    markdown: String::from("# Error\n\nFile not found"),
                });
            }

            // Get from cache or load
            if let Some((markdown_content, _)) = {
                let cache = file_cache.read().await;
                cache.get(&filename).cloned()
            } {
                return Json(MarkdownResponse {
                    markdown: markdown_content.clone(),
                });
            }

            // If not in cache, load it
            if let Some(file) = files_snapshot.iter().find(|f| f.name == filename) {
                match MarkdownParser::from_file(file.path.to_str().unwrap()) {
                    Ok(parser) => {
                        let html_content = parser.to_html();
                        let markdown_content = parser.raw_content().to_string();
                        let mut cache = file_cache.write().await;
                        cache.insert(filename.clone(), (markdown_content.clone(), html_content));
                        Json(MarkdownResponse {
                            markdown: markdown_content,
                        })
                    }
                    Err(_) => Json(MarkdownResponse {
                        markdown: String::from("# Error\n\nFailed to read file"),
                    }),
                }
            } else {
                Json(MarkdownResponse {
                    markdown: String::from("# Error\n\nFile not found"),
                })
            }
        }
        _ => Json(MarkdownResponse {
            markdown: String::from("# Error\n\nInvalid mode"),
        }),
    }
}

#[derive(Serialize)]
struct UploadResponse {
    success: bool,
    message: String,
    file: Option<String>,
}

const MAX_UPLOAD_SIZE: usize = 10 * 1024 * 1024;

/// API: Upload a markdown file into the current directory
async fn handle_upload(State(state): State<Arc<AppState>>, request: Request) -> impl IntoResponse {
    match state.as_ref() {
        AppState::Directory {
            dir_path,
            files,
            file_cache,
            language,
            base_dir,
        } => {
            let (parts, body) = request.into_parts();
            let Some(raw_name) = parts
                .headers
                .get("x-file-name")
                .and_then(|v| v.to_str().ok())
                .map(|v| v.trim())
                .filter(|v| !v.is_empty())
            else {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(UploadResponse {
                        success: false,
                        message: language.text("upload_error").to_string(),
                        file: None,
                    }),
                );
            };

            let sanitized = std::path::Path::new(raw_name)
                .file_name()
                .map(|name| name.to_string_lossy().to_string())
                .unwrap_or_default();

            if sanitized.is_empty() {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(UploadResponse {
                        success: false,
                        message: language.text("upload_error").to_string(),
                        file: None,
                    }),
                );
            }

            let lowered = sanitized.to_lowercase();
            if !lowered.ends_with(".md") && !lowered.ends_with(".markdown") {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(UploadResponse {
                        success: false,
                        message: language.text("upload_invalid_type").to_string(),
                        file: None,
                    }),
                );
            }

            let bytes = match body::to_bytes(body, MAX_UPLOAD_SIZE).await {
                Ok(data) => data,
                Err(err) => {
                    tracing::error!(error = %err, "Failed to read upload body");
                    return (
                        StatusCode::BAD_REQUEST,
                        Json(UploadResponse {
                            success: false,
                            message: language.text("upload_error").to_string(),
                            file: None,
                        }),
                    );
                }
            };

            if bytes.is_empty() {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(UploadResponse {
                        success: false,
                        message: language.text("upload_error").to_string(),
                        file: None,
                    }),
                );
            }

            let file_name = sanitized;

            let dir_header_raw = parts
                .headers
                .get("x-directory-path")
                .and_then(|v| v.to_str().ok())
                .map(|v| v.trim().replace('\\', "/"));
            let dir_header = dir_header_raw.as_deref().unwrap_or("");

            let dir_segments: Vec<&str> = dir_header
                .split('/')
                .map(str::trim)
                .filter(|seg| !seg.is_empty())
                .collect();

            if dir_segments
                .iter()
                .any(|seg| *seg == "." || *seg == ".." || seg.contains(".."))
            {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(UploadResponse {
                        success: false,
                        message: language.text("upload_error").to_string(),
                        file: None,
                    }),
                );
            }

            let normalized_dir = dir_segments.join("/");

            let destination = if normalized_dir.is_empty() {
                base_dir.join(&file_name)
            } else {
                base_dir.join(&normalized_dir).join(&file_name)
            };

            let logical_name = if normalized_dir.is_empty() {
                file_name.clone()
            } else {
                format!("{}/{}", normalized_dir, file_name)
            };

            if let Some(parent_dir) = destination.parent()
                && let Err(err) = fs::create_dir_all(parent_dir).await
            {
                tracing::error!(
                    error = %err,
                    ?parent_dir,
                    "Failed to create directory for uploaded markdown file",
                );
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(UploadResponse {
                        success: false,
                        message: language.text("upload_error").to_string(),
                        file: None,
                    }),
                );
            }

            if let Err(err) = fs::write(&destination, bytes.as_ref()).await {
                tracing::error!(
                    error = %err,
                    ?destination,
                    "Failed to write uploaded markdown file",
                );
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(UploadResponse {
                        success: false,
                        message: language.text("upload_error").to_string(),
                        file: None,
                    }),
                );
            }

            {
                let mut guard = files.write().await;
                if let Some(existing) = guard.iter_mut().find(|f| f.name == logical_name) {
                    existing.path = destination.clone();
                } else {
                    guard.push(MarkdownFile {
                        name: logical_name.clone(),
                        path: destination.clone(),
                    });
                }
                guard.sort_by(|a, b| a.name.cmp(&b.name));
            }

            {
                let mut cache = file_cache.write().await;
                cache.remove(&logical_name);
            }

            tracing::info!(
                directory = %dir_path,
                file = %logical_name,
                "Markdown file uploaded",
            );

            (
                StatusCode::OK,
                Json(UploadResponse {
                    success: true,
                    message: language.text("upload_success").to_string(),
                    file: Some(logical_name),
                }),
            )
        }
        _ => (
            StatusCode::BAD_REQUEST,
            Json(UploadResponse {
                success: false,
                message: "Uploads are only available in directory mode".to_string(),
                file: None,
            }),
        ),
    }
}
