//! Checks that the manifest root holds every manifest artifact and nothing else.

use std::collections::BTreeSet;

use crate::support::{
    Check, Failures, Report, captures, entries, exists, files, json, pairs, re, read, strip_fences,
    tracked_files,
};

/// The area folders that live under the manifest root.
const AREAS: [&str; 5] = ["settings", "core", "standards", "languages", "capabilities"];
/// The files that live directly in the manifest root.
const COMMON_FILES: [&str; 2] = ["CHANGELOG.md", "package.json"];

/// Whether `name` is an area folder or a common file, which belong under `manifests/`.
fn is_manifest_artifact(name: &str) -> bool {
    AREAS.contains(&name) || COMMON_FILES.contains(&name)
}

/// Returns the tracked paths inside the manifest root, relative to the repository.
fn under_manifest_root() -> Result<Vec<String>, Report> {
    Ok(tracked_files()?
        .into_iter()
        .filter(|name| name.starts_with("manifests/"))
        .collect())
}

/// Returns the first path segment of `path`.
fn first_segment(path: &str) -> &str {
    path.split('/').next().unwrap_or(path)
}

/// Resolves `target` against the directory `base`, dropping `.` and `..` segments.
///
/// Returns `None` when the result would leave the repository.
fn resolve(base: &str, target: &str) -> Option<String> {
    let mut parts: Vec<&str> = base.split('/').filter(|part| !part.is_empty()).collect();
    for part in target.split('/') {
        match part {
            "" | "." => {}
            ".." => {
                parts.pop()?;
            }
            other => parts.push(other),
        }
    }
    Some(parts.join("/"))
}

/// Finds no area folder, changelog or package record beside the tooling.
#[test]
fn no_artifact_remains_outside_the_manifest_root() -> Check {
    let mut failures = Failures::default();
    let outside: Vec<String> = tracked_files()?
        .into_iter()
        .filter(|name| is_manifest_artifact(first_segment(name)))
        .collect();

    failures.require_eq("tracked files", &outside, &Vec::new());
    for name in [
        "CHANGELOG.md",
        "package.json",
        "settings/settings.json",
        "settings/policy.json",
        "core/SYSTEM.md",
    ] {
        let path = format!("manifests/{name}");
        failures.require(read(&path).is_ok(), &path, "is not a file");
    }

    failures.finish()
}

/// Holds the root to the common files and the area folders.
#[test]
fn nothing_in_the_manifest_root_is_repository_tooling() -> Check {
    let mut failures = Failures::default();
    let names: BTreeSet<String> = under_manifest_root()?
        .iter()
        .filter_map(|path| path.split('/').nth(1))
        .map(str::to_owned)
        .collect();

    failures.require(!names.is_empty(), "manifests/", "holds no tracked file");
    for name in names.iter().filter(|name| !is_manifest_artifact(name)) {
        failures.require(
            false,
            "manifests/",
            &format!("holds `{name}`, which is not an area or common file"),
        );
    }

    failures.finish()
}

/// Gives every group folder and capability folder its owner, by repository path.
#[test]
fn codeowners_names_each_group_and_capability_under_the_manifest_root() -> Check {
    let mut failures = Failures::default();
    let owners = read(".github/CODEOWNERS")?;
    let patterns: Vec<&str> = owners
        .lines()
        .filter(|line| !line.trim().is_empty() && !line.starts_with('#'))
        .filter_map(|line| line.split_whitespace().next())
        .collect();
    let mut folders = Vec::new();
    for (group, _) in entries("manifests/capabilities")? {
        folders.extend(
            entries(&group)?
                .into_iter()
                .map(|(capability, _)| capability),
        );
        folders.push(group);
    }

    for pattern in patterns
        .iter()
        .filter(|pattern| **pattern != "*" && !pattern.starts_with("/manifests/"))
    {
        failures.require(
            false,
            ".github/CODEOWNERS",
            &format!("`{pattern}` is outside the manifest root"),
        );
    }
    failures.require(
        !folders.is_empty(),
        "manifests/capabilities",
        "holds no folder",
    );
    for folder in &folders {
        failures.require(
            patterns.contains(&format!("/{folder}/").as_str()),
            folder,
            "has no CODEOWNERS entry",
        );
    }

    failures.finish()
}

/// Points the release at the changelog Maestro reads, and quotes that in the guide.
#[test]
fn release_please_writes_the_changelog_in_the_manifest_root() -> Check {
    let mut failures = Failures::default();
    let path = ".github/release-please/config.json";
    let config = json(path)?;
    let changelog = config["packages"]["."]["changelog-path"]
        .as_str()
        .unwrap_or_default();

    failures.require_eq(path, changelog, "manifests/CHANGELOG.md");
    failures.require(exists(changelog), changelog, "does not exist");
    failures.quoted(path, "docs/releases-and-updates.md", "json", &read(path)?)?;

    failures.finish()
}

/// Excludes only the manifest root's changelog from the Markdown check.
#[test]
fn content_check_skips_the_generated_changelog_at_its_new_path() -> Check {
    let mut failures = Failures::default();
    let path = ".pre-commit-config.yaml";
    let excludes = captures(r"(?s)id: rumdl\n.*?exclude: '([^']+)'", &read(path)?)?;

    failures.require_eq(path, &excludes.len(), &1);
    if let Some(exclude) = excludes.first() {
        let exclude = re(exclude)?;
        failures.require(
            exclude.is_match("manifests/CHANGELOG.md"),
            path,
            "does not skip manifests/CHANGELOG.md",
        );
        failures.require(!exclude.is_match("README.md"), path, "skips README.md");
        failures.require(
            !exclude.is_match("docs/releases-and-updates.md"),
            path,
            "skips docs/releases-and-updates.md",
        );
    }

    failures.finish()
}

/// Indexes each file in the root by the path that holds it.
#[test]
fn readme_index_links_every_manifest_file() -> Check {
    let mut failures = Failures::default();
    let readme = read("README.md")?;
    let tracked = under_manifest_root()?;

    failures.require(!tracked.is_empty(), "manifests/", "holds no tracked file");
    for name in &tracked {
        failures.contains(name, &readme, &format!("[`{name}`]({name})"));
    }

    failures.finish()
}

/// Records every link in `guide` that does not resolve to a file in its place.
fn check_links(failures: &mut Failures, guide: &str) -> Check {
    let text = strip_fences(&read(guide)?)?;
    let base = guide
        .rsplit_once('/')
        .map_or("", |(directory, _)| directory);
    let scheme = re(r"^[a-z][a-z0-9+.-]*:")?;

    for target in captures(r"\]\(([^)\s#]+)(?:#[^)\s]*)?\)", &text)? {
        if scheme.is_match(&target) {
            continue;
        }
        let subject = format!("{guide} -> {target}");
        let Some(resolved) = resolve(base, &target) else {
            failures.require(false, &subject, "leaves the repository");
            continue;
        };
        failures.require(exists(&resolved), &subject, "does not exist");
        failures.require(
            !is_manifest_artifact(first_segment(&resolved)),
            &subject,
            "points to a moved location",
        );
    }
    for (caption, target) in pairs(r"(?m)^From \[`([^`]+)`\]\(([^):]+)\):$", &text)? {
        let resolved = resolve(base, &target);
        failures.require(
            resolved.as_deref() == Some(caption.as_str()),
            &format!("{guide} caption {caption}"),
            "does not name the linked file",
        );
    }
    Ok(())
}

/// Resolves every link and caption to a file, and none to a moved location.
#[test]
fn documentation_links_resolve_into_the_manifest_root() -> Check {
    let mut failures = Failures::default();
    let mut guides: Vec<String> = [
        "README.md",
        "CONTEXT.md",
        "AGENTS.md",
        ".github/copilot-instructions.md",
    ]
    .map(str::to_owned)
    .into();
    guides.extend(files("docs", ".md")?);

    for guide in &guides {
        check_links(&mut failures, guide)?;
    }

    failures.finish()
}
