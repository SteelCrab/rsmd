use rsmd::{i18n::Language, server::AppState, server::ServerConfig};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

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
