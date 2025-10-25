use axum::{
    Json, Router,
    extract::{Path, State},
    http::HeaderMap,
    response::{Html, IntoResponse},
    routing::get,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;

use crate::{ajax, directory::MarkdownFile, html, i18n::Language, markdown::MarkdownParser};

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
        files: Vec<MarkdownFile>,
        file_cache: Arc<HashMap<String, (String, String)>>, // filename -> (markdown, html)
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
            .nest_service("/static", ServeDir::new(base_dir))
            .with_state(state)
            .layer(TraceLayer::new_for_http()),
        AppState::Directory { .. } => Router::new()
            .route("/", get(serve_directory))
            .route("/view/:filename", get(serve_file_html))
            .route("/raw/:filename", get(serve_file_raw))
            .route("/api/content/:filename", get(serve_partial_content))
            .route("/api/files", get(api_get_files))
            .route("/api/markdown/:filename", get(api_get_markdown))
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
        } => Html(html::render_directory_page(
            files, dir_path, language, false,
        )),
        _ => Html("<h1>Error: Invalid mode</h1>".to_string()),
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
            // Check if file exists in the directory
            if !files.iter().any(|f| f.name == filename) {
                return Html(format!("<h1>{}</h1>", language.text("error_not_found")));
            }

            // Get from cache or load
            if let Some((_, html_content)) = file_cache.get(&filename) {
                return Html(ajax::render_partial_content(html_content));
            }

            // If not in cache, load it
            if let Some(file) = files.iter().find(|f| f.name == filename) {
                match MarkdownParser::from_file(file.path.to_str().unwrap()) {
                    Ok(parser) => Html(ajax::render_partial_content(&parser.to_html())),
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
            // Check if file exists in the directory
            if !files.iter().any(|f| f.name == filename) {
                return Html(format!("<h1>{}</h1>", language.text("error_not_found")));
            }

            // Get from cache or load
            if let Some((_, html_content)) = file_cache.get(&filename) {
                return Html(html::render_page(html_content, language));
            }

            // If not in cache, load it
            if let Some(file) = files.iter().find(|f| f.name == filename) {
                match MarkdownParser::from_file(file.path.to_str().unwrap()) {
                    Ok(parser) => Html(html::render_page(&parser.to_html(), language)),
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
            // Check if file exists in the directory
            if !files.iter().any(|f| f.name == filename) {
                return Html(format!("<h1>{}</h1>", language.text("error_not_found")));
            }

            // Get from cache or load
            if let Some((markdown_content, _)) = file_cache.get(&filename) {
                return Html(html::render_raw_page(markdown_content, language));
            }

            // If not in cache, load it
            if let Some(file) = files.iter().find(|f| f.name == filename) {
                match MarkdownParser::from_file(file.path.to_str().unwrap()) {
                    Ok(parser) => Html(html::render_raw_page(parser.raw_content(), language)),
                    Err(_) => Html(format!("<h1>{}</h1>", language.text("error_reading_file"))),
                }
            } else {
                Html(format!("<h1>{}</h1>", language.text("error_not_found")))
            }
        }
        _ => Html("<h1>Error: Invalid mode</h1>".to_string()),
    }
}

/// API: Get list of markdown files
async fn api_get_files(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    match state.as_ref() {
        AppState::Directory { files, .. } => {
            let file_names: Vec<String> = files.iter().map(|f| f.name.clone()).collect();
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
            // Check if file exists in the directory
            if !files.iter().any(|f| f.name == filename) {
                return Json(MarkdownResponse {
                    markdown: String::from("# Error\n\nFile not found"),
                });
            }

            // Get from cache or load
            if let Some((markdown_content, _)) = file_cache.get(&filename) {
                return Json(MarkdownResponse {
                    markdown: markdown_content.clone(),
                });
            }

            // If not in cache, load it
            if let Some(file) = files.iter().find(|f| f.name == filename) {
                match MarkdownParser::from_file(file.path.to_str().unwrap()) {
                    Ok(parser) => Json(MarkdownResponse {
                        markdown: parser.raw_content().to_string(),
                    }),
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
