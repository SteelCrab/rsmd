use rsmd::{Language, ServerConfig, directory, markdown::MarkdownParser, server, server::AppState};
use std::collections::HashMap;
use std::env;
use std::path::Path;
use std::sync::Arc;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "rsmd=debug,tower_http=debug,axum::rejection=trace".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Detect system language
    let language = Language::detect();

    // Get path from command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <markdown-file.md|directory>", args[0]);
        std::process::exit(1);
    }

    let path = &args[1];
    let path_obj = Path::new(path);

    // Determine if path is a file or directory
    let state = if path_obj.is_file() {
        // Single file mode
        let parser = MarkdownParser::from_file(path).unwrap_or_else(|err| {
            eprintln!("Error reading file '{}': {}", path, err);
            std::process::exit(1);
        });

        // Get parent directory for static file serving
        let base_dir = path_obj
            .parent()
            .unwrap_or_else(|| Path::new("."))
            .to_path_buf();

        Arc::new(AppState::SingleFile {
            markdown_content: parser.raw_content().to_string(),
            html_content: parser.to_html(),
            language: language.clone(),
            base_dir,
        })
    } else if path_obj.is_dir() {
        // Directory mode
        let files = directory::scan_markdown_files(path).unwrap_or_else(|err| {
            eprintln!("Error scanning directory '{}': {}", path, err);
            std::process::exit(1);
        });

        if files.is_empty() {
            eprintln!("Warning: No markdown files found in directory '{}'", path);
        }

        Arc::new(AppState::Directory {
            dir_path: path.to_string(),
            files,
            file_cache: Arc::new(HashMap::new()),
            language: language.clone(),
            base_dir: path_obj.to_path_buf(),
        })
    } else {
        eprintln!("Error: '{}' is not a valid file or directory", path);
        std::process::exit(1);
    };

    // Create router
    let app = server::create_router(state.clone());

    // Server configuration
    let config = ServerConfig::default();
    let addr = config.address();
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    // Display appropriate startup message based on mode
    match state.as_ref() {
        AppState::SingleFile { .. } => {
            println!("🚀 Markdown viewer running at http://{}", addr);
            println!("   View rendered: http://{}/", addr);
            println!("   View raw:      http://{}/raw", addr);
        }
        AppState::Directory { files, .. } => {
            println!("🚀 Markdown directory viewer running at http://{}", addr);
            println!("   Directory listing: http://{}/", addr);
            println!("   Found {} markdown file(s)", files.len());
        }
    }

    axum::serve(listener, app).await.unwrap();
}
