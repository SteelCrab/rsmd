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

- `src/main.rs` - Application entry point
- `Cargo.toml` - Project manifest and dependencies
