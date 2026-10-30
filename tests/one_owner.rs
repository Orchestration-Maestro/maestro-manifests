//! Checks that each shared job of the test crate has one owner and no second copy.

use crate::support::{Check, Failures, files, read};

/// The file that owns the helpers every check module shares.
const SUPPORT: &str = "tests/support.rs";

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

/// Cuts the text under a heading in one place.
#[test]
fn headings_are_split_only_in_support() -> Check {
    only_in(SUPPORT, r#"split(?:_once)?\("[^"]*## "#)
}

/// Reads a core role's frontmatter in one place.
#[test]
fn role_frontmatter_is_read_only_in_support() -> Check {
    only_in(SUPPORT, r#""model: "|split_once\(": "\)"#)
}

/// Reads the string entries of a JSON array in one place.
#[test]
fn string_arrays_are_read_only_in_support() -> Check {
    only_in(SUPPORT, r"Value::as_array")
}

/// Reads files and compiles patterns in one place.
#[test]
fn files_are_read_and_patterns_compiled_only_in_support() -> Check {
    only_in(SUPPORT, r"Regex::new|read_to_string|read_dir")
}

/// Names the repository root in one place.
#[test]
fn repository_root_is_defined_once() -> Check {
    only_in("src/lib.rs", "CARGO_MANIFEST_DIR")
}
