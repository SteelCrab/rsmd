use pulldown_cmark::{Options, Parser, html};
use std::fs;
use std::io;

/// Markdown parser that converts markdown text to HTML
pub struct MarkdownParser {
    content: String,
}

impl MarkdownParser {
    /// Create a new MarkdownParser from a string
    pub fn new(content: String) -> Self {
        Self { content }
    }

    /// Create a new MarkdownParser by reading from a file
    pub fn from_file(path: &str) -> io::Result<Self> {
        let content = fs::read_to_string(path)?;
        Ok(Self::new(content))
    }

    /// Get the raw markdown content
    pub fn raw_content(&self) -> &str {
        &self.content
    }

    /// Convert markdown to HTML
    pub fn to_html(&self) -> String {
        let parser = Parser::new_ext(&self.content, Options::all());
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);
        html_output
    }
}
