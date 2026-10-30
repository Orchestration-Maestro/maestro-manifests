//! Checks the public author guides without claiming runtime validation.

use crate::support::{Check, Failures, Sections, captures, files, re, read, strip_fences};

/// Records every way the guide at `path` fails to show its sections and checks.
fn check_guide(failures: &mut Failures, path: &str) -> Check {
    let text = strip_fences(&read(path)?)?;
    let sections = Sections::split(&text, "##")?;
    let opens_with_contents = re(r"(?s)# [^\n]+\n\n[^#\n].*?\n\n## Contents\n")?;
    failures.require(
        opens_with_contents.is_match(&text),
        path,
        "no title, purpose paragraph and `## Contents` in that order",
    );
    if let Some(contents) = sections.get("Contents") {
        let others: Vec<String> = sections
            .titles()
            .into_iter()
            .filter(|name| *name != "Contents")
            .map(str::to_owned)
            .collect();
        failures.require_eq(
            path,
            &captures(r"(?m)^\d+\. \[([^]]+)\]", contents)?,
            &others,
        );
    }
    for required in ["What the organization can lock", "Check a change"] {
        failures.require(
            sections.get(required).is_some(),
            path,
            &format!("no section `{required}`"),
        );
    }
    failures.require(
        !re(r"\bshould\b|when useful")?.is_match(&text),
        path,
        "uses `should` or `when useful`",
    );
    Ok(())
}

/// Shows every section and distinguishes requirements from optional guidance.
#[test]
fn guides_have_contents_constraints_and_change_checks() -> Check {
    let mut failures = Failures::default();
    let mut pages = vec!["README.md".to_owned()];
    pages.extend(files("docs", ".md")?);

    for page in &pages {
        check_guide(&mut failures, page)?;
    }

    failures.finish()
}
