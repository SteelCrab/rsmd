# Repository Guidelines

## Project Structure & Module Organization
- `src/` contains the Rust application: `main.rs` bootstraps the Axum server, while modules like `markdown.rs`, `html.rs`, `server.rs`, `ajax.rs`, and `directory.rs` keep rendering, routing, and filesystem logic isolated; `i18n/` stores locale bundles consumed by `i18n.rs`.
- `static/` serves shared assets in directory mode, and `sample.md` is the quick-start document for manual smoke tests.
- `tests/` holds integration-style checks (one file per feature) and `test_docs/` provides markdown fixtures used by those tests.
- `target/` is cargo’s build output; never commit its contents.

## Build, Test, and Development Commands
- `cargo build` and `cargo check --all-targets` validate compilation; use the nightly toolchain pinned in `rust-toolchain.toml`.
- `cargo run -- sample.md` launches the viewer against the bundled example at `http://127.0.0.1:3000`.
- `cargo clippy --all-targets --all-features -D warnings` enforces lint cleanliness to match CI.
- `cargo fmt` formats the workspace; run it before every commit.

## Coding Style & Naming Conventions
- Follow Rust defaults: four-space indentation, snake_case for modules/functions, UpperCamelCase for types, and SCREAMING_SNAKE_CASE for constants.
- Keep modules small and cohesive; prefer adding a new file in `src/` over growing `main.rs`.
- Document non-trivial functions with `///` doc comments and prefer `tracing` spans for runtime context.

## Testing Guidelines
- `cargo test --all-targets` runs unit and integration suites; add focused cases in `tests/<feature>_test.rs` when extending behavior.
- Maintain the CI coverage floor (80%); if you need a local check, run `cargo llvm-cov --workspace` after installing `cargo-llvm-cov`.
- Mark slow or environment-dependent tests with `#[ignore]` and note activation steps in the file header.

## Commit & Pull Request Guidelines
- Use Conventional Commit prefixes observed in history (e.g., `feat:`, `fix:`, `docs:`, `ci:`) and keep messages in the imperative mood.
- Each PR should describe the user-facing impact, list manual/automated test commands executed, and link related issues.
- Include screenshots or terminal captures when modifying HTML output or logging.
- Ensure `cargo fmt`, `cargo clippy`, and `cargo test` pass locally before requesting review.

## Configuration & Observability Tips
- The server binds to `127.0.0.1:3000` via `ServerConfig`; adjust host/port inside `src/server.rs` if you need different defaults.
- Set `RUST_LOG=rsmd=debug,tower_http=debug` to mirror the tracing filters used in CI and diagnose routing issues quickly.
