use axum::{
    Router,
    extract::{Path, State},
    http::HeaderMap,
    response::{Html, IntoResponse},
    routing::get,
};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;

use crate::{directory::MarkdownFile, html, htmx, i18n::Language, markdown::MarkdownParser};

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
            .nest_service("/static", ServeDir::new(base_dir))
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
        } => Html(html::render_directory_page(files, dir_path, language, true)),
        _ => Html("<h1>Error: Invalid mode</h1>".to_string()),
    }
}

/// Handler for partial content (HTMX requests)
async fn serve_partial_content(
    State(state): State<Arc<AppState>>,
    Path(filename): Path<String>,
    headers: HeaderMap,
) -> impl IntoResponse {
    let is_htmx = htmx::is_htmx_request(headers.get("hx-request").and_then(|v| v.to_str().ok()));

    if !is_htmx {
        // If not HTMX request, redirect to full page
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
                return Html(htmx::render_partial_content(html_content));
            }

            // If not in cache, load it
            if let Some(file) = files.iter().find(|f| f.name == filename) {
                match MarkdownParser::from_file(file.path.to_str().unwrap()) {
                    Ok(parser) => Html(htmx::render_partial_content(&parser.to_html())),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_server_config_default() {
        let config = ServerConfig::default();
        assert_eq!(config.host, "127.0.0.1");
        assert_eq!(config.port, 3000);
        assert_eq!(config.address(), "127.0.0.1:3000");
    }

    #[test]
    fn test_server_config_custom() {
        let config = ServerConfig::new("0.0.0.0".to_string(), 8080);
        assert_eq!(config.host, "0.0.0.0");
        assert_eq!(config.port, 8080);
        assert_eq!(config.address(), "0.0.0.0:8080");
    }

    #[test]
    fn test_app_state_single_file_creation() {
        use std::path::PathBuf;
        let state = AppState::SingleFile {
            markdown_content: "# Test".to_string(),
            html_content: "<h1>Test</h1>".to_string(),
            language: Language::English,
            base_dir: PathBuf::from("/tmp"),
        };
        match state {
            AppState::SingleFile {
                markdown_content,
                html_content,
                language,
                base_dir,
            } => {
                assert_eq!(markdown_content, "# Test");
                assert_eq!(html_content, "<h1>Test</h1>");
                assert_eq!(language, Language::English);
                assert_eq!(base_dir, PathBuf::from("/tmp"));
            }
            _ => panic!("Expected SingleFile state"),
        }
    }

    #[test]
    fn test_app_state_directory_creation() {
        use std::collections::HashMap;
        use std::path::PathBuf;
        let state = AppState::Directory {
            dir_path: "/test".to_string(),
            files: vec![],
            file_cache: Arc::new(HashMap::new()),
            language: Language::Korean,
            base_dir: PathBuf::from("/test"),
        };
        match state {
            AppState::Directory {
                dir_path,
                files,
                language,
                base_dir,
                ..
            } => {
                assert_eq!(dir_path, "/test");
                assert_eq!(files.len(), 0);
                assert_eq!(language, Language::Korean);
                assert_eq!(base_dir, PathBuf::from("/test"));
            }
            _ => panic!("Expected Directory state"),
        }
    }
}
