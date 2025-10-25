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

        // Fetch content with custom header
        fetch(url, {
            headers: {
                'X-Requested-With': 'XMLHttpRequest'
            }
        })
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

/// Check if request is from dynamic loading (AJAX/fetch)
pub fn is_dynamic_request(hx_request_header: Option<&str>, xhr_header: Option<&str>) -> bool {
    // Check for HTMX header (legacy)
    if hx_request_header.is_some_and(|v| v == "true") {
        return true;
    }
    // Check for X-Requested-With header (vanilla JS fetch)
    xhr_header.is_some_and(|v| v == "XMLHttpRequest")
}
