/// Generate a complete HTML page with rendered markdown content
pub fn render_page(html_content: &str) -> String {
    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Markdown Viewer</title>
    <style>
        body {{
            max-width: 900px;
            margin: 0 auto;
            padding: 2rem;
            font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif;
            line-height: 1.6;
            color: #333;
            background-color: #f5f5f5;
        }}
        .container {{
            background: white;
            padding: 3rem;
            border-radius: 8px;
            box-shadow: 0 2px 4px rgba(0,0,0,0.1);
        }}
        h1, h2, h3, h4, h5, h6 {{
            margin-top: 1.5em;
            margin-bottom: 0.5em;
            font-weight: 600;
        }}
        code {{
            background: #f4f4f4;
            padding: 0.2em 0.4em;
            border-radius: 3px;
            font-family: "Monaco", "Courier New", monospace;
            font-size: 0.9em;
        }}
        pre {{
            background: #f4f4f4;
            padding: 1rem;
            border-radius: 5px;
            overflow-x: auto;
        }}
        pre code {{
            background: none;
            padding: 0;
        }}
        blockquote {{
            border-left: 4px solid #ddd;
            margin-left: 0;
            padding-left: 1rem;
            color: #666;
        }}
        a {{
            color: #0066cc;
            text-decoration: none;
        }}
        a:hover {{
            text-decoration: underline;
        }}
        img {{
            max-width: 100%;
            height: auto;
        }}
        table {{
            border-collapse: collapse;
            width: 100%;
            margin: 1em 0;
        }}
        th, td {{
            border: 1px solid #ddd;
            padding: 0.5rem;
            text-align: left;
        }}
        th {{
            background-color: #f4f4f4;
            font-weight: 600;
        }}
    </style>
</head>
<body>
    <div class="container">
        {}
    </div>
</body>
</html>"#,
        html_content
    )
}

/// Generate a page to display raw markdown
pub fn render_raw_page(markdown_content: &str) -> String {
    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Raw Markdown</title>
    <style>
        body {{
            max-width: 900px;
            margin: 0 auto;
            padding: 2rem;
            font-family: "Monaco", "Courier New", monospace;
            background-color: #f5f5f5;
        }}
        pre {{
            background: white;
            padding: 2rem;
            border-radius: 8px;
            box-shadow: 0 2px 4px rgba(0,0,0,0.1);
            overflow-x: auto;
            white-space: pre-wrap;
            word-wrap: break-word;
        }}
    </style>
</head>
<body>
    <pre>{}</pre>
</body>
</html>"#,
        escape_html(markdown_content)
    )
}

/// Escape HTML special characters
pub fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#x27;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_escape_html() {
        assert_eq!(escape_html("<div>"), "&lt;div&gt;");
        assert_eq!(escape_html("a & b"), "a &amp; b");
        assert_eq!(escape_html("\"quote\""), "&quot;quote&quot;");
    }

    #[test]
    fn test_render_page_contains_content() {
        let result = render_page("<h1>Test</h1>");
        assert!(result.contains("<h1>Test</h1>"));
        assert!(result.contains("<!DOCTYPE html>"));
        assert!(result.contains("Markdown Viewer"));
    }

    #[test]
    fn test_render_raw_page_escapes_html() {
        let result = render_raw_page("<script>alert('xss')</script>");
        assert!(result.contains("&lt;script&gt;"));
        assert!(!result.contains("<script>"));
    }
}
