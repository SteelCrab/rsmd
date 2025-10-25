use rsmd::markdown::MarkdownParser;

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
