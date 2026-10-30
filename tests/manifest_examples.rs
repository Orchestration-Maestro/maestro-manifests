//! Checks the core role and workflow examples without running agents.

use crate::support::{Check, Failures, agents, captures, owned, read, split_frontmatter, tasks};

/// What a core role's file must keep through prose edits.
struct Contract {
    /// The role's file stem and `name`.
    role: &'static str,
    /// The `tools` line, or `None` when the role holds every tool.
    tools: Option<&'static str>,
    /// The `model` line.
    model: &'static str,
    /// The `##` headings of the prompt, in order.
    headings: &'static [&'static str],
    /// A phrase from the earlier wording that must be gone.
    former_intro: &'static str,
}

/// Records every way the role file for `contract` departs from it.
fn check_role(failures: &mut Failures, contract: &Contract) -> Check {
    let path = format!("manifests/core/agents/{}.md", contract.role);
    let text = read(&path)?;
    let (frontmatter, body) = split_frontmatter(&path, &text)?;

    failures.contains(&path, frontmatter, &format!("name: {}\n", contract.role));
    failures.contains(&path, frontmatter, &format!("model: {}\n", contract.model));
    match contract.tools {
        Some(tools) => failures.contains(&path, frontmatter, &format!("tools: {tools}\n")),
        None => failures.require(!frontmatter.contains("tools:"), &path, "has a `tools` line"),
    }
    failures.require_eq(
        &path,
        &captures(r"(?m)^## (.+)$", body)?,
        &owned(contract.headings),
    );
    failures.require(
        !body.contains(contract.former_intro),
        &path,
        "keeps its former intro",
    );
    failures.require(
        !body.contains("path/to/file.ts"),
        &path,
        "keeps the old file example",
    );
    failures.require(
        !body.contains("```typescript"),
        &path,
        "keeps a typescript block",
    );
    Ok(())
}

/// Preserves permissions, models and output sections during prose edits.
#[test]
fn core_roles_keep_contracts_with_maestro_wording() -> Check {
    let mut failures = Failures::default();
    let contracts = [
        Contract {
            role: "scout",
            tools: Some("read, grep, find, ls, bash"),
            model: "claude-haiku-5-5",
            headings: &["Files retrieved", "Key code", "Architecture", "Start here"],
            former_intro: "Quickly investigate a codebase",
        },
        Contract {
            role: "planner",
            tools: Some("read, grep, find, ls"),
            model: "claude-opus-5-5",
            headings: &[
                "Goal",
                "Plan",
                "Files to modify",
                "New files (if any)",
                "Verification",
                "Risks",
            ],
            former_intro: "You are a planning specialist",
        },
        Contract {
            role: "reviewer",
            tools: Some("read, grep, find, ls"),
            model: "claude-opus-5-5",
            headings: &[
                "Files reviewed",
                "Critical (blocks)",
                "Important (blocks)",
                "Minor (fix in this change)",
                "Security",
                "Summary",
            ],
            former_intro: "You are a senior code reviewer",
        },
        Contract {
            role: "worker",
            tools: None,
            model: "claude-sonnet-5-5",
            headings: &[
                "Completed",
                "Files changed",
                "Patch",
                "Commands run",
                "Notes (if any)",
            ],
            former_intro: "without polluting the main conversation",
        },
    ];

    for contract in &contracts {
        check_role(&mut failures, contract)?;
    }

    failures.finish()
}

/// Records every way the workflow `name` departs from its steps and handoffs.
fn check_workflow(
    failures: &mut Failures,
    name: &str,
    roles: &[&str],
    original: &[usize],
) -> Check {
    let path = format!("manifests/core/workflows/{name}.yaml");
    let text = read(&path)?;
    failures.contains(&path, &text, "apiVersion: maestro.workflow/v1\n");
    failures.contains(&path, &text, &format!("kind: Workflow\nname: {name}\n"));
    failures.require_eq(&path, &agents(&text)?, &owned(roles));
    let tasks = tasks(&text)?;
    failures.require_eq(&path, &tasks.len(), &roles.len());
    for (index, task) in tasks.iter().enumerate() {
        failures.require_eq(
            &path,
            &task.matches("$@").count(),
            &usize::from(original.contains(&index)),
        );
        failures.require_eq(
            &path,
            &task.matches("{previous}").count(),
            &usize::from(index > 0),
        );
    }
    failures.lacks(&path, &text, "task: \"Implement: $@\"");
    failures.lacks(&path, &text, "Find all code relevant to:");
    if name == "scout-and-plan" {
        failures.contains(
            &path,
            tasks.last().map_or("", String::as_str),
            "Do not implement",
        );
    }
    if name == "implement-and-review" {
        for guide in ["workflows", "schemas-and-checks", "catalog"] {
            failures.quoted(&path, &format!("docs/{guide}.md"), "yaml", &text)?;
        }
    }
    Ok(())
}

/// Retains argument and previous-output passing in all three chains.
#[test]
fn core_workflows_keep_steps_and_handoffs_with_maestro_tasks() -> Check {
    let mut failures = Failures::default();

    check_workflow(
        &mut failures,
        "implement-and-review",
        &["worker", "reviewer", "worker"],
        &[0],
    )?;
    check_workflow(
        &mut failures,
        "implement",
        &["scout", "planner", "worker"],
        &[0, 1],
    )?;
    check_workflow(
        &mut failures,
        "scout-and-plan",
        &["scout", "planner"],
        &[0, 1],
    )?;

    failures.finish()
}
