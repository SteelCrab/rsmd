/// Korean language strings
pub fn get_text(key: &str) -> &'static str {
    match key {
        "title_viewer" => "마크다운 뷰어",
        "title_raw" => "원본 마크다운",
        "title_directory" => "마크다운 디렉토리",
        "directory_label" => "마크다운 파일 목록",
        "directory_path" => "디렉토리",
        "no_files" => "이 디렉토리에서 마크다운 파일을 찾을 수 없습니다.",
        "error_invalid_mode" => "오류: 잘못된 모드",
        "error_not_found" => "404 - 파일을 찾을 수 없습니다",
        "error_reading_file" => "파일 읽기 오류",
        _ => "",
    }
}
