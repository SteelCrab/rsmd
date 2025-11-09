use crate::ajax;
use crate::directory::DirectoryListing;
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
            font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", sans-serif;
            line-height: 1.75;
            color: #1a1a1a;
            background: #fafafa;
            min-height: 100vh;
            padding: 2rem 1rem;
            -webkit-font-smoothing: antialiased;
        }}

        .container {{
            max-width: 800px;
            margin: 0 auto;
            background: #ffffff;
            padding: 4rem 3.5rem;
            border-radius: 12px;
            box-shadow: 0 1px 3px rgba(0, 0, 0, 0.05);
            animation: fadeIn 0.3s ease-out;
        }}

        @keyframes fadeIn {{
            from {{ opacity: 0; transform: translateY(10px); }}
            to {{ opacity: 1; transform: translateY(0); }}
        }}

        h1, h2, h3, h4, h5, h6 {{
            font-weight: 600;
            line-height: 1.4;
            color: #1a1a1a;
            margin: 2.5rem 0 1rem;
        }}

        h1 {{ font-size: 2.5rem; margin-top: 0; }}
        h2 {{ font-size: 2rem; margin-top: 3rem; }}
        h3 {{ font-size: 1.5rem; }}
        h4 {{ font-size: 1.25rem; }}
        h5 {{ font-size: 1.125rem; }}
        h6 {{ font-size: 1rem; }}

        p {{
            margin: 1.25rem 0;
            color: #404040;
            font-size: 1.0625rem;
        }}

        a {{
            color: #0066cc;
            text-decoration: none;
            transition: color 0.15s;
            border-bottom: 1px solid transparent;
        }}

        a:hover {{
            color: #0052a3;
            border-bottom-color: #0052a3;
        }}

        code {{
            background: #f5f5f5;
            color: #e01e5a;
            padding: 0.2em 0.4em;
            border-radius: 3px;
            font-family: "SF Mono", Monaco, "Cascadia Code", "Roboto Mono", Consolas, monospace;
            font-size: 0.875em;
        }}

        pre {{
            background: #f8f8f8;
            padding: 1.5rem;
            border-radius: 6px;
            overflow-x: auto;
            margin: 2rem 0;
            border: 1px solid #e8e8e8;
        }}

        pre code {{
            background: none;
            color: #1a1a1a;
            padding: 0;
            font-size: 0.9375rem;
        }}

        blockquote {{
            border-left: 3px solid #e0e0e0;
            margin: 2rem 0;
            padding: 0.5rem 1.5rem;
            color: #606060;
            font-style: normal;
        }}

        blockquote p {{
            color: #606060;
        }}

        img {{
            max-width: 100%;
            height: auto;
            border-radius: 6px;
            margin: 2rem 0;
        }}

        table {{
            width: 100%;
            margin: 2rem 0;
            border-collapse: collapse;
            font-size: 0.9375rem;
        }}

        th, td {{
            padding: 0.75rem 1rem;
            text-align: left;
            border-bottom: 1px solid #e8e8e8;
        }}

        th {{
            background: #fafafa;
            font-weight: 600;
            color: #1a1a1a;
        }}

        tr:last-child td {{
            border-bottom: none;
        }}

        ul, ol {{
            margin: 1.25rem 0;
            padding-left: 2rem;
            color: #404040;
        }}

        li {{
            margin: 0.5rem 0;
            line-height: 1.75;
        }}

        hr {{
            border: none;
            height: 1px;
            background: #e8e8e8;
            margin: 3rem 0;
        }}

        @media (max-width: 768px) {{
            body {{ padding: 1rem; }}
            .container {{
                padding: 2.5rem 2rem;
                border-radius: 8px;
            }}
            h1 {{ font-size: 2rem; }}
            h2 {{ font-size: 1.75rem; }}
        }}
    </style>
    <script>
    document.addEventListener("keydown", function(event) {{
      if ((event.ctrlKey || event.metaKey) && event.key === "\\") {{
        event.preventDefault();
        window.open("/compare", "_self");
      }}
    }});
    </script>
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
        * {{ margin: 0; padding: 0; box-sizing: border-box; }}

        body {{
            font-family: "SF Mono", Monaco, "Cascadia Code", "Roboto Mono", Consolas, monospace;
            background: #fafafa;
            min-height: 100vh;
            padding: 2rem 1rem;
            -webkit-font-smoothing: antialiased;
        }}

        pre {{
            max-width: 900px;
            margin: 0 auto;
            background: #ffffff;
            color: #1a1a1a;
            padding: 3rem;
            border-radius: 12px;
            box-shadow: 0 1px 3px rgba(0, 0, 0, 0.05);
            overflow-x: auto;
            white-space: pre-wrap;
            word-wrap: break-word;
            line-height: 1.7;
            border: 1px solid #e8e8e8;
            animation: fadeIn 0.3s ease-out;
            font-size: 0.9375rem;
        }}

        @keyframes fadeIn {{
            from {{ opacity: 0; transform: translateY(10px); }}
            to {{ opacity: 1; transform: translateY(0); }}
        }}

        @media (max-width: 768px) {{
            body {{ padding: 1rem; }}
            pre {{
                padding: 2rem;
                border-radius: 8px;
                font-size: 0.875rem;
            }}
        }}
    </style>
    <script>
    document.addEventListener("keydown", function(event) {{
      if ((event.ctrlKey || event.metaKey) && event.key === "\\") {{
        event.preventDefault();
        window.open("/compare", "_self");
      }}
    }});
    </script>
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

/// Generate a page where both raw and markdown content are shown side by side
pub fn render_compare_page(
    html_content: &str,
    markdown_content: &str,
    language: &Language,
) -> String {
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
            font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", sans-serif;
            background: #fafafa;
            min-height: 100vh;
            padding: 1rem;
            -webkit-font-smoothing: antialiased;
        }}

        .compare-container {{
            max-width: 1400px;
            margin: 0 auto;
            display: flex;
            gap: 2rem;
            height: calc(100vh - 2rem);
        }}

        @media (min-width: 1200px) {{
            .compare-container {{
                gap: 2.5rem;
            }}
        }}

        .compare-panel {{
            flex: 1;
            display: flex;
            flex-direction: column;
            background: #ffffff;
            border-radius: 16px;
            box-shadow: 0 4px 12px rgba(0, 0, 0, 0.08);
            border: 1px solid #e8e8e8;
            overflow: hidden;
            transition: box-shadow 0.2s ease, transform 0.2s ease;
        }}

        .compare-panel:hover {{
            box-shadow: 0 6px 20px rgba(0, 0, 0, 0.12);
            transform: translateY(-2px);
        }}

        .panel-header {{
            padding: 1rem 1.5rem;
            background: #f8f8f8;
            border-bottom: 1px solid #e8e8e8;
            font-weight: 600;
            color: #1a1a1a;
            font-size: 0.875rem;
            display: flex;
            align-items: center;
            gap: 0.5rem;
        }}

        .panel-content {{
            flex: 1;
            overflow-y: auto;
            padding: 0;
        }}

        .rendered-content {{
            padding: 4rem 3.5rem;
            line-height: 1.75;
            color: #1a1a1a;
        }}

        .raw-content {{
            padding: 2rem;
            font-family: "SF Mono", Monaco, "Cascadia Code", "Roboto Mono", Consolas, monospace;
            font-size: 0.875rem;
            line-height: 1.7;
            color: #1a1a1a;
            white-space: pre-wrap;
            word-wrap: break-word;
            background: #fafafa;
            height: 100%;
        }}

        /* Rendered content styles */
        .rendered-content h1, .rendered-content h2, .rendered-content h3, 
        .rendered-content h4, .rendered-content h5, .rendered-content h6 {{
            font-weight: 600;
            line-height: 1.4;
            color: #1a1a1a;
            margin: 2.5rem 0 1rem;
        }}

        .rendered-content h1 {{ font-size: 2.5rem; margin-top: 0; }}
        .rendered-content h2 {{ font-size: 2rem; margin-top: 3rem; }}
        .rendered-content h3 {{ font-size: 1.5rem; }}
        .rendered-content h4 {{ font-size: 1.25rem; }}
        .rendered-content h5 {{ font-size: 1.125rem; }}
        .rendered-content h6 {{ font-size: 1rem; }}

        .rendered-content p {{
            margin: 1.25rem 0;
            color: #404040;
            font-size: 1.0625rem;
        }}

        .rendered-content a {{
            color: #0066cc;
            text-decoration: none;
            transition: color 0.15s;
            border-bottom: 1px solid transparent;
        }}

        .rendered-content a:hover {{
            color: #0052a3;
            border-bottom-color: #0052a3;
        }}

        .rendered-content code {{
            background: #f5f5f5;
            color: #e01e5a;
            padding: 0.2em 0.4em;
            border-radius: 3px;
            font-family: "SF Mono", Monaco, "Cascadia Code", "Roboto Mono", Consolas, monospace;
            font-size: 0.875em;
        }}

        .rendered-content pre {{
            background: #f8f8f8;
            padding: 1.5rem;
            border-radius: 6px;
            overflow-x: auto;
            margin: 2rem 0;
            border: 1px solid #e8e8e8;
        }}

        .rendered-content pre code {{
            background: none;
            color: #1a1a1a;
            padding: 0;
            font-size: 0.9375rem;
        }}

        .rendered-content blockquote {{
            border-left: 3px solid #e0e0e0;
            margin: 2rem 0;
            padding: 0.5rem 1.5rem;
            color: #606060;
            font-style: normal;
        }}

        .rendered-content blockquote p {{
            color: #606060;
        }}

        .rendered-content img {{
            max-width: 100%;
            height: auto;
            border-radius: 6px;
            margin: 2rem 0;
        }}

        .rendered-content table {{
            width: 100%;
            margin: 2rem 0;
            border-collapse: collapse;
            font-size: 0.9375rem;
        }}

        .rendered-content th, .rendered-content td {{
            padding: 0.75rem 1rem;
            text-align: left;
            border-bottom: 1px solid #e8e8e8;
        }}

        .rendered-content th {{
            background: #fafafa;
            font-weight: 600;
            color: #1a1a1a;
        }}

        .rendered-content tr:last-child td {{
            border-bottom: none;
        }}

        .rendered-content ul, .rendered-content ol {{
            margin: 1.25rem 0;
            padding-left: 2rem;
            color: #404040;
        }}

        .rendered-content li {{
            margin: 0.5rem 0;
            line-height: 1.75;
        }}

        .rendered-content hr {{
            border: none;
            height: 1px;
            background: #e8e8e8;
            margin: 3rem 0;
        }}

        @media (max-width: 1024px) {{
            .compare-container {{
                flex-direction: column;
                height: auto;
                gap: 1.5rem;
            }}

            .compare-panel {{
                min-height: 50vh;
            }}

            .rendered-content {{
                padding: 2.5rem 2rem;
            }}

            .raw-content {{
                padding: 1.5rem;
            }}
        }}

        @media (max-width: 768px) {{
            body {{ padding: 0.5rem; }}
            
            .compare-container {{
                gap: 1rem;
            }}

            .rendered-content {{
                padding: 2rem 1.5rem;
            }}

            .rendered-content h1 {{ font-size: 2rem; }}
            .rendered-content h2 {{ font-size: 1.75rem; }}

            .raw-content {{
                padding: 1rem;
                font-size: 0.8125rem;
            }}

            .panel-header {{
                padding: 0.75rem 1rem;
                font-size: 0.8125rem;
            }}
        }}
    </style>
    <script>
    document.addEventListener("keydown", function(event) {{
      if ((event.ctrlKey || event.metaKey) && event.key === "\\") {{
        event.preventDefault();
        window.history.back();
      }}
    }});
    </script>
</head>
<body>
    <div class="compare-container">
        <div class="compare-panel">
            <div class="panel-header">
                <span>📄</span>
                <span>{}</span>
            </div>
            <div class="panel-content">
                <div class="rendered-content">
                    {}
                </div>
            </div>
        </div>
        <div class="compare-panel">
            <div class="panel-header">
                <span>🔤</span>
                <span>{}</span>
            </div>
            <div class="panel-content">
                <div class="raw-content">{}</div>
            </div>
        </div>
    </div>
</body>
</html>"#,
        lang_code,
        language.text("title_compare"),
        language.text("rendered_view"),
        html_content,
        language.text("raw_view"),
        escape_html(markdown_content)
    )
}

/// Generate a directory listing page with a navigable folder structure
pub fn render_directory_page(
    listing: &DirectoryListing,
    dir_path: &str,
    language: &Language,
    use_htmx: bool,
) -> String {
    fn encode_segment(segment: &str) -> String {
        segment
            .bytes()
            .map(|byte| match byte {
                b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'.' | b'_' | b'~' => {
                    (byte as char).to_string()
                }
                _ => format!("%{:02X}", byte),
            })
            .collect()
    }

    fn encode_path(path: &str) -> String {
        path.split('/')
            .filter(|segment| !segment.is_empty())
            .map(encode_segment)
            .collect::<Vec<_>>()
            .join("/")
    }

    let lang_code = match language {
        Language::English => "en",
        Language::Korean => "ko",
    };

    let escaped_dir_path = escape_html(dir_path);
    let directory_title = language.text("title_directory");
    let directory_label = language.text("directory_label");
    let folders_label = language.text("folders_label");
    let files_heading = language.text("files_heading");
    let directory_path_label = language.text("directory_path");
    let empty_state_text = escape_html(language.text("no_files"));
    let breadcrumb_root = language.text("breadcrumb_root");
    let back_to_parent = language.text("back_to_parent");

    let has_directories = !listing.directories.is_empty();
    let has_files = !listing.files.is_empty();

    let mut breadcrumb_segments = Vec::new();
    breadcrumb_segments.push(("".to_string(), breadcrumb_root.to_string()));

    if !listing.current_path.is_empty() {
        let mut accumulated = String::new();
        for segment in listing.current_path.split('/') {
            if !accumulated.is_empty() {
                accumulated.push('/');
            }
            accumulated.push_str(segment);
            breadcrumb_segments.push((accumulated.clone(), segment.to_string()));
        }
    }

    let breadcrumbs = breadcrumb_segments
        .iter()
        .enumerate()
        .map(|(idx, (path, label))| {
            let display = escape_html(label);
            if idx == breadcrumb_segments.len() - 1 {
                format!(r#"<span class="breadcrumb-current">{display}</span>"#)
            } else if path.is_empty() {
                format!(r#"<a href="/">{display}</a>"#)
            } else {
                format!(
                    r#"<a href="/dir/{href}">{display}</a>"#,
                    href = encode_path(path),
                    display = display
                )
            }
        })
        .collect::<Vec<_>>()
        .join(r#"<span class="breadcrumb-separator">/</span>"#);

    let folder_items: Vec<String> = listing
        .directories
        .iter()
        .map(|entry| {
            let safe_name = escape_html(&entry.name);
            let safe_path = escape_html(&entry.path);
            let display_path = if entry.path.is_empty() {
                "/".to_string()
            } else {
                format!("/{safe}", safe = safe_path)
            };
            let href_value = if entry.path.is_empty() {
                "/".to_string()
            } else {
                format!("/dir/{}", encode_path(&entry.path))
            };
            let safe_href = escape_html(&href_value);
            let data_path = escape_html(&entry.path);
            format!(
                r#"<a class="folder-card" href="{href}" data-path="{data_path}">
                    <span class="folder-card__icon">📁</span>
                    <span class="folder-card__meta">
                        <span class="folder-card__name">{name}</span>
                        <span class="folder-card__path">{path}</span>
                    </span>
                    <span class="folder-card__arrow">→</span>
                </a>"#,
                href = safe_href,
                name = safe_name,
                path = display_path,
                data_path = data_path
            )
        })
        .collect();

    let file_items: Vec<String> = listing
        .files
        .iter()
        .map(|file| {
            let display_name = if listing.current_path.is_empty() {
                file.name.as_str()
            } else {
                file.name
                    .strip_prefix(&(listing.current_path.clone() + "/"))
                    .unwrap_or(&file.name)
            };
            let short_name = display_name.rsplit('/').next().unwrap_or(display_name);
            let safe_short = escape_html(short_name);
            let safe_path = escape_html(&file.name);
            let href_value = format!("/view/{}", encode_path(&file.name));
            let safe_href = escape_html(&href_value);
            format!(
                r#"<a class="file-entry" href="{href}">
                    <span class="file-entry__icon">📄</span>
                    <span class="file-entry__text">
                        <span class="file-entry__name">{name}</span>
                        <span class="file-entry__path">/{path}</span>
                    </span>
                    <span class="file-entry__arrow">→</span>
                </a>"#,
                href = safe_href,
                name = safe_short,
                path = safe_path
            )
        })
        .collect();

    let parent_link = listing.parent.as_ref().map(|parent| {
        let href = if parent.is_empty() {
            "/".to_string()
        } else {
            format!("/dir/{}", encode_path(parent))
        };
        format!(
            r#"<a class="back-link" href="{href}">← {label}</a>"#,
            label = escape_html(back_to_parent)
        )
    });

    let files_markup = file_items.join("\n");

    let folders_section = if has_directories {
        format!(
            r#"<div class="folder-grid">
            {items}
        </div>"#,
            items = folder_items.join("\n")
        )
    } else {
        r#"<div class="folder-empty">—</div>"#.to_string()
    };

    let files_section = if has_files {
        format!(
            r#"<div class="file-list file-entries">
            {items}
        </div>"#,
            items = files_markup
        )
    } else {
        r#"<div class="file-list file-entries"></div>"#.to_string()
    };

    let current_path_attr = escape_html(&listing.current_path);
    let current_path_label = if listing.current_path.is_empty() {
        "/".to_string()
    } else {
        format!("/{path}", path = listing.current_path)
    };
    let current_path_label = escape_html(&current_path_label);

    let file_list_markup = if use_htmx {
        let upload_panel = format!(
            r#"<div class="upload-card" id="upload-area" data-success="{success}" data-error="{error}" data-invalid="{invalid}" data-uploading="{uploading}" data-current-path="{current_path}">
    <div class="upload-card__icon">📤</div>
    <div class="upload-card__content">
        <h3 class="upload-card__title">{title}</h3>
        <p class="upload-card__description">{instructions}</p>
        <div class="upload-card__actions">
            <button type="button" id="upload-browse" class="upload-card__button">
                <span class="button-icon">📁</span>
                <span>{browse}</span>
            </button>
            <input type="file" id="file-input" accept=".md,.markdown" hidden>
        </div>
        <div class="upload-status" id="upload-status"></div>
    </div>
</div>"#,
            success = escape_html(language.text("upload_success")),
            error = escape_html(language.text("upload_error")),
            invalid = escape_html(language.text("upload_invalid_type")),
            uploading = escape_html(language.text("upload_uploading")),
            title = escape_html(language.text("upload_title")),
            instructions = escape_html(language.text("upload_instructions")),
            browse = escape_html(language.text("upload_browse")),
            current_path = escape_html(&listing.current_path),
        );

        let empty_class = if has_directories || has_files {
            " hidden"
        } else {
            ""
        };

        format!(
            r#"<div class="directory-body" data-current-path="{current_path_attr}">
    {upload_panel}
    <div class="directory-navigation">
        <div class="directory-head">
            <nav class="breadcrumbs">{breadcrumbs}</nav>
            {parent_link}
        </div>
        <div class="folder-section">
            <h2>{folders_label}</h2>
            {folders_section}
        </div>
    </div>
    <div class="file-browser">
        <div class="empty-state{empty_class}" id="empty-state">{empty_text}</div>
        <div class="section-head">
            <h2>{files_heading}</h2>
            <code class="section-path">{current_path_label}</code>
        </div>
        {files_section}
    </div>
</div>"#,
            upload_panel = upload_panel,
            empty_class = empty_class,
            empty_text = empty_state_text,
            breadcrumbs = breadcrumbs,
            parent_link = parent_link.unwrap_or_default(),
            folders_label = escape_html(folders_label),
            folders_section = folders_section,
            files_heading = escape_html(files_heading),
            files_section = files_section,
            current_path_label = current_path_label,
            current_path_attr = current_path_attr,
        )
    } else if has_files {
        format!(
            r#"<div class="file-browser file-browser--static">
    <div class="file-list file-entries">
        {items}
    </div>
</div>"#,
            items = files_markup
        )
    } else {
        format!(
            r#"<div class="file-browser file-browser--static">
    <div class="empty-state">{empty_text}</div>
</div>"#,
            empty_text = empty_state_text
        )
    };

    let layout_styles = if use_htmx {
        r#"
        .directory-body {
            display: flex;
            flex-direction: column;
            gap: 1.5rem;
        }

        .directory-navigation {
            background: #fafafa;
            border: 1px solid #e8e8e8;
            border-radius: 8px;
            padding: 1.5rem;
            display: flex;
            flex-direction: column;
            gap: 1.25rem;
        }

        .directory-head {
            display: flex;
            flex-direction: column;
            gap: 0.75rem;
        }

        .breadcrumbs {
            display: flex;
            flex-wrap: wrap;
            gap: 0.5rem;
            font-size: 0.875rem;
            color: #606060;
        }

        .breadcrumbs a {
            color: #0066cc;
            text-decoration: none;
            transition: color 0.15s;
        }

        .breadcrumbs a:hover {
            color: #0052a3;
        }

        .breadcrumb-current {
            color: #1a1a1a;
            font-weight: 600;
        }

        .breadcrumb-separator {
            color: #d0d0d0;
        }

        .back-link {
            font-size: 0.875rem;
            color: #606060;
            text-decoration: none;
            transition: color 0.15s;
        }

        .back-link:hover {
            color: #1a1a1a;
        }

        .folder-section h2,
        .file-browser h2 {
            font-size: 1rem;
            font-weight: 600;
            color: #1a1a1a;
        }

        .section-head {
            display: flex;
            align-items: center;
            justify-content: space-between;
            gap: 0.75rem;
        }

        .section-path {
            font-family: "SF Mono", Monaco, monospace;
            font-size: 0.75rem;
            background: #f5f5f5;
            color: #606060;
            padding: 0.25rem 0.65rem;
            border-radius: 4px;
        }

        .file-browser {
            display: flex;
            flex-direction: column;
            gap: 1rem;
            background: #ffffff;
            border: 1px solid #e8e8e8;
            border-radius: 8px;
            padding: 1.5rem;
        }

        .file-browser--static {
            background: transparent;
            border: none;
            padding: 0;
        }

        .upload-card {
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            padding: 2rem;
            border-radius: 20px;
            box-shadow: 0 10px 30px rgba(102, 126, 234, 0.3);
            transition: transform 0.3s, box-shadow 0.3s;
            cursor: pointer;
            display: flex;
            gap: 1.5rem;
            align-items: flex-start;
            position: relative;
            overflow: hidden;
        }

        .upload-card::before {
            content: '';
            position: absolute;
            top: 0;
            left: 0;
            right: 0;
            bottom: 0;
            background: linear-gradient(135deg, rgba(255, 255, 255, 0.1) 0%, rgba(255, 255, 255, 0) 100%);
            pointer-events: none;
        }

        .upload-card:hover,
        .upload-card.dragover {
            transform: translateY(-4px);
            box-shadow: 0 15px 40px rgba(102, 126, 234, 0.4);
        }

        .upload-card__icon {
            font-size: 3rem;
            flex-shrink: 0;
            filter: drop-shadow(0 4px 6px rgba(0, 0, 0, 0.1));
        }

        .upload-card__content {
            flex: 1;
            position: relative;
            z-index: 1;
        }

        .upload-card__title {
            color: #ffffff;
            font-weight: 700;
            font-size: 1.4rem;
            margin: 0 0 0.5rem 0;
            text-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
        }

        .upload-card__description {
            color: rgba(255, 255, 255, 0.9);
            margin: 0 0 1.25rem 0;
            line-height: 1.5;
            font-size: 0.95rem;
        }

        .upload-card__actions {
            display: flex;
            gap: 0.75rem;
            align-items: center;
        }

        .upload-card__button {
            background: rgba(255, 255, 255, 0.95);
            color: #667eea;
            border: none;
            padding: 0.75rem 1.5rem;
            border-radius: 12px;
            font-weight: 600;
            cursor: pointer;
            transition: all 0.2s;
            display: flex;
            align-items: center;
            gap: 0.5rem;
            box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
        }

        .upload-card__button:hover {
            background: #ffffff;
            transform: translateY(-2px);
            box-shadow: 0 6px 16px rgba(0, 0, 0, 0.15);
        }

        .upload-card__button .button-icon {
            font-size: 1.1rem;
        }

        .upload-status {
            margin-top: 1rem;
            min-height: 1.25rem;
            font-size: 0.9rem;
            padding: 0.5rem 0.75rem;
            border-radius: 8px;
            font-weight: 500;
        }

        .upload-status.success {
            background: rgba(255, 255, 255, 0.25);
            color: #ffffff;
        }

        .upload-status.error {
            background: rgba(220, 38, 38, 0.25);
            color: #ffffff;
        }

        .upload-status.info {
            background: rgba(255, 255, 255, 0.2);
            color: #ffffff;
        }

        .empty-state {
            background: rgba(148, 163, 184, 0.12);
            border-radius: 12px;
            padding: 1.25rem;
            border: 1px dashed #cbd5f5;
            color: #64748b;
            text-align: center;
            margin-bottom: 1rem;
        }

        .empty-state.hidden {
            display: none;
        }

        .folder-grid {
            display: grid;
            grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
            gap: 0.75rem;
        }

        .folder-card {
            display: flex;
            align-items: center;
            gap: 0.875rem;
            padding: 1rem 1.25rem;
            border-radius: 8px;
            text-decoration: none;
            background: #ffffff;
            border: 1px solid #e8e8e8;
            transition: all 0.15s ease;
        }

        .folder-card:hover {
            transform: translateY(-2px);
            border-color: #d0d0d0;
            box-shadow: 0 4px 12px rgba(0, 0, 0, 0.08);
        }

        .folder-card__icon {
            font-size: 1.5rem;
        }

        .folder-card__meta {
            display: flex;
            flex-direction: column;
            gap: 0.25rem;
        }

        .folder-card__name {
            font-weight: 600;
            color: #1a1a1a;
            font-size: 0.9375rem;
        }

        .folder-card__path {
            font-size: 0.75rem;
            color: #808080;
            font-family: "SF Mono", Monaco, monospace;
        }

        .folder-card__arrow {
            margin-left: auto;
            font-size: 0.875rem;
            color: #b0b0b0;
        }

        .folder-empty {
            color: #b0b0b0;
            font-style: italic;
            font-size: 0.875rem;
        }

        .file-list {
            display: flex;
            flex-direction: column;
            gap: 0.5rem;
            margin: 0;
            padding: 0;
        }

        .file-entry {
            display: flex;
            align-items: center;
            gap: 0.875rem;
            padding: 1rem 1.25rem;
            border-radius: 8px;
            border: 1px solid #e8e8e8;
            background: #ffffff;
            text-decoration: none;
            color: #1a1a1a;
            transition: all 0.15s ease;
        }

        .file-entry:hover {
            transform: translateY(-1px);
            border-color: #d0d0d0;
            box-shadow: 0 2px 8px rgba(0, 0, 0, 0.06);
        }

        .file-entry__icon {
            font-size: 1.25rem;
        }

        .file-entry__text {
            display: flex;
            flex-direction: column;
            gap: 0.25rem;
        }

        .file-entry__name {
            font-weight: 600;
            color: #1a1a1a;
            font-size: 0.9375rem;
        }

        .file-entry__path {
            font-size: 0.75rem;
            color: #808080;
            font-family: "SF Mono", Monaco, monospace;
        }

        .file-entry__arrow {
            margin-left: auto;
            color: #b0b0b0;
            font-size: 0.875rem;
        }
        "#
    } else {
        ""
    };

    let dynamic_script = if use_htmx {
        ajax::dynamic_script().to_string()
    } else {
        String::new()
    };

    format!(
        r#"<!DOCTYPE html>
<html lang="{lang_code}">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{directory_title} - {escaped_dir_path}</title>
    <style>
        * {{ margin: 0; padding: 0; box-sizing: border-box; }}

        body {{
            font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", sans-serif;
            color: #1a1a1a;
            background: #fafafa;
            min-height: 100vh;
            padding: 2rem 1rem;
            -webkit-font-smoothing: antialiased;
        }}

        .container {{
            max-width: 1200px;
            margin: 0 auto;
            background: #ffffff;
            padding: 2.5rem;
            border-radius: 12px;
            border: 1px solid #e8e8e8;
            box-shadow: 0 1px 3px rgba(0, 0, 0, 0.05);
            animation: fadeIn 0.3s ease-out;
        }}

        @keyframes fadeIn {{
            from {{ opacity: 0; transform: translateY(10px); }}
            to {{ opacity: 1; transform: translateY(0); }}
        }}

        .header {{
            margin-bottom: 2rem;
            padding-bottom: 1.5rem;
            border-bottom: 1px solid #e8e8e8;
        }}

        h1 {{
            font-size: 1.75rem;
            font-weight: 600;
            display: flex;
            align-items: center;
            gap: 0.5rem;
            color: #1a1a1a;
        }}

        .directory-path {{
            color: #606060;
            font-size: 0.875rem;
            margin-top: 0.5rem;
        }}

        .directory-path code {{
            background: #f5f5f5;
            padding: 0.25rem 0.5rem;
            border-radius: 3px;
            font-family: "SF Mono", Monaco, monospace;
            color: #404040;
            font-size: 0.8125rem;
        }}

        .empty-state {{
            background: #f8f8f8;
            border-radius: 8px;
            padding: 2rem;
            border: 1px dashed #d0d0d0;
            color: #606060;
            text-align: center;
            margin-bottom: 1rem;
        }}

        .empty-state.hidden {{
            display: none;
        }}

        {layout_styles}

        @media (max-width: 768px) {{
            body {{
                padding: 1rem;
            }}

            .container {{
                padding: 1.5rem;
                border-radius: 8px;
            }}

            h1 {{
                font-size: 1.5rem;
            }}

            .upload-card {{
                flex-direction: column;
                padding: 1.5rem;
                gap: 1rem;
            }}

            .upload-card__icon {{
                font-size: 2.5rem;
            }}

            .upload-card__title {{
                font-size: 1.2rem;
            }}

            .upload-card__description {{
                font-size: 0.875rem;
            }}

            .upload-card__button {{
                padding: 0.65rem 1.2rem;
                font-size: 0.875rem;
            }}

            .folder-grid {{
                grid-template-columns: 1fr;
                gap: 0.5rem;
            }}

            .file-entry,
            .folder-card {{
                padding: 0.875rem 1rem;
            }}
        }}
    </style>
    {dynamic_script}
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>📁 {directory_label}</h1>
            <div class="directory-path">{directory_path_label}: <code>{escaped_dir_path}</code></div>
        </div>
        {file_list_markup}
    </div>
</body>
</html>"#,
        lang_code = lang_code,
        directory_title = directory_title,
        escaped_dir_path = escaped_dir_path,
        layout_styles = layout_styles,
        dynamic_script = dynamic_script,
        directory_label = directory_label,
        directory_path_label = directory_path_label,
        file_list_markup = file_list_markup,
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
