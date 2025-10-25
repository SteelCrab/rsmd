# rsmd - Rust Markdown Viewer

A fast, simple markdown file viewer that renders `.md` files in your web browser.

## Features

- 🚀 **Fast rendering** - Built with Rust for maximum performance
- 🎨 **Clean styling** - Beautiful, readable HTML output
- 🌐 **Web-based** - View in any browser
- ✅ **Well-tested** - Comprehensive unit tests
- 🔧 **Modular** - Clean architecture with separated concerns

## Installation

### From Source

```bash
git clone <repository-url>
cd rsmd
cargo build --release
```

The binary will be available at `target/release/rsmd`.

### From Release

Download pre-built binaries from the [Releases](../../releases) page for:
- Linux (x86_64)
- macOS (x86_64 and Apple Silicon)

## Usage

```bash
rsmd <markdown-file.md>
```

This will start a web server at `http://127.0.0.1:3000`.

- View rendered markdown: `http://127.0.0.1:3000/`
- View raw markdown: `http://127.0.0.1:3000/raw`

### Example

```bash
cargo run --release sample.md
```

Then open your browser to `http://127.0.0.1:3000`.

## Development

### Build

```bash
cargo build          # Debug build
cargo build --release  # Release build
```

### Test

```bash
cargo test           # Run all tests
cargo clippy         # Run linter
cargo fmt            # Format code
```

### Project Structure

```
rsmd/
├── src/
│   ├── main.rs       # CLI entry point
│   ├── lib.rs        # Library root
│   ├── markdown.rs   # Markdown parsing
│   ├── html.rs       # HTML template generation
│   └── server.rs     # Web server and routing
├── .github/
│   └── workflows/
│       ├── ci.yml       # CI testing
│       └── release.yml  # Release automation
└── sample.md         # Example markdown file
```

## CI/CD

### Continuous Integration
Every push/PR runs:
- Tests on Ubuntu and macOS
- Clippy linting
- Format checking

### Releases
Create a release by pushing a version tag:

```bash
git tag v0.1.0
git push origin v0.1.0
```

This automatically builds and publishes binaries for all supported platforms.

## License

MIT
