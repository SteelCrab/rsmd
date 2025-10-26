use rsmd::i18n::Language;
use std::sync::{Mutex, OnceLock};

fn env_lock() -> &'static Mutex<()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
}

fn set_lang(value: Option<&str>) {
    unsafe {
        match value {
            Some(val) => std::env::set_var("LANG", val),
            None => std::env::remove_var("LANG"),
        }
    }
}

#[test]
fn test_language_text_english() {
    let lang = Language::English;
    assert_eq!(lang.text("title_viewer"), "Markdown Viewer");
    assert_eq!(lang.text("directory_label"), "Markdown Files");
}

#[test]
fn test_language_detect_korean_from_env() {
    let _guard = env_lock().lock().unwrap();
    let original = std::env::var("LANG").ok();
    set_lang(Some("ko_KR.UTF-8"));

    assert_eq!(Language::detect(), Language::Korean);

    set_lang(original.as_deref());
}

#[test]
fn test_language_detect_defaults_to_english() {
    let _guard = env_lock().lock().unwrap();
    let original = std::env::var("LANG").ok();
    set_lang(None);

    assert_eq!(Language::detect(), Language::English);

    set_lang(original.as_deref());
}

#[test]
fn test_language_text_korean() {
    let lang = Language::Korean;
    assert_eq!(lang.text("title_viewer"), "마크다운 뷰어");
    assert_eq!(lang.text("directory_label"), "마크다운 파일 목록");
}
