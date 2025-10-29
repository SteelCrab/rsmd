use rsmd::directory::MarkdownFile;
use rsmd::html::{escape_html, render_directory_page, render_page, render_raw_page};
use rsmd::i18n::Language;
use std::path::PathBuf;

#[test]
fn test_escape_html() {
    assert_eq!(escape_html("<div>"), "&lt;div&gt;");
    assert_eq!(escape_html("a & b"), "a &amp; b");
    assert_eq!(escape_html("\"quote\""), "&quot;quote&quot;");
    assert_eq!(escape_html("rock 'n' roll"), "rock &#x27;n&#x27; roll");
}

#[test]
fn test_render_page_contains_content() {
    let lang = Language::English;
    let result = render_page("<h1>Test</h1>", &lang);
    assert!(result.contains("<h1>Test</h1>"));
    assert!(result.contains("<!DOCTYPE html>"));
    assert!(result.contains("Markdown Viewer"));
}

#[test]
fn test_render_raw_page_escapes_html() {
    let lang = Language::English;
    let result = render_raw_page("<script>alert('xss')</script>", &lang);
    assert!(result.contains("&lt;script&gt;"));
    assert!(!result.contains("<script>"));
}

#[test]
fn test_render_directory_page_without_htmx() {
    let files = vec![MarkdownFile {
        name: "test.md".to_string(),
        path: PathBuf::from("test.md"),
    }];
    let lang = Language::English;
    let result = render_directory_page(&files, "/test", &lang, false);

    assert!(result.contains("test.md"));
    assert!(result.contains("/view/test.md"));
    assert!(!result.contains("htmx"));
    assert!(!result.contains("hx-get"));
    assert!(!result.contains("upload-panel"));
}

#[test]
fn test_render_directory_page_with_dynamic_loading() {
    let files = vec![MarkdownFile {
        name: "test.md".to_string(),
        path: PathBuf::from("test.md"),
    }];
    let lang = Language::English;
    let result = render_directory_page(&files, "/test", &lang, true);

    assert!(result.contains("test.md"));
    assert!(result.contains("data-load"));
    assert!(result.contains("#content-area"));
    assert!(result.contains("document.addEventListener"));
    assert!(result.contains("upload-panel"));
    assert!(result.contains("upload-browse"));
}

#[test]
fn test_render_directory_page_empty() {
    let files = vec![];
    let lang = Language::English;
    let result = render_directory_page(&files, "/test", &lang, true);

    assert!(result.contains("No markdown files found"));
}

#[test]
fn test_render_directory_page_korean() {
    let files = vec![MarkdownFile {
        name: "test.md".to_string(),
        path: PathBuf::from("test.md"),
    }];
    let lang = Language::Korean;
    let result = render_directory_page(&files, "/test", &lang, true);

    assert!(result.contains("마크다운"));
    assert!(result.contains("lang=\"ko\""));
}
