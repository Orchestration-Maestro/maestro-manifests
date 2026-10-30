//! Checks that the baseline policy covers the core release content and that its guide lists it.

use crate::policy::Baseline;
use crate::support::{Check, Failures, Report, exists, files, read, read_role, strings};

/// Returns the core role stems, from the role files.
fn core_roles() -> Result<Vec<String>, Report> {
    let paths = files("manifests/core/agents", ".md")?;
    Ok(paths
        .iter()
        .filter_map(|path| path.rsplit('/').next()?.strip_suffix(".md"))
        .map(str::to_owned)
        .collect())
}

/// Requires every core role and workflow, and only paths the release holds.
#[test]
fn core_roles_and_workflows_are_required() -> Check {
    let mut failures = Failures::default();
    let baseline = Baseline::load()?;
    let required = strings(&baseline.policy, "required")?;
    let mut resources = files("manifests/core/agents", ".md")?;
    resources.extend(files("manifests/core/workflows", ".yaml")?);

    failures.require(!required.is_empty(), "required", "is empty");
    for path in &required {
        failures.require(
            exists(&format!("manifests/{path}")),
            path,
            "required path is missing",
        );
    }
    for resource in resources {
        let relative = resource.trim_start_matches("manifests/");
        let covered = required
            .iter()
            .any(|path| relative == *path || relative.starts_with(&format!("{path}/")));
        failures.require(covered, relative, "is under no required path");
    }

    failures.finish()
}

/// Fixes what each core role runs on and may use.
#[test]
fn every_core_role_locks_model_thinking_and_tools() -> Check {
    let mut failures = Failures::default();
    let baseline = Baseline::load()?;
    let roles = baseline.roles();
    let mut core = core_roles()?;
    core.sort();

    let mut names: Vec<String> = roles.keys().cloned().collect();
    names.sort();

    failures.require_eq("roles", &names, &core);
    for (name, role) in &roles {
        let mut locked = strings(role, "locked")?;
        locked.sort_unstable();
        failures.require_eq(name, &locked, &vec!["model", "thinking", "tools"]);
    }

    failures.finish()
}

/// Allows each model a core role declares, since its model lock fixes that choice.
#[test]
fn every_model_a_core_role_declares_is_allowed() -> Check {
    let mut failures = Failures::default();
    let baseline = Baseline::load()?;
    let models = strings(&baseline.policy, "models")?;

    for name in core_roles()? {
        if let Some(model) = read_role(&name)?.get("model") {
            failures.require(
                models.contains(&model),
                &format!("manifests/core/agents/{name}.md"),
                &format!("declares {model}, which the policy does not allow"),
            );
        }
    }

    failures.finish()
}

/// Quotes both files and names every lock, model and required path in the guide.
#[test]
fn policy_guide_lists_the_baseline() -> Check {
    let mut failures = Failures::default();
    let baseline = Baseline::load()?;
    let guide = read("docs/policy.md")?;
    let baseline_section = guide
        .split("## The baseline")
        .nth(1)
        .and_then(|rest| rest.split("\n## ").next());
    let bounds = baseline.bounds();
    let roles = baseline.roles();
    let mut listed = strings(&baseline.policy, "locked")?;
    listed.extend(strings(&baseline.policy, "models")?);
    listed.extend(strings(&baseline.policy, "required")?);
    listed.extend(bounds.keys().map(String::as_str));
    listed.extend(roles.keys().map(String::as_str));

    failures.quoted(
        "policy.json",
        "docs/policy.md",
        "json",
        &read("manifests/settings/policy.json")?,
    )?;
    failures.quoted(
        "settings.json",
        "docs/settings.md",
        "json",
        &read("manifests/settings/settings.json")?,
    )?;
    failures.require(!listed.is_empty(), "policy.json", "names nothing");
    match baseline_section {
        Some(section) => {
            for name in listed {
                failures.contains("docs/policy.md baseline", section, &format!("`{name}`"));
            }
        }
        None => failures.require(false, "docs/policy.md", "no `## The baseline` section"),
    }

    failures.finish()
}
