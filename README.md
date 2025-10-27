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
git clone https://github.com/SteelCrab/rsmd.git
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
Every push/PR to `main` runs automated checks:

**Quality Checks**:
- ✅ Tests on all platforms (Ubuntu, macOS, Windows)
- ✅ Clippy linter with `-D warnings` (strict mode)
- ✅ Code formatting check with `cargo fmt`
- ✅ Security audit (RustSec and cargo-deny)
- ✅ Code coverage (minimum 80% required)

**Build Matrix**:
- Linux x86_64 & ARM64
- macOS ARM64 (Apple Silicon)
- Windows x86_64

### Releases
Create a release by pushing a version tag:

```bash
git tag v0.1.0
git push origin v0.1.0
```

This automatically builds and publishes binaries for all supported platforms.

## Contributors

<!-- ALL-CONTRIBUTORS-BADGE:START - Do not remove or modify this section -->
[![All Contributors](https://img.shields.io/badge/all_contributors-1-orange.svg?style=flat-square)](#contributors-)
<!-- ALL-CONTRIBUTORS-BADGE:END -->

<!-- ALL-CONTRIBUTORS-LIST:START - Do not remove or modify this section -->
<!-- prettier-ignore-start -->
<!-- markdownlint-disable -->
<table>
  <tbody>
    <tr>
      <td align="center"><a href="https://github.com/SteelCrab"><img src="https://github.com/SteelCrab.png" width="100px;" alt="SteelCrab"/><br /><sub><b>SteelCrab</b></sub></a><br /><a href="#code-SteelCrab" title="Code">💻</a> <a href="#infra-SteelCrab" title="Infrastructure (Hosting, Build-Tools, etc)">🚇</a></td>
    </tr>
  </tbody>
</table>
<!-- markdownlint-enable -->
<!-- prettier-ignore-end -->
<!-- ALL-CONTRIBUTORS-LIST:END -->

Thanks goes to these wonderful people!

To contribute, comment on an issue or PR with:
```
@all-contributors please add @username for <contribution type>
```

**Contribution types**: code, docs, bug, feature, design, infra, maintenance, test, etc.

## License

MIT
