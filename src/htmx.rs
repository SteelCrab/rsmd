//! Simple dynamic loading functionality without external dependencies

/// Generate inline JavaScript for dynamic content loading (replaces HTMX)
pub fn dynamic_script() -> &'static str {
    r#"<script>
// Simple fetch-based dynamic loading (no external dependencies)
document.addEventListener('DOMContentLoaded', function() {
    const contentArea = document.getElementById('content-area');
    if (!contentArea) return;

    // Handle clicks on file links
    document.addEventListener('click', function(e) {
        const link = e.target.closest('a[data-load]');
        if (!link) return;

        e.preventDefault();
        const url = link.getAttribute('data-load');

        // Loading state
        contentArea.style.opacity = '0.6';

        // Fetch content
        fetch(url)
            .then(r => r.text())
            .then(html => {
                contentArea.innerHTML = html;
                contentArea.style.opacity = '1';

                // Update URL without reload
                history.pushState({}, '', link.href);

                // Update title
                const h1 = contentArea.querySelector('h1');
                if (h1) document.title = h1.textContent;
            })
            .catch(err => {
                contentArea.innerHTML = '<p style="color: #ef4444;">Error loading file</p>';
                contentArea.style.opacity = '1';
            });
    });

    // Handle back/forward buttons
    window.addEventListener('popstate', function() {
        location.reload();
    });
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
    fn test_is_htmx_request() {
        assert!(is_htmx_request(Some("true")));
        assert!(!is_htmx_request(Some("false")));
        assert!(!is_htmx_request(None));
    }
}
