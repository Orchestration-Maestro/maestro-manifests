//! Checks that each shared job of the test crate has one owner and no second copy.

use crate::support::{Check, Failures, files, read};

/// The file that owns the helpers every check module shares.
const SUPPORT: &str = "tests/support.rs";

/// The library file that owns reading, listing and splitting files under a root, for the scorecard and the checks.
const READING: &str = "src/reading.rs";

/// Records each source file but `owner` and this one that matches `pattern`.
fn only_in(owner: &str, pattern: &str) -> Check {
    let mut failures = Failures::default();
    let mut sources = files("tests", ".rs")?;
    sources.extend(files("src", ".rs")?);

    for path in sources
        .iter()
        .filter(|path| *path != owner && *path != "tests/one_owner.rs")
    {
        failures.avoids(&format!("{path} (owner {owner})"), &read(path)?, pattern)?;
    }

    failures.finish()
}

/// Cuts the text under a heading, and lists a page's headings, in one place.
#[test]
fn headings_are_split_only_in_support() -> Check {
    only_in(SUPPORT, r#"split(?:_once)?\("[^"]*## |\^## \("#)
}

/// Reads a role's frontmatter in one place.
#[test]
fn role_frontmatter_is_read_only_in_support() -> Check {
    only_in(
        SUPPORT,
        r#""(?:name|model|tools): |\^description: |split_once\(": "\)"#,
    )
}

/// Reads the string entries of a JSON array in one place.
#[test]
fn string_arrays_are_read_only_in_support() -> Check {
    only_in(SUPPORT, r"Value::as_array")
}

/// Reads, lists and parses files in one place, which the scorecard and the checks share.
#[test]
fn files_are_read_listed_and_parsed_only_in_reading() -> Check {
    only_in(READING, r"read_to_string|read_dir|serde_json::from_str")
}

/// Splits a Markdown file at its frontmatter in one place.
#[test]
fn frontmatter_is_split_only_in_reading() -> Check {
    only_in(READING, r#"strip_prefix\("---\\n"\)|splitn\(3, "---"#)
}

/// Collapses whitespace in one place.
#[test]
fn whitespace_is_collapsed_only_in_reading() -> Check {
    only_in(
        READING,
        r#"split_whitespace\(\)\.collect::<Vec<_>>\(\)\.join\(" "\)"#,
    )
}

/// Compiles patterns in one place.
#[test]
fn patterns_are_compiled_only_in_support() -> Check {
    only_in(SUPPORT, r"Regex::new")
}

/// Names the repository root in one place.
#[test]
fn repository_root_is_defined_once() -> Check {
    only_in("src/lib.rs", "CARGO_MANIFEST_DIR")
}
