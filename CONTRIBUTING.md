# Contributing Guidelines

Thanks for taking the time to contribute to **rsmd**! This document explains how to work on the project, the checks we expect before a pull request (PR) is raised, and tips for running the app locally.

## 1. Workflow Overview

1. Create a feature branch from `main`. Do not push directly to `main` (there is a branch protection rule enforcing this).
2. Make your changes and keep commits focused. We encourage using conventional commit prefixes (e.g. `feat:`, `fix:`, `ci:`).
3. Run formatting, linting, and tests locally (details below).
4. Open a PR against `main`, fill in a short description, and list the verification steps you ran.
5. Ensure the PR CI run is green before requesting review.

### Optional pre-commit hook

We ship a git hook that mirrors the CI checks:

```bash
git config core.hooksPath githooks
```

This will automatically run `cargo fmt` and `cargo clippy -- -D warnings` before each commit.

## 2. Local Development

Install the nightly toolchain pinned by `rust-toolchain.toml` and compile:

```bash
rustup toolchain install nightly
cargo build
```

Run the entire validation suite before committing:

```bash
cargo fmt
cargo clippy -- -D warnings
cargo test --workspace
```

If you modify directory handling or Markdown rendering, add or update tests in `tests/` to cover the new behaviour.

## 3. Running the Application

Serve a single Markdown file:

```bash
cargo run -- ./sample.md
```

Serve an entire directory (supports nested folders):

```bash
cargo run -- ./test_docs
```

The server listens on `http://127.0.0.1:3000`. Directory entries open dedicated pages instead of inline previews, so each document can be linked directly.

## 4. Pull Request Checklist

- ✅ `cargo build`
- ✅ `cargo fmt`
- ✅ `cargo clippy -- -D warnings`
- ✅ `cargo test --workspace`
- ✅ Added/updated tests (if logic changed)
- ✅ Updated docs if behaviour or workflow changed
- ✅ PR description summarises the change and lists tests run
- ✅ Screenshots included for UI-facing changes

Dependabot PRs automatically skip Codecov uploads to avoid token issues; manual PRs still require successful coverage uploads.

## 5. Continuous Integration

GitHub Actions runs the following on every PR/push:

- `cargo fmt -- --check`
- `cargo clippy -- -D warnings`
- `cargo test --workspace`
- Coverage checks (`cargo tarpaulin` + `cargo llvm-cov`) on Ubuntu
- Security audit (`cargo audit`) and policy checks (`cargo deny`)
- Musl cross compiles for x86_64 and aarch64 via `cross`

Keep CI green by running the same commands locally before pushing.

## 6. Need Help?

Open a draft PR or start a GitHub Discussion if you want early feedback. Maintainers are happy to guide you through the process. Thanks again for helping improve rsmd! 🎉
