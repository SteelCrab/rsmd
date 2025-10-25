use rsmd::ajax::{dynamic_script, is_dynamic_request, render_partial_content};

#[test]
fn test_dynamic_script_contains_fetch() {
    let script = dynamic_script();
    assert!(script.contains("fetch("));
    assert!(script.contains("<script"));
    assert!(script.contains("data-load"));
}

#[test]
fn test_dynamic_script_event_listeners() {
    let script = dynamic_script();
    assert!(script.contains("addEventListener"));
    assert!(script.contains("DOMContentLoaded"));
    assert!(script.contains("click"));
}

#[test]
fn test_render_partial_content() {
    let html = "<h1>Test</h1>";
    let result = render_partial_content(html);
    assert_eq!(result, html);
}

#[test]
fn test_is_dynamic_request_htmx() {
    assert!(is_dynamic_request(Some("true"), None));
    assert!(!is_dynamic_request(Some("false"), None));
    assert!(!is_dynamic_request(None, None));
}

#[test]
fn test_is_dynamic_request_xhr() {
    assert!(is_dynamic_request(None, Some("XMLHttpRequest")));
    assert!(!is_dynamic_request(None, Some("other")));
    assert!(!is_dynamic_request(None, None));
}

#[test]
fn test_is_dynamic_request_both() {
    assert!(is_dynamic_request(Some("true"), Some("XMLHttpRequest")));
}
