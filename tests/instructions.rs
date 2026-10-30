//! Checks that instruction and configuration files state only what holds.

use regex::Regex;

use crate::support::{Check, Failures, Report, Sections, captures, re, read, tracked_files};

/// Expands the first `{a,b}` group the way `EditorConfig` reads it, recursively.
fn expand_braces(pattern: &str) -> Result<Vec<String>, Report> {
    let Some(group) = re(r"\{([^{}]*)\}")?.find(pattern) else {
        return Ok(vec![pattern.to_owned()]);
    };
    let options = &pattern[group.start() + 1..group.end() - 1];
    let mut expanded = Vec::new();
    for option in options.split(',') {
        let chosen = format!(
            "{}{option}{}",
            &pattern[..group.start()],
            &pattern[group.end()..]
        );
        expanded.extend(expand_braces(&chosen)?);
    }
    Ok(expanded)
}

/// Translates an `EditorConfig` glob into a regex that matches a whole file name.
///
/// Only `*` and `?` are wildcards; a `[` class is matched literally, so a section
/// that needs one matches no file and fails the check.
fn glob_regex(glob: &str) -> Result<Regex, Report> {
    let body: String = glob
        .chars()
        .map(|c| match c {
            '*' => ".*".to_owned(),
            '?' => ".".to_owned(),
            other => regex::escape(&other.to_string()),
        })
        .collect();
    re(&format!("^{body}$"))
}

/// Finds no claim that CI notices a skipped `just setup`.
#[test]
fn setup_is_not_detected_by_the_required_workflow() -> Check {
    let mut failures = Failures::default();
    let text = read("AGENTS.md")?;

    match Sections::split(&text, "##")?.get("Setup") {
        Some(setup) => failures.avoids("AGENTS.md", setup, r"(?i)required\s+workflow")?,
        None => failures.require(false, "AGENTS.md", "no `## Setup` section"),
    }

    failures.finish()
}

/// Finds `gitleaks` attributed to the commit hook, not CI.
#[test]
fn secret_scan_is_not_attributed_to_ci() -> Check {
    let mut failures = Failures::default();
    let path = "docs/standards/engineering.md";
    let text = read(path)?;

    match Sections::split(&text, "##")?.get("Pin inputs and keep secrets out") {
        Some(section) => {
            failures.contains(
                path,
                section,
                "The hook `gitleaks` scans each staged change at commit.",
            );
            failures.matches(
                path,
                section,
                r"It is a local hook and no\s+CI job runs it\.",
            )?;
        }
        None => failures.require(false, path, "no `Pin inputs and keep secrets out` section"),
    }

    failures.finish()
}

/// Finds no section for a kind of file the repository does not hold.
#[test]
fn every_editorconfig_section_matches_a_tracked_file() -> Check {
    let mut failures = Failures::default();
    let sections = captures(r"(?m)^\[(.+)\]$", &read(".editorconfig")?)?;
    let names: Vec<String> = tracked_files()?
        .iter()
        .map(|path| path.rsplit('/').next().unwrap_or(path).to_owned())
        .collect();

    for section in &sections {
        let mut matched = false;
        for pattern in expand_braces(section.trim_start_matches('/'))? {
            let glob = glob_regex(&pattern)?;
            matched |= names.iter().any(|name| glob.is_match(name));
        }
        failures.require(matched, section, "matches no tracked file");
    }

    failures.finish()
}
