mod en;
mod ko;

use std::env;

#[derive(Clone, Debug, PartialEq)]
pub enum Language {
    English,
    Korean,
}

impl Language {
    /// Detect system language from environment variables
    pub fn detect() -> Self {
        if let Ok(lang) = env::var("LANG")
            && (lang.starts_with("ko") || lang.starts_with("ko_KR"))
        {
            return Language::Korean;
        }
        Language::English
    }

    /// Get localized text for a given key
    pub fn text(&self, key: &str) -> &'static str {
        match self {
            Language::English => en::get_text(key),
            Language::Korean => ko::get_text(key),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language_text_english() {
        let lang = Language::English;
        assert_eq!(lang.text("title_viewer"), "Markdown Viewer");
        assert_eq!(lang.text("directory_label"), "Markdown Files");
    }

    #[test]
    fn test_language_text_korean() {
        let lang = Language::Korean;
        assert_eq!(lang.text("title_viewer"), "마크다운 뷰어");
        assert_eq!(lang.text("directory_label"), "마크다운 파일 목록");
    }
}
