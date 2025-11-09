use rsmd::markdown::MarkdownParser;
use std::io::Write;
use tempfile::NamedTempFile;

#[test]
fn test_markdown_to_html_simple() {
    let parser = MarkdownParser::new("# Hello World".to_string());
    let html = parser.to_html();
    assert!(html.contains("<h1>"));
    assert!(html.contains("Hello World"));
}

#[test]
fn test_markdown_to_html_bold() {
    let parser = MarkdownParser::new("**bold text**".to_string());
    let html = parser.to_html();
    assert!(html.contains("<strong>"));
    assert!(html.contains("bold text"));
}

#[test]
fn test_markdown_to_html_code() {
    let parser = MarkdownParser::new("`inline code`".to_string());
    let html = parser.to_html();
    assert!(html.contains("<code>"));
    assert!(html.contains("inline code"));
}

#[test]
fn test_raw_content() {
    let content = "# Test\n\nSome content";
    let parser = MarkdownParser::new(content.to_string());
    assert_eq!(parser.raw_content(), content);
}

#[test]
fn test_from_file_reads_content() {
    let mut tempfile = NamedTempFile::new().expect("temp file");
    write!(tempfile, "# Title\n\nBody").expect("write markdown");

    let parser =
        MarkdownParser::from_file(tempfile.path().to_str().expect("path utf8")).expect("parser");

    assert_eq!(parser.raw_content().trim(), "# Title\n\nBody");
    let html = parser.to_html();
    assert!(html.contains("<h1"));
}

#[test]
fn test_from_file_missing_returns_error() {
    let tempfile = NamedTempFile::new().expect("temp file");
    let path = tempfile.path().to_path_buf();
    drop(tempfile);

    let result = MarkdownParser::from_file(path.to_str().expect("path utf8"));
    assert!(result.is_err());
}

#[test]
fn test_markdown_parser_complex_markdown() {
    let parser =
        MarkdownParser::new("# Header\n\n- List item\n- Another item\n\n**Bold text**".to_string());
    let html = parser.to_html();
    assert!(html.contains("<h1>"));
    assert!(html.contains("<ul>"));
    assert!(html.contains("<li>"));
    assert!(html.contains("<strong>"));
}

#[test]
fn test_markdown_parser_tables() {
    let parser = MarkdownParser::new(
        "| Column 1 | Column 2 |\n|----------|----------|\n| Cell 1   | Cell 2   |".to_string(),
    );
    let html = parser.to_html();
    assert!(html.contains("<table>"));
    assert!(html.contains("<th>"));
    assert!(html.contains("<td>"));
}

#[test]
fn test_markdown_parser_code_blocks() {
    let parser =
        MarkdownParser::new("```rust\nfn main() {\n    println!(\"Hello\");\n}\n```".to_string());
    let html = parser.to_html();
    assert!(html.contains("<pre>"));
    assert!(html.contains("<code"));
}
