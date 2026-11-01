//! Checks the public author guides without claiming runtime validation.

use crate::support::{Check, Failures, Sections, captures, files, re, read, strip_fences};

/// Vague requirements that need direct wording outside quoted examples.
const VAGUE_WORDING: &str = r"(?i)\bshould\b|when useful|as needed|ideally|recommended";

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
        !re(VAGUE_WORDING)?.is_match(&text),
        path,
        "uses a vague modal; state the requirement and its check",
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

/// Lists every guide in the index, the guard and scorecard pages included.
#[test]
fn index_links_every_guide() -> Check {
    let mut failures = Failures::default();
    let index = read("docs/index.md")?;

    for page in files("docs", ".md")? {
        let Some(link) = page
            .strip_prefix("docs/")
            .filter(|link| *link != "index.md")
        else {
            continue;
        };
        failures.require(
            index.contains(&format!("]({link})")) || index.contains(&format!("]({link}#")),
            "docs/index.md",
            &format!("does not link {link}"),
        );
    }

    failures.finish()
}
