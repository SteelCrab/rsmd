use rsmd::i18n::Language;

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
