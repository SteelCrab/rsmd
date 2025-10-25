use rsmd::{ServerConfig, markdown::MarkdownParser, server, server::AppState};
use std::env;
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

    // Get markdown file path from command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <markdown-file.md>", args[0]);
        std::process::exit(1);
    }

    let file_path = &args[1];

    // Parse markdown file
    let parser = MarkdownParser::from_file(file_path).unwrap_or_else(|err| {
        eprintln!("Error reading file '{}': {}", file_path, err);
        std::process::exit(1);
    });

    let state = Arc::new(AppState {
        markdown_content: parser.raw_content().to_string(),
        html_content: parser.to_html(),
    });

    // Create router
    let app = server::create_router(state);

    // Server configuration
    let config = ServerConfig::default();
    let addr = config.address();
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    println!("🚀 Markdown viewer running at http://{}", addr);
    println!("   View rendered: http://{}/", addr);
    println!("   View raw:      http://{}/raw", addr);

    axum::serve(listener, app).await.unwrap();
}
