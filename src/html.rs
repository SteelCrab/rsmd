use crate::htmx;
use crate::i18n::Language;

/// Generate a complete HTML page with rendered markdown content
pub fn render_page(html_content: &str, language: &Language) -> String {
    let lang_code = match language {
        Language::English => "en",
        Language::Korean => "ko",
    };

    format!(
        r#"<!DOCTYPE html>
<html lang="{}">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{}</title>
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
        lang_code,
        language.text("title_viewer"),
        html_content
    )
}

/// Generate a page to display raw markdown
pub fn render_raw_page(markdown_content: &str, language: &Language) -> String {
    let lang_code = match language {
        Language::English => "en",
        Language::Korean => "ko",
    };

    format!(
        r#"<!DOCTYPE html>
<html lang="{}">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{}</title>
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
        lang_code,
        language.text("title_raw"),
        escape_html(markdown_content)
    )
}

/// Generate a directory listing page with links to markdown files
pub fn render_directory_page(
    files: &[crate::directory::MarkdownFile],
    dir_path: &str,
    language: &Language,
    use_htmx: bool,
) -> String {
    let lang_code = match language {
        Language::English => "en",
        Language::Korean => "ko",
    };

    let file_list = if files.is_empty() {
        format!("<p>{}</p>", language.text("no_files"))
    } else {
        let items: Vec<String> = files
            .iter()
            .map(|f| {
                if use_htmx {
                    format!(
                        r##"<li>
                            <a href="/view/{0}"
                               hx-get="/api/content/{0}"
                               hx-target="#content-area"
                               hx-swap="innerHTML"
                               hx-push-url="true">
                                {0}
                            </a>
                        </li>"##,
                        escape_html(&f.name)
                    )
                } else {
                    format!(
                        r#"<li><a href="/view/{}">{}</a></li>"#,
                        escape_html(&f.name),
                        escape_html(&f.name)
                    )
                }
            })
            .collect();
        format!("<ul class=\"file-list\">\n{}\n</ul>", items.join("\n"))
    };

    format!(
        r#"<!DOCTYPE html>
<html lang="{}">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{} - {}</title>
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
        h1 {{
            margin-top: 0;
            color: #2c3e50;
            border-bottom: 2px solid #3498db;
            padding-bottom: 0.5rem;
        }}
        .file-list {{
            list-style: none;
            padding: 0;
        }}
        .file-list li {{
            padding: 0.75rem;
            margin: 0.5rem 0;
            background: #f8f9fa;
            border-radius: 4px;
            border-left: 3px solid #3498db;
            transition: all 0.2s;
        }}
        .file-list li:hover {{
            background: #e9ecef;
            transform: translateX(5px);
        }}
        .file-list a {{
            color: #2c3e50;
            text-decoration: none;
            font-size: 1.1rem;
            display: block;
        }}
        .file-list a:hover {{
            color: #3498db;
        }}
        .directory-path {{
            color: #7f8c8d;
            font-size: 0.9rem;
            margin-bottom: 1.5rem;
        }}
        {6}
    </style>
    {7}
    {8}
</head>
<body>
    <div class="container">
        <h1>📁 {3}</h1>
        <div class="directory-path">{4}: <code>{5}</code></div>
        {9}
    </div>
</body>
</html>"#,
        lang_code,                        // {0}
        language.text("title_directory"), // {1}
        escape_html(dir_path),            // {2}
        language.text("directory_label"), // {3}
        language.text("directory_path"),  // {4}
        escape_html(dir_path),            // {5}
        if use_htmx {
            // {6}
            r#"#content-area {
            margin-top: 2rem;
            padding: 2rem;
            background: #f8f9fa;
            border-radius: 8px;
            min-height: 200px;
        }
        #content-area.htmx-swapping {
            opacity: 0.5;
            transition: opacity 200ms ease-out;
        }
        .layout-container {
            display: grid;
            grid-template-columns: 1fr 2fr;
            gap: 2rem;
        }
        @media (max-width: 768px) {
            .layout-container {
                grid-template-columns: 1fr;
            }
        }"#
        } else {
            ""
        },
        if use_htmx { htmx::htmx_script() } else { "" }, // {7}
        if use_htmx { htmx::htmx_config() } else { "" }, // {8}
        if use_htmx {
            // {9}
            format!(
                r#"<div class="layout-container">
            <div>
                {}
            </div>
            <div id="content-area">
                <p style="color: #7f8c8d; text-align: center;">← Select a file to preview</p>
            </div>
        </div>"#,
                file_list
            )
        } else {
            file_list.clone()
        }
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
        use crate::directory::MarkdownFile;
        use std::path::PathBuf;

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
    }

    #[test]
    fn test_render_directory_page_with_htmx() {
        use crate::directory::MarkdownFile;
        use std::path::PathBuf;

        let files = vec![MarkdownFile {
            name: "test.md".to_string(),
            path: PathBuf::from("test.md"),
        }];
        let lang = Language::English;
        let result = render_directory_page(&files, "/test", &lang, true);

        assert!(result.contains("test.md"));
        assert!(result.contains("htmx.org"));
        assert!(result.contains("hx-get"));
        assert!(result.contains("hx-target"));
        assert!(result.contains("#content-area"));
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
        use crate::directory::MarkdownFile;
        use std::path::PathBuf;

        let files = vec![MarkdownFile {
            name: "test.md".to_string(),
            path: PathBuf::from("test.md"),
        }];
        let lang = Language::Korean;
        let result = render_directory_page(&files, "/test", &lang, true);

        assert!(result.contains("마크다운"));
        assert!(result.contains("lang=\"ko\""));
    }
}
