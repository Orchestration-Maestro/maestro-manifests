//! Checks that each shared job of the test crate has one owner and no second copy.

use crate::support::{Check, Failures, files, read};

/// The file that owns the helpers every check module shares.
const SUPPORT: &str = "tests/support.rs";

/// The scorecard reads the release under a root it is given, which `tests/support.rs` cannot do for `src`.
const SCORECARD: &str = "src/scorecard.rs";

/// Records each source file but `owner` and this one that matches `pattern`.
fn only_in(owner: &str, pattern: &str) -> Check {
    only_in_except(owner, &[], pattern)
}

/// Records each source file but `owner`, the `exempt` files and this one that matches `pattern`.
fn only_in_except(owner: &str, exempt: &[&str], pattern: &str) -> Check {
    let mut failures = Failures::default();
    let mut sources = files("tests", ".rs")?;
    sources.extend(files("src", ".rs")?);

    for path in sources.iter().filter(|path| {
        *path != owner && *path != "tests/one_owner.rs" && !exempt.contains(&path.as_str())
    }) {
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

/// Reads files and compiles patterns in one place.
#[test]
fn files_are_read_and_patterns_compiled_only_in_support() -> Check {
    only_in_except(SUPPORT, &[SCORECARD], r"Regex::new|read_to_string|read_dir")
}

/// Names the repository root in one place.
#[test]
fn repository_root_is_defined_once() -> Check {
    only_in("src/lib.rs", "CARGO_MANIFEST_DIR")
}
