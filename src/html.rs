use crate::ajax;
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
        * {{ margin: 0; padding: 0; box-sizing: border-box; }}

        body {{
            font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
            line-height: 1.7;
            color: #0f172a;
            background: #ffffff;
            min-height: 100vh;
            padding: 2rem 1rem;
        }}

        .container {{
            max-width: 800px;
            margin: 0 auto;
            background: #fff;
            padding: 3rem;
            border-radius: 16px;
            box-shadow: 0 20px 60px rgba(15, 23, 42, 0.08);
            animation: fadeIn 0.4s ease-out;
        }}

        @keyframes fadeIn {{
            from {{ opacity: 0; transform: translateY(20px); }}
            to {{ opacity: 1; transform: translateY(0); }}
        }}

        h1, h2, h3 {{
            margin: 2rem 0 1rem;
            font-weight: 700;
            line-height: 1.3;
        }}

        h1 {{ font-size: 2.5rem; margin-top: 0; }}
        h2 {{ font-size: 2rem; border-bottom: 2px solid #e2e8f0; padding-bottom: 0.5rem; }}
        h3 {{ font-size: 1.5rem; color: #475569; }}

        p {{ margin: 1rem 0; color: #475569; }}

        a {{
            color: #3b82f6;
            text-decoration: none;
            transition: color 0.2s;
        }}

        a:hover {{ color: #2563eb; }}

        code {{
            background: #f1f5f9;
            color: #db2777;
            padding: 0.2em 0.5em;
            border-radius: 4px;
            font-family: monospace;
            font-size: 0.9em;
        }}

        pre {{
            background: #f1f5f9;
            padding: 1.5rem;
            border-radius: 8px;
            overflow-x: auto;
            margin: 1.5rem 0;
        }}

        pre code {{ background: none; color: #0f172a; padding: 0; }}

        blockquote {{
            border-left: 4px solid #3b82f6;
            margin: 1.5rem 0;
            padding: 1rem 1.5rem;
            background: #f8fafc;
            color: #475569;
            font-style: italic;
        }}

        img {{
            max-width: 100%;
            height: auto;
            border-radius: 8px;
            margin: 1.5rem 0;
        }}

        table {{
            width: 100%;
            margin: 1.5rem 0;
            border-collapse: collapse;
        }}

        th, td {{
            padding: 0.75rem;
            text-align: left;
            border-bottom: 1px solid #e2e8f0;
        }}

        th {{ background: #f8fafc; font-weight: 600; }}
        tr:hover {{ background: #f8fafc; }}

        ul, ol {{ margin: 1rem 0; padding-left: 2rem; color: #475569; }}
        li {{ margin: 0.5rem 0; }}

        hr {{ border: none; height: 1px; background: #e2e8f0; margin: 2rem 0; }}

        @media (max-width: 768px) {{
            body {{ padding: 1rem; }}
            .container {{ padding: 2rem; }}
            h1 {{ font-size: 2rem; }}
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
        :root {{
            --bg-primary: #1e293b;
            --bg-secondary: #0f172a;
            --text-primary: #e2e8f0;
            --text-secondary: #94a3b8;
            --border: #334155;
        }}

        * {{ margin: 0; padding: 0; box-sizing: border-box; }}

        body {{
            font-family: "JetBrains Mono", "Fira Code", "Monaco", "Courier New", monospace;
            background: #ffffff;
            min-height: 100vh;
            padding: 2rem 1rem;
        }}

        pre {{
            max-width: 900px;
            margin: 0 auto;
            background: var(--bg-primary);
            color: var(--text-primary);
            padding: 2.5rem;
            border-radius: 16px;
            box-shadow: 0 20px 60px rgba(0, 0, 0, 0.3);
            overflow-x: auto;
            white-space: pre-wrap;
            word-wrap: break-word;
            line-height: 1.6;
            border: 1px solid var(--border);
            animation: fadeIn 0.5s ease-out;
        }}

        @keyframes fadeIn {{
            from {{ opacity: 0; transform: translateY(20px); }}
            to {{ opacity: 1; transform: translateY(0); }}
        }}

        @media (max-width: 768px) {{
            pre {{ padding: 1.5rem; border-radius: 12px; }}
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
                            <a href="/view/{0}" data-load="/api/content/{0}">
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
        * {{ margin: 0; padding: 0; box-sizing: border-box; }}

        body {{
            font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
            color: #0f172a;
            background: #ffffff;
            min-height: 100vh;
            padding: 2rem 1rem;
        }}

        .container {{
            max-width: 1200px;
            margin: 0 auto;
            background: #fff;
            padding: 2.5rem;
            border-radius: 16px;
            box-shadow: 0 20px 60px rgba(15, 23, 42, 0.08);
            animation: slideIn 0.4s ease-out;
        }}

        @keyframes slideIn {{
            from {{ opacity: 0; transform: translateY(20px); }}
            to {{ opacity: 1; transform: translateY(0); }}
        }}

        .header {{
            margin-bottom: 2rem;
            padding-bottom: 1.5rem;
            border-bottom: 2px solid #e2e8f0;
        }}

        h1 {{
            font-size: 2rem;
            font-weight: 700;
            display: flex;
            align-items: center;
            gap: 0.75rem;
        }}

        .directory-path {{
            color: #94a3b8;
            font-size: 0.875rem;
            margin-top: 0.5rem;
        }}

        .directory-path code {{
            background: #f8fafc;
            padding: 0.25rem 0.5rem;
            border-radius: 4px;
            font-family: monospace;
            color: #475569;
        }}

        .file-list {{
            list-style: none;
            padding: 0;
            display: grid;
            gap: 0.75rem;
        }}

        .file-list li {{
            background: #f8fafc;
            border-radius: 8px;
            transition: all 0.2s;
            border: 1px solid transparent;
        }}

        .file-list li:hover {{
            transform: translateY(-2px);
            box-shadow: 0 4px 12px rgba(15, 23, 42, 0.1);
            border-color: #3b82f6;
        }}

        .file-list a {{
            color: #0f172a;
            text-decoration: none;
            padding: 1rem 1.25rem;
            display: flex;
            align-items: center;
            gap: 0.75rem;
            font-weight: 500;
        }}

        .file-list a::before {{
            content: "📄";
            font-size: 1.25rem;
        }}

        .file-list a::after {{
            content: "→";
            margin-left: auto;
            opacity: 0;
            transform: translateX(-10px);
            transition: all 0.2s;
            color: #3b82f6;
        }}

        .file-list li:hover a::after {{
            opacity: 1;
            transform: translateX(0);
        }}

        {6}
    </style>
    {7}
    {8}
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>📁 {3}</h1>
            <div class="directory-path">{4}: <code>{5}</code></div>
        </div>
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
            r#"
        .layout-container {{
            display: grid;
            grid-template-columns: 350px 1fr;
            gap: 1.5rem;
        }}

        #content-area {{
            background: #f8fafc;
            border-radius: 8px;
            padding: 2rem;
            min-height: 400px;
            transition: opacity 0.2s;
        }}

        #content-area:empty::before {{
            content: "← Select a file";
            display: flex;
            align-items: center;
            justify-content: center;
            height: 100%;
            color: #94a3b8;
            font-style: italic;
        }}

        @media (max-width: 968px) {{
            .layout-container {{ grid-template-columns: 1fr; }}
        }}
        "#
        } else {
            ""
        },
        if use_htmx { ajax::dynamic_script() } else { "" }, // {7}
        "",                                                 // {8}
        if use_htmx {
            // {9}
            format!(
                r#"<div class="layout-container">
            <div>
                {}
            </div>
            <div id="content-area"></div>
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
