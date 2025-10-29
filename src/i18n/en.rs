/// English language strings
pub fn get_text(key: &str) -> &'static str {
    match key {
        "title_viewer" => "Markdown Viewer",
        "title_raw" => "Raw Markdown",
        "title_directory" => "Markdown Directory",
        "directory_label" => "Markdown Files",
        "directory_path" => "Directory",
        "no_files" => "No markdown files found in this directory.",
        "error_invalid_mode" => "Error: Invalid mode",
        "error_not_found" => "404 - File not found",
        "error_reading_file" => "Error reading file",
        "upload_title" => "Add markdown file",
        "upload_instructions" => "Drag & drop a markdown file here or click to browse.",
        "upload_browse" => "Browse file",
        "upload_success" => "Upload complete! Loading file...",
        "upload_error" => "Failed to upload file.",
        "upload_invalid_type" => "Only markdown (.md) files are supported.",
        "upload_uploading" => "Uploading…",
        _ => "",
    }
}
