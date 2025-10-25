//! HTMX-related functionality for dynamic page updates

/// Generate HTMX script tag
pub fn htmx_script() -> &'static str {
    r#"<script src="https://unpkg.com/htmx.org@1.9.10"></script>"#
}

/// Generate HTMX configuration script
pub fn htmx_config() -> &'static str {
    r#"<script>
    // HTMX configuration
    htmx.config.historyCacheSize = 10;
    htmx.config.refreshOnHistoryMiss = true;

    // Update page title on content load
    document.body.addEventListener('htmx:afterSwap', function(evt) {
        const title = evt.detail.target.querySelector('h1');
        if (title) {
            document.title = title.textContent;
        }
    });
    </script>"#
}

/// Render content-only HTML (for HTMX partial updates)
pub fn render_partial_content(html_content: &str) -> String {
    html_content.to_string()
}

/// Check if request is from HTMX
pub fn is_htmx_request(hx_request_header: Option<&str>) -> bool {
    hx_request_header.is_some_and(|v| v == "true")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_htmx_script_tag() {
        let script = htmx_script();
        assert!(script.contains("htmx.org"));
        assert!(script.contains("<script"));
    }

    #[test]
    fn test_htmx_config() {
        let config = htmx_config();
        assert!(config.contains("htmx.config"));
        assert!(config.contains("historyCacheSize"));
    }

    #[test]
    fn test_render_partial_content() {
        let html = "<h1>Test</h1>";
        let result = render_partial_content(html);
        assert_eq!(result, html);
    }

    #[test]
    fn test_is_htmx_request() {
        assert!(is_htmx_request(Some("true")));
        assert!(!is_htmx_request(Some("false")));
        assert!(!is_htmx_request(None));
    }
}
