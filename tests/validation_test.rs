//! Validation tests for configuration and documentation files
//!
//! These tests ensure that `Cargo.toml` maintains proper structure, valid syntax,
//! and required content.

use std::fs;
use std::path::Path;

// ============================================================================
// CONTRIBUTING.md Tests
// ============================================================================

// ============================================================================
// README.md Tests
// ============================================================================

// ============================================================================
// Cargo.toml Tests
// ============================================================================

#[test]
fn test_cargo_toml_exists() {
    assert!(
        Path::new("Cargo.toml").exists(),
        "Cargo.toml should exist in the repository root"
    );
}

#[test]
fn test_cargo_toml_is_valid() {
    let content = fs::read_to_string("Cargo.toml").expect("Should be able to read Cargo.toml");

    // Basic TOML validation - should have package section
    assert!(
        content.contains("[package]"),
        "Should have [package] section"
    );
    assert!(
        content.contains("[dependencies]"),
        "Should have [dependencies] section"
    );
}

#[test]
fn test_cargo_toml_has_required_package_fields() {
    let content = fs::read_to_string("Cargo.toml").expect("Should be able to read Cargo.toml");

    // Required fields
    assert!(
        content.contains("name = \"rsmd\""),
        "Should have package name 'rsmd'"
    );
    assert!(content.contains("version ="), "Should have version field");
    assert!(content.contains("edition ="), "Should have edition field");
}

#[test]
fn test_cargo_toml_package_name() {
    let content = fs::read_to_string("Cargo.toml").expect("Should be able to read Cargo.toml");

    assert!(
        content.contains("name = \"rsmd\""),
        "Package name should be 'rsmd'"
    );
}

#[test]
fn test_cargo_toml_has_version() {
    let content = fs::read_to_string("Cargo.toml").expect("Should be able to read Cargo.toml");

    // Version should be in semver format
    let version_line = content
        .lines()
        .find(|line| line.trim().starts_with("version ="))
        .expect("Should have version field");

    assert!(
        version_line.contains("0.1.0") || version_line.contains('"'),
        "Version should be specified"
    );
}

#[test]
fn test_cargo_toml_has_edition() {
    let content = fs::read_to_string("Cargo.toml").expect("Should be able to read Cargo.toml");

    assert!(
        content.contains("edition = \"2024\"") || content.contains("edition = \"2021\""),
        "Should specify Rust edition"
    );
}

#[test]
fn test_cargo_toml_has_core_dependencies() {
    let content = fs::read_to_string("Cargo.toml").expect("Should be able to read Cargo.toml");

    let required_deps = vec![
        "axum",
        "tokio",
        "pulldown-cmark",
        "tower",
        "tower-http",
        "serde",
    ];

    for dep in required_deps {
        assert!(content.contains(dep), "Should have dependency '{}'", dep);
    }
}

#[test]
fn test_cargo_toml_tower_version() {
    let content = fs::read_to_string("Cargo.toml").expect("Should be able to read Cargo.toml");

    // Check for tower version (the change in this PR)
    assert!(
        content.contains("tower = { version = \"0.5\""),
        "tower should be version 0.5"
    );
}

#[test]
fn test_cargo_toml_tower_http_version() {
    let content = fs::read_to_string("Cargo.toml").expect("Should be able to read Cargo.toml");

    // Check for tower-http version (the change in this PR)
    assert!(
        content.contains("tower-http = { version = \"0.6\""),
        "tower-http should be version 0.6"
    );
}

#[test]
fn test_cargo_toml_has_dev_dependencies() {
    let content = fs::read_to_string("Cargo.toml").expect("Should be able to read Cargo.toml");

    assert!(
        content.contains("[dev-dependencies]"),
        "Should have dev-dependencies section"
    );
    assert!(
        content.contains("tempfile"),
        "Should have tempfile for testing"
    );
}

#[test]
fn test_cargo_toml_has_release_profile() {
    let content = fs::read_to_string("Cargo.toml").expect("Should be able to read Cargo.toml");

    assert!(
        content.contains("[profile.release]"),
        "Should have release profile configuration"
    );
}

#[test]
fn test_cargo_toml_release_optimizations() {
    let content = fs::read_to_string("Cargo.toml").expect("Should be able to read Cargo.toml");

    // Check for optimization settings
    if content.contains("[profile.release]") {
        assert!(
            content.contains("opt-level"),
            "Should specify optimization level"
        );
        assert!(content.contains("lto"), "Should specify LTO settings");
    }
}

#[test]
fn test_cargo_toml_tokio_has_full_features() {
    let content = fs::read_to_string("Cargo.toml").expect("Should be able to read Cargo.toml");

    // Tokio should have "full" features for async runtime
    assert!(
        content.contains("tokio") && content.contains("features") && content.contains("full"),
        "tokio should have 'full' features enabled"
    );
}

#[test]
fn test_cargo_toml_no_syntax_errors() {
    let content = fs::read_to_string("Cargo.toml").expect("Should be able to read Cargo.toml");

    // Basic syntax checks
    let open_brackets = content.matches('[').count();
    let close_brackets = content.matches(']').count();
    assert_eq!(
        open_brackets, close_brackets,
        "Brackets should be balanced in TOML"
    );

    // Check for properly quoted strings
    let quotes = content.matches('"').count();
    assert!(quotes.is_multiple_of(2), "Quotes should be balanced");
}

// ============================================================================
// Cross-file Consistency Tests
// ============================================================================

// ============================================================================
// Additional Quality Checks
// ============================================================================

#[test]
fn test_cargo_toml_ends_with_newline() {
    let content = fs::read_to_string("Cargo.toml").expect("Should be able to read Cargo.toml");

    assert!(
        content.ends_with('\n'),
        "Cargo.toml should end with a newline"
    );
}

#[test]
fn test_cargo_toml_dependencies_have_versions() {
    let content = fs::read_to_string("Cargo.toml").expect("Should be able to read Cargo.toml");

    let mut in_dependencies = false;
    let mut in_dev_dependencies = false;

    for line in content.lines() {
        let trimmed = line.trim();

        if trimmed == "[dependencies]" {
            in_dependencies = true;
            in_dev_dependencies = false;
        } else if trimmed == "[dev-dependencies]" {
            in_dependencies = false;
            in_dev_dependencies = true;
        } else if trimmed.starts_with('[') {
            in_dependencies = false;
            in_dev_dependencies = false;
        }

        // Check that dependency lines have version specifications
        if (in_dependencies || in_dev_dependencies)
            && !trimmed.is_empty()
            && !trimmed.starts_with('#')
        {
            if trimmed.contains('=') && !trimmed.starts_with('[') {
                assert!(
                    trimmed.contains("version") || trimmed.contains('"'),
                    "Dependency '{}' should have a version specification",
                    trimmed.split('=').next().unwrap_or("").trim()
                );
            }
        }
    }
}
