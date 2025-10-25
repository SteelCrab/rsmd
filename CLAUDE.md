# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

`rsmd` is a Rust-based Markdown file viewer designed to display `.md` files easily in a web browser. The project uses Rust edition 2024.

### Project Goals
- Parse and render Markdown files
- Serve rendered content through a web interface
- Provide an easy-to-use browser-based viewer

## Development Commands

### Building
```bash
cargo build          # Debug build
cargo build --release  # Release build
```

### Running
```bash
cargo run           # Run debug build
cargo run --release # Run release build
```

### Testing
```bash
cargo test          # Run all tests
cargo test <test_name>  # Run specific test
cargo test -- --nocapture  # Show println! output during tests
```

### Code Quality
```bash
cargo check         # Quick compilation check
cargo clippy        # Lint with Clippy
cargo fmt           # Format code
cargo fmt -- --check  # Check formatting without modifying files
```

## Project Structure

```
rsmd/
├── src/
│   ├── main.rs       # CLI entry point
│   ├── lib.rs        # Library root with module exports
│   ├── markdown.rs   # Markdown parsing (pulldown-cmark)
│   ├── html.rs       # HTML template generation
│   └── server.rs     # Web server (Axum) and routing
├── .github/
│   └── workflows/
│       ├── ci.yml       # CI: test, clippy, fmt on push/PR
│       └── release.yml  # Release: build binaries on tags
└── Cargo.toml       # Dependencies and metadata
```

### Architecture

- **Modular design**: Core logic separated into focused modules
- **MarkdownParser** (markdown.rs): Handles file reading and HTML conversion
- **HTML templates** (html.rs): Generates styled HTML pages with escaping
- **Web server** (server.rs): Axum routes for `/` (rendered) and `/raw` (source)
- **Main** (main.rs): Minimal CLI that wires modules together

## CI/CD

### Continuous Integration
On every push/PR to `master` or `main`:
- Runs tests on Ubuntu and macOS
- Runs `cargo clippy` (no warnings allowed)
- Checks code formatting with `cargo fmt`

### Release
On version tags (e.g., `v0.1.0`):
- Builds optimized binaries for:
  - Linux (x86_64)
  - macOS (x86_64 and Apple Silicon)
- Creates GitHub release with artifacts

**To create a release:**
```bash
git tag v0.1.0
git push origin v0.1.0
```
