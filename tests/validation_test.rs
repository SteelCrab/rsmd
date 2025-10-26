//! Validation tests for configuration and documentation files
//! 
//! These tests ensure that CONTRIBUTING.md, README.md, and Cargo.toml
//! maintain proper structure, valid syntax, and required content.

use rsmd::markdown::MarkdownParser;
use std::fs;
use std::path::Path;

// ============================================================================
// CONTRIBUTING.md Tests
// ============================================================================

#[test]
fn test_contributing_file_exists() {
    assert!(
        Path::new("CONTRIBUTING.md").exists(),
        "CONTRIBUTING.md should exist in the repository root"
    );
}

#[test]
fn test_contributing_is_valid_markdown() {
    let content = fs::read_to_string("CONTRIBUTING.md")
        .expect("Should be able to read CONTRIBUTING.md");
    
    let parser = MarkdownParser::new(content.clone());
    let html = parser.to_html();
    
    // Valid markdown should produce HTML output
    assert!(!html.is_empty(), "Markdown should produce HTML output");
    assert!(html.contains("<h1>") || html.contains("<h2>"), 
            "Should contain at least one heading");
}

#[test]
fn test_contributing_has_required_sections() {
    let content = fs::read_to_string("CONTRIBUTING.md")
        .expect("Should be able to read CONTRIBUTING.md");
    
    // Check for required sections
    let required_sections = vec![
        "Workflow",
        "Development",
        "Pull Request",
        "Continuous Integration",
    ];
    
    for section in required_sections {
        assert!(
            content.contains(section),
            "CONTRIBUTING.md should contain section about '{}'",
            section
        );
    }
}

#[test]
fn test_contributing_has_workflow_overview() {
    let content = fs::read_to_string("CONTRIBUTING.md")
        .expect("Should be able to read CONTRIBUTING.md");
    
    // Verify workflow mentions main branch
    assert!(content.contains("main"), "Should mention main branch");
    assert!(content.contains("PR") || content.contains("pull request"), 
            "Should mention pull requests");
    assert!(content.contains("branch"), "Should mention branching");
}

#[test]
fn test_contributing_has_local_development_commands() {
    let content = fs::read_to_string("CONTRIBUTING.md")
        .expect("Should be able to read CONTRIBUTING.md");
    
    // Check for essential cargo commands
    assert!(content.contains("cargo build"), "Should mention cargo build");
    assert!(content.contains("cargo test"), "Should mention cargo test");
    assert!(content.contains("cargo fmt"), "Should mention cargo fmt");
    assert!(content.contains("cargo clippy"), "Should mention cargo clippy");
}

#[test]
fn test_contributing_has_code_blocks() {
    let content = fs::read_to_string("CONTRIBUTING.md")
        .expect("Should be able to read CONTRIBUTING.md");
    
    // Check for code blocks (bash/shell examples)
    assert!(content.contains("```"), "Should contain code blocks");
    
    // Count opening and closing code blocks - they should match
    let opening_blocks = content.matches("```").count();
    assert!(
        opening_blocks > 0 && opening_blocks % 2 == 0,
        "Code blocks should be properly closed (found {} markers)",
        opening_blocks
    );
}

#[test]
fn test_contributing_mentions_ci_checks() {
    let content = fs::read_to_string("CONTRIBUTING.md")
        .expect("Should be able to read CONTRIBUTING.md");
    
    // CI-related content
    assert!(
        content.contains("CI") || content.contains("Continuous Integration"),
        "Should mention CI/Continuous Integration"
    );
    assert!(content.contains("GitHub Actions") || content.contains("workflow"),
            "Should mention CI system");
}

#[test]
fn test_contributing_has_pr_checklist() {
    let content = fs::read_to_string("CONTRIBUTING.md")
        .expect("Should be able to read CONTRIBUTING.md");
    
    // Look for checklist items
    assert!(
        content.contains("Pull Request Checklist") || content.contains("PR Checklist"),
        "Should have a PR checklist section"
    );
    
    // Check for checklist markers
    assert!(
        content.contains("- ✅") || content.contains("- [ ]") || content.contains("* ✅"),
        "Should contain checklist items"
    );
}

#[test]
fn test_contributing_references_rust_toolchain() {
    let content = fs::read_to_string("CONTRIBUTING.md")
        .expect("Should be able to read CONTRIBUTING.md");
    
    assert!(
        content.contains("rust-toolchain") || content.contains("toolchain"),
        "Should reference Rust toolchain configuration"
    );
}

#[test]
fn test_contributing_has_help_section() {
    let content = fs::read_to_string("CONTRIBUTING.md")
        .expect("Should be able to read CONTRIBUTING.md");
    
    assert!(
        content.contains("Help") || content.contains("help") || content.contains("question"),
        "Should have guidance on getting help"
    );
}

#[test]
fn test_contributing_markdown_structure() {
    let content = fs::read_to_string("CONTRIBUTING.md")
        .expect("Should be able to read CONTRIBUTING.md");
    
    // Check for proper heading hierarchy
    let lines: Vec<&str> = content.lines().collect();
    let mut has_h1 = false;
    let mut has_h2 = false;
    
    for line in lines {
        if line.starts_with("# ") && !line.starts_with("## ") {
            has_h1 = true;
        }
        if line.starts_with("## ") {
            has_h2 = true;
        }
    }
    
    assert!(has_h1, "Should have at least one H1 heading");
    assert!(has_h2, "Should have at least one H2 heading for sections");
}

#[test]
fn test_contributing_no_trailing_whitespace() {
    let content = fs::read_to_string("CONTRIBUTING.md")
        .expect("Should be able to read CONTRIBUTING.md");
    
    let lines_with_trailing_whitespace: Vec<usize> = content
        .lines()
        .enumerate()
        .filter(|(_, line)| line.ends_with(' ') || line.ends_with('\t'))
        .map(|(i, _)| i + 1)
        .collect();
    
    assert!(
        lines_with_trailing_whitespace.is_empty(),
        "Lines with trailing whitespace: {:?}",
        lines_with_trailing_whitespace
    );
}

// ============================================================================
// README.md Tests
// ============================================================================

#[test]
fn test_readme_file_exists() {
    assert!(
        Path::new("README.md").exists(),
        "README.md should exist in the repository root"
    );
}

#[test]
fn test_readme_is_valid_markdown() {
    let content = fs::read_to_string("README.md")
        .expect("Should be able to read README.md");
    
    let parser = MarkdownParser::new(content.clone());
    let html = parser.to_html();
    
    assert!(!html.is_empty(), "Markdown should produce HTML output");
    assert!(html.contains("<h1>") || html.contains("<h2>"), 
            "Should contain headings");
}

#[test]
fn test_readme_has_project_title() {
    let content = fs::read_to_string("README.md")
        .expect("Should be able to read README.md");
    
    // Should have project name in title
    assert!(content.contains("# rsmd") || content.contains("rsmd"), 
            "README should mention project name 'rsmd'");
    assert!(content.contains("Markdown") || content.contains("markdown"),
            "README should mention Markdown");
}

#[test]
fn test_readme_has_essential_sections() {
    let content = fs::read_to_string("README.md")
        .expect("Should be able to read README.md");
    
    let required_sections = vec![
        "Features",
        "Installation",
        "Usage",
        "Development",
        "Contributing",
    ];
    
    for section in required_sections {
        assert!(
            content.contains(section),
            "README.md should contain section about '{}'",
            section
        );
    }
}

#[test]
fn test_readme_has_contributing_link() {
    let content = fs::read_to_string("README.md")
        .expect("Should be able to read README.md");
    
    // Should link to CONTRIBUTING.md
    assert!(
        content.contains("CONTRIBUTING.md") || content.contains("Contributing Guidelines"),
        "README should reference CONTRIBUTING.md"
    );
    
    // Check for markdown link syntax
    assert!(
        content.contains("](./CONTRIBUTING.md)") || content.contains("](CONTRIBUTING.md)"),
        "README should have a proper link to CONTRIBUTING.md"
    );
}

#[test]
fn test_readme_contributing_section_references_guidelines() {
    let content = fs::read_to_string("README.md")
        .expect("Should be able to read README.md");
    
    // Find the Contributing section
    let has_contributing_section = content.contains("## Contributing");
    assert!(has_contributing_section, "Should have Contributing section");
    
    // The section should mention the guidelines
    if let Some(idx) = content.find("## Contributing") {
        let section_content = &content[idx..];
        let next_section = section_content.find("\n## ").unwrap_or(section_content.len());
        let contributing_section = &section_content[..next_section];
        
        assert!(
            contributing_section.contains("Contributing Guidelines") 
                || contributing_section.contains("CONTRIBUTING.md"),
            "Contributing section should reference the guidelines"
        );
    }
}

#[test]
fn test_readme_has_installation_instructions() {
    let content = fs::read_to_string("README.md")
        .expect("Should be able to read README.md");
    
    assert!(content.contains("Installation"), "Should have Installation section");
    assert!(content.contains("cargo build") || content.contains("cargo install"),
            "Should mention cargo build/install");
}

#[test]
fn test_readme_has_usage_examples() {
    let content = fs::read_to_string("README.md")
        .expect("Should be able to read README.md");
    
    assert!(content.contains("Usage"), "Should have Usage section");
    assert!(content.contains("```"), "Should have code examples");
    assert!(content.contains("rsmd") || content.contains("cargo run"),
            "Should show how to run the application");
}

#[test]
fn test_readme_mentions_server_address() {
    let content = fs::read_to_string("README.md")
        .expect("Should be able to read README.md");
    
    // Should mention the localhost address
    assert!(
        content.contains("127.0.0.1") || content.contains("localhost"),
        "Should mention server address"
    );
    assert!(content.contains("3000"), "Should mention port 3000");
}

#[test]
fn test_readme_has_development_commands() {
    let content = fs::read_to_string("README.md")
        .expect("Should be able to read README.md");
    
    let dev_commands = vec!["cargo build", "cargo test", "cargo clippy", "cargo fmt"];
    
    for cmd in dev_commands {
        assert!(
            content.contains(cmd),
            "README should mention development command '{}'",
            cmd
        );
    }
}

#[test]
fn test_readme_has_ci_cd_section() {
    let content = fs::read_to_string("README.md")
        .expect("Should be able to read README.md");
    
    assert!(
        content.contains("CI/CD") || content.contains("CI") || content.contains("Continuous"),
        "Should have CI/CD information"
    );
}

#[test]
fn test_readme_has_license_section() {
    let content = fs::read_to_string("README.md")
        .expect("Should be able to read README.md");
    
    assert!(content.contains("## License"), "Should have License section");
    assert!(content.contains("MIT"), "Should specify MIT license");
}

#[test]
fn test_readme_markdown_links_are_well_formed() {
    let content = fs::read_to_string("README.md")
        .expect("Should be able to read README.md");
    
    // Basic validation: check for markdown link patterns
    // Every [ should have a corresponding ]( pattern for links
    let open_brackets = content.matches('[').count();
    let link_starts = content.matches("](").count();

    assert!(
        open_brackets >= link_starts,
        "Each markdown link should have a matching opening bracket"
    );
    
    assert!(
        link_starts > 0,
        "README should contain markdown links"
    );
    
    // Count opening and closing parentheses in link contexts
    // This is a basic check - proper links should have balanced brackets and parens
    let mut check_passed = true;
    for line in content.lines() {
        if line.contains("](") {
            // Basic validation: if line has ]( it should also have a closing )
            let after_link_start = line.split("](").count() - 1;
            let closing_parens = line.matches(')').count();
            if after_link_start > 0 && closing_parens == 0 {
                check_passed = false;
                break;
            }
        }
    }
    
    assert!(check_passed, "Markdown links should be properly formed with closing parentheses");
}

#[test]
fn test_readme_references_actual_files() {
    let content = fs::read_to_string("README.md")
        .expect("Should be able to read README.md");
    
    // Check that files referenced in README actually exist
    if content.contains("CONTRIBUTING.md") {
        assert!(
            Path::new("CONTRIBUTING.md").exists(),
            "Referenced CONTRIBUTING.md should exist"
        );
    }
    
    if content.contains("Cargo.toml") {
        assert!(
            Path::new("Cargo.toml").exists(),
            "Referenced Cargo.toml should exist"
        );
    }
}

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
    let content = fs::read_to_string("Cargo.toml")
        .expect("Should be able to read Cargo.toml");
    
    // Basic TOML validation - should have package section
    assert!(content.contains("[package]"), "Should have [package] section");
    assert!(content.contains("[dependencies]"), "Should have [dependencies] section");
}

#[test]
fn test_cargo_toml_has_required_package_fields() {
    let content = fs::read_to_string("Cargo.toml")
        .expect("Should be able to read Cargo.toml");
    
    // Required fields
    assert!(content.contains("name = \"rsmd\""), "Should have package name 'rsmd'");
    assert!(content.contains("version ="), "Should have version field");
    assert!(content.contains("edition ="), "Should have edition field");
}

#[test]
fn test_cargo_toml_package_name() {
    let content = fs::read_to_string("Cargo.toml")
        .expect("Should be able to read Cargo.toml");
    
    assert!(
        content.contains("name = \"rsmd\""),
        "Package name should be 'rsmd'"
    );
}

#[test]
fn test_cargo_toml_has_version() {
    let content = fs::read_to_string("Cargo.toml")
        .expect("Should be able to read Cargo.toml");
    
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
    let content = fs::read_to_string("Cargo.toml")
        .expect("Should be able to read Cargo.toml");
    
    assert!(
        content.contains("edition = \"2024\"") || content.contains("edition = \"2021\""),
        "Should specify Rust edition"
    );
}

#[test]
fn test_cargo_toml_has_core_dependencies() {
    let content = fs::read_to_string("Cargo.toml")
        .expect("Should be able to read Cargo.toml");
    
    let required_deps = vec![
        "axum",
        "tokio",
        "pulldown-cmark",
        "tower",
        "tower-http",
        "serde",
    ];
    
    for dep in required_deps {
        assert!(
            content.contains(dep),
            "Should have dependency '{}'",
            dep
        );
    }
}

#[test]
fn test_cargo_toml_tower_version() {
    let content = fs::read_to_string("Cargo.toml")
        .expect("Should be able to read Cargo.toml");
    
    // Check for tower version (the change in this PR)
    assert!(
        content.contains("tower = { version = \"0.5\""),
        "tower should be version 0.5"
    );
}

#[test]
fn test_cargo_toml_tower_http_version() {
    let content = fs::read_to_string("Cargo.toml")
        .expect("Should be able to read Cargo.toml");
    
    // Check for tower-http version (the change in this PR)
    assert!(
        content.contains("tower-http = { version = \"0.6\""),
        "tower-http should be version 0.6"
    );
}

#[test]
fn test_cargo_toml_has_dev_dependencies() {
    let content = fs::read_to_string("Cargo.toml")
        .expect("Should be able to read Cargo.toml");
    
    assert!(
        content.contains("[dev-dependencies]"),
        "Should have dev-dependencies section"
    );
    assert!(content.contains("tempfile"), "Should have tempfile for testing");
}

#[test]
fn test_cargo_toml_has_release_profile() {
    let content = fs::read_to_string("Cargo.toml")
        .expect("Should be able to read Cargo.toml");
    
    assert!(
        content.contains("[profile.release]"),
        "Should have release profile configuration"
    );
}

#[test]
fn test_cargo_toml_release_optimizations() {
    let content = fs::read_to_string("Cargo.toml")
        .expect("Should be able to read Cargo.toml");
    
    // Check for optimization settings
    if content.contains("[profile.release]") {
        assert!(content.contains("opt-level"), "Should specify optimization level");
        assert!(content.contains("lto"), "Should specify LTO settings");
    }
}

#[test]
fn test_cargo_toml_tokio_has_full_features() {
    let content = fs::read_to_string("Cargo.toml")
        .expect("Should be able to read Cargo.toml");
    
    // Tokio should have "full" features for async runtime
    assert!(
        content.contains("tokio") && content.contains("features") && content.contains("full"),
        "tokio should have 'full' features enabled"
    );
}

#[test]
fn test_cargo_toml_no_syntax_errors() {
    let content = fs::read_to_string("Cargo.toml")
        .expect("Should be able to read Cargo.toml");
    
    // Basic syntax checks
    let open_brackets = content.matches('[').count();
    let close_brackets = content.matches(']').count();
    assert_eq!(
        open_brackets, close_brackets,
        "Brackets should be balanced in TOML"
    );
    
    // Check for properly quoted strings
    let quotes = content.matches('"').count();
    assert!(
        quotes % 2 == 0,
        "Quotes should be balanced"
    );
}

// ============================================================================
// Cross-file Consistency Tests
// ============================================================================

#[test]
fn test_readme_and_contributing_are_consistent() {
    let readme = fs::read_to_string("README.md")
        .expect("Should be able to read README.md");
    let contributing = fs::read_to_string("CONTRIBUTING.md")
        .expect("Should be able to read CONTRIBUTING.md");
    
    // Both should mention the same commands
    let commands = vec!["cargo build", "cargo test", "cargo fmt", "cargo clippy"];
    
    for cmd in commands {
        let in_readme = readme.contains(cmd);
        let in_contributing = contributing.contains(cmd);
        
        assert!(
            in_readme && in_contributing,
            "Both README and CONTRIBUTING should mention '{}'",
            cmd
        );
    }
}

#[test]
fn test_cargo_package_name_matches_documentation() {
    let readme = fs::read_to_string("README.md")
        .expect("Should be able to read README.md");
    let cargo_toml = fs::read_to_string("Cargo.toml")
        .expect("Should be able to read Cargo.toml");
    
    // Package name in Cargo.toml should match references in README
    assert!(cargo_toml.contains("name = \"rsmd\""));
    assert!(readme.contains("rsmd"));
}

#[test]
fn test_documentation_mentions_same_server_port() {
    let readme = fs::read_to_string("README.md")
        .expect("Should be able to read README.md");
    let contributing = fs::read_to_string("CONTRIBUTING.md")
        .expect("Should be able to read CONTRIBUTING.md");
    
    // Both should mention port 3000
    assert!(readme.contains("3000"), "README should mention port 3000");
    assert!(contributing.contains("3000"), "CONTRIBUTING should mention port 3000");
}

#[test]
fn test_documentation_files_use_consistent_formatting() {
    let readme = fs::read_to_string("README.md")
        .expect("Should be able to read README.md");
    let contributing = fs::read_to_string("CONTRIBUTING.md")
        .expect("Should be able to read CONTRIBUTING.md");
    
    // Both should use consistent code block formatting
    assert!(readme.contains("```"), "README should have code blocks");
    assert!(contributing.contains("```"), "CONTRIBUTING should have code blocks");
    
    // Both should use consistent heading style (ATX-style with #)
    assert!(readme.contains("## "), "README should use ## for H2 headings");
    assert!(contributing.contains("## "), "CONTRIBUTING should use ## for H2 headings");
}

#[test]
fn test_contributing_commands_match_cargo_dependencies() {
    let contributing = fs::read_to_string("CONTRIBUTING.md")
        .expect("Should be able to read CONTRIBUTING.md");
    let cargo_toml = fs::read_to_string("Cargo.toml")
        .expect("Should be able to read Cargo.toml");
    
    // If CONTRIBUTING mentions certain tools, dependencies should support them
    if contributing.contains("cargo test") {
        assert!(
            cargo_toml.contains("[dev-dependencies]"),
            "Should have dev-dependencies for testing"
        );
    }
}

// ============================================================================
// Additional Quality Checks
// ============================================================================

#[test]
fn test_markdown_files_end_with_newline() {
    let files = vec!["README.md", "CONTRIBUTING.md"];
    
    for file in files {
        let content = fs::read_to_string(file)
            .unwrap_or_else(|_| panic!("Should be able to read {}", file));
        
        assert!(
            content.ends_with('\n'),
            "{} should end with a newline",
            file
        );
    }
}

#[test]
fn test_cargo_toml_ends_with_newline() {
    let content = fs::read_to_string("Cargo.toml")
        .expect("Should be able to read Cargo.toml");
    
    assert!(
        content.ends_with('\n'),
        "Cargo.toml should end with a newline"
    );
}

#[test]
fn test_no_hardcoded_absolute_paths() {
    let files = vec![
        ("README.md", fs::read_to_string("README.md").unwrap()),
        ("CONTRIBUTING.md", fs::read_to_string("CONTRIBUTING.md").unwrap()),
    ];
    
    for (filename, content) in files {
        // Check for suspicious absolute paths
        assert!(
            !content.contains("/home/") && !content.contains("C:\\"),
            "{} should not contain hardcoded absolute paths",
            filename
        );
    }
}

#[test]
fn test_documentation_uses_inclusive_language() {
    let files = vec![
        ("README.md", fs::read_to_string("README.md").unwrap()),
        ("CONTRIBUTING.md", fs::read_to_string("CONTRIBUTING.md").unwrap()),
    ];
    
    for (filename, content) in files {
        let lower_content = content.to_lowercase();
        
        // Check for non-inclusive terms (whitelist/blacklist should be allowlist/denylist)
        assert!(
            !lower_content.contains("whitelist") && !lower_content.contains("blacklist"),
            "{} should use inclusive language (allowlist/denylist instead of whitelist/blacklist)",
            filename
        );
    }
}

#[test]
fn test_cargo_toml_dependencies_have_versions() {
    let content = fs::read_to_string("Cargo.toml")
        .expect("Should be able to read Cargo.toml");
    
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
        if (in_dependencies || in_dev_dependencies) && !trimmed.is_empty() && !trimmed.starts_with('#') {
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

#[test]
fn test_contributing_has_numbered_sections() {
    let content = fs::read_to_string("CONTRIBUTING.md")
        .expect("Should be able to read CONTRIBUTING.md");
    
    // CONTRIBUTING.md should have clear numbered sections for better navigation
    let has_numbers = content.contains("## 1.") || content.contains("## 2.");
    
    assert!(
        has_numbers,
        "CONTRIBUTING.md should have numbered sections for clarity"
    );
}