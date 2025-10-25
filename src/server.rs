use axum::{
    extract::State,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use std::sync::Arc;
use tower_http::trace::TraceLayer;

use crate::html;

#[derive(Clone)]
pub struct AppState {
    pub markdown_content: String,
    pub html_content: String,
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
    Router::new()
        .route("/", get(serve_html))
        .route("/raw", get(serve_raw))
        .with_state(state)
        .layer(TraceLayer::new_for_http())
}

/// Handler for rendering markdown as HTML
async fn serve_html(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    Html(html::render_page(&state.html_content))
}

/// Handler for displaying raw markdown
async fn serve_raw(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    Html(html::render_raw_page(&state.markdown_content))
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
    fn test_app_state_creation() {
        let state = AppState {
            markdown_content: "# Test".to_string(),
            html_content: "<h1>Test</h1>".to_string(),
        };
        assert_eq!(state.markdown_content, "# Test");
        assert_eq!(state.html_content, "<h1>Test</h1>");
    }
}
