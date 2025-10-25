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
