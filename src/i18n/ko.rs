/// Korean language strings
pub fn get_text(key: &str) -> &'static str {
    match key {
        "title_viewer" => "마크다운 뷰어",
        "title_raw" => "원본 마크다운",
        "title_directory" => "마크다운 디렉토리",
        "directory_label" => "마크다운 파일 목록",
        "directory_path" => "디렉토리",
        "no_files" => "이 디렉토리에서 마크다운 파일을 찾을 수 없습니다.",
        "folders_label" => "폴더",
        "files_heading" => "마크다운 파일",
        "breadcrumb_root" => "홈",
        "back_to_parent" => "상위 폴더로 돌아가기",
        "error_invalid_mode" => "오류: 잘못된 모드",
        "error_not_found" => "404 - 파일을 찾을 수 없습니다",
        "error_reading_file" => "파일 읽기 오류",
        "upload_title" => "마크다운 파일 추가",
        "upload_instructions" => "마크다운 파일을 끌어다 놓거나 파일 찾기를 클릭하세요.",
        "upload_browse" => "파일 선택",
        "upload_success" => "업로드 완료! 파일을 불러오는 중...",
        "upload_error" => "파일 업로드에 실패했습니다.",
        "upload_invalid_type" => "md 확장자 파일만 지원됩니다.",
        "upload_uploading" => "업로드 중...",
        _ => "",
    }
}
