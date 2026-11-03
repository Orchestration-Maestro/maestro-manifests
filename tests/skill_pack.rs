//! Checks Maestro's development skill pack without loading it.

use std::collections::BTreeSet;

use crate::support::{
    Check, Failures, Report, captures, entries, exists, files, git, owned, parse_yaml, read,
    split_frontmatter,
};

/// The skill pack's folder in the manifest root.
const PACK: &str = "manifests/capabilities/practice/maestro-skills";
/// Each skill folder of the pack, with whether only a user may invoke it.
const SKILL_TYPES: [(&str, bool); 37] = [
    ("maestro-ask", true),
    ("maestro-code-review", false),
    ("maestro-codebase-design", false),
    ("maestro-diagnose", false),
    ("maestro-documentation", false),
    ("maestro-domain-modeling", false),
    ("maestro-frontend-ui", false),
    ("maestro-git-workflow", false),
    ("maestro-grilling", false),
    ("maestro-handoff", true),
    ("maestro-humanizer", false),
    ("maestro-idea-refine", true),
    ("maestro-implement", true),
    ("maestro-implement-spec", true),
    ("maestro-improve-codebase-architecture", true),
    ("maestro-observability", false),
    ("maestro-performance", false),
    ("maestro-pr", false),
    ("maestro-prototype", false),
    ("maestro-quality-gates", false),
    ("maestro-receive-review", false),
    ("maestro-research", false),
    ("maestro-retro", true),
    ("maestro-security", false),
    ("maestro-ship", false),
    ("maestro-show-options", false),
    ("maestro-tdd", false),
    ("maestro-teach", true),
    ("maestro-to-questionnaire", true),
    ("maestro-to-spec", true),
    ("maestro-to-tickets", true),
    ("maestro-triage", true),
    ("maestro-verify", false),
    ("maestro-wait-what", true),
    ("maestro-wayfinder", true),
    ("maestro-wizard", false),
    ("maestro-writing-for-agents", false),
];

/// The pack's skills folder.
fn skills() -> String {
    format!("{PACK}/skills")
}

/// Returns the pack's skill names.
fn names() -> Vec<&'static str> {
    SKILL_TYPES.iter().map(|(name, _)| *name).collect()
}

/// Returns the pack's skill folders, sorted.
fn folders() -> Result<Vec<String>, Report> {
    let mut found: Vec<String> = entries(&skills())?
        .into_iter()
        .filter(|(_, is_directory)| *is_directory)
        .filter_map(|(path, _)| path.rsplit_once('/').map(|(_, name)| name.to_owned()))
        .collect();
    found.sort();
    Ok(found)
}

/// Returns every `/skill:` target in `text`: the run of letters, digits and `-` after it.
fn targets(text: &str) -> BTreeSet<String> {
    text.split("/skill:")
        .skip(1)
        .map(|tail| {
            tail.split(|c: char| !(c.is_ascii_alphanumeric() || c == '-'))
                .next()
                .unwrap_or_default()
                .to_owned()
        })
        .filter(|name| !name.is_empty())
        .collect()
}

/// Whether `target`, an inline link in the pack file `from`, names no file outside the pack.
///
/// An in-page anchor, an address and a `mailto:` link need no file.
fn resolves(from: &str, target: &str) -> bool {
    if target.starts_with('#') || target.contains("://") || target.starts_with("mailto:") {
        return true;
    }
    let file = target.split('#').next().unwrap_or(target);
    let Some((base, _)) = from.rsplit_once('/') else {
        return false;
    };
    let mut parts: Vec<&str> = base.split('/').collect();
    for part in file.split('/') {
        match part {
            "" | "." => {}
            ".." => {
                if parts.pop().is_none() {
                    return false;
                }
            }
            other => parts.push(other),
        }
    }
    let resolved = parts.join("/");

    resolved.starts_with(&format!("{PACK}/")) && exists(&resolved)
}

/// Names every skill folder the pack holds, and gives each one its skill file.
#[test]
fn skill_pack_names_every_skill_folder() -> Check {
    let mut failures = Failures::default();
    let mut expected = owned(&names());
    expected.sort();

    failures.require_eq(&skills(), &folders()?, &expected);
    for name in names() {
        let path = format!("{}/{name}/SKILL.md", skills());
        failures.require(exists(&path), &path, "does not exist");
    }

    failures.finish()
}

/// Holds each skill's frontmatter to its folder, its description and its invocation rule.
#[test]
fn skill_frontmatter_matches_its_folder() -> Check {
    let mut failures = Failures::default();
    let mut type_only = 0;

    for (name, only_a_user) in SKILL_TYPES {
        let path = format!("{}/{name}/SKILL.md", skills());
        let text = read(&path)?;
        let (frontmatter, _) = split_frontmatter(&path, &text)?;
        let Some(document) = parse_yaml(&path, frontmatter)?.into_iter().next() else {
            failures.require(false, &path, "has no frontmatter document");
            continue;
        };
        let described = document["description"].as_str().unwrap_or_default();
        let held = document["disable-model-invocation"]
            .as_bool()
            .unwrap_or(false);

        failures.require_eq(&path, &document["name"].as_str(), &Some(name));
        failures.require(
            described.starts_with("Use when"),
            &path,
            "description does not start with `Use when`",
        );
        failures.require(
            described.chars().count() < 500,
            &path,
            "description is 500 characters or longer",
        );
        failures.require_eq(&path, &held, &only_a_user);
        type_only += usize::from(held);
    }

    failures.require_eq("type-only skills", &type_only, &14);
    failures.require_eq(
        "model-invoked skills",
        &(SKILL_TYPES.len() - type_only),
        &23,
    );

    failures.finish()
}

/// Points the router at every other skill in the pack, and at nothing else.
#[test]
fn router_names_every_other_skill() -> Check {
    let mut failures = Failures::default();
    let router = format!("{}/{}/SKILL.md", skills(), "maestro-ask");
    let found = targets(&read(&router)?);
    let expected: BTreeSet<String> = names()
        .into_iter()
        .filter(|name| *name != "maestro-ask")
        .map(str::to_owned)
        .collect();

    failures.require_eq(&router, &found, &expected);

    failures.finish()
}

/// Keeps every skill pointer and every inline link inside the pack.
#[test]
fn skill_pointers_and_links_resolve_inside_the_pack() -> Check {
    let mut failures = Failures::default();
    let known: BTreeSet<&str> = names().into_iter().collect();

    for path in files(PACK, ".md")? {
        let text = read(&path)?;
        for name in targets(&text) {
            failures.require(
                known.contains(name.as_str()),
                &path,
                &format!("names unknown skill `{name}`"),
            );
        }
        for target in captures(r"\]\(([^)]*)\)", &text)? {
            failures.require(
                resolves(&path, &target),
                &path,
                &format!("link `{target}` does not resolve inside the pack"),
            );
        }
    }

    failures.finish()
}

/// Keeps every file under the skills folder inside a skill folder, with no symbolic link.
#[test]
fn skill_pack_holds_no_loose_document() -> Check {
    let mut failures = Failures::default();
    let known: BTreeSet<&str> = names().into_iter().collect();
    let prefix = format!("{}/", skills());
    let listing = git(&["ls-files", "-s", PACK])?;

    for path in files(&skills(), "")? {
        let relative = path.strip_prefix(&prefix).unwrap_or_default();
        let mut segments = relative.split('/');
        let inside =
            segments.next().is_some_and(|name| known.contains(name)) && segments.next().is_some();
        failures.require(inside, &path, "lies outside every skill folder");
    }
    failures.require(
        listing.code == Some(0),
        "git ls-files",
        &format!("exits {:?}: {}", listing.code, listing.stderr),
    );
    for line in listing.stdout.lines() {
        failures.require(
            !line.starts_with("120000"),
            line,
            "is a symbolic link inside the pack",
        );
    }

    failures.finish()
}

/// Shows the pointer reader and the link resolver accepting qualified content and rejecting broken content.
#[test]
fn skill_pack_checks_reject_broken_controls() -> Check {
    let mut failures = Failures::default();
    let router = format!("{}/{}/SKILL.md", skills(), "maestro-ask");

    failures.require_eq(
        "targets",
        &targets("/skill:maestro-ask, /skill:maestro-ask args."),
        &BTreeSet::from(["maestro-ask".to_owned()]),
    );
    failures.require(
        targets("/lean /focus security-audit").is_empty(),
        "targets",
        "finds a skill in prose without a pointer",
    );
    for broken in ["missing.md", "../../../../Cargo.toml"] {
        failures.require(
            !resolves(&router, broken),
            broken,
            "resolves inside the pack",
        );
    }
    for qualified in ["SKILL.md#flow", "#flow", "https://example.com/x"] {
        failures.require(resolves(&router, qualified), qualified, "does not resolve");
    }

    failures.finish()
}
