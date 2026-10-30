//! Checks the contracts the core roles state, without running agents.

use std::collections::{BTreeSet, HashMap};

use crate::support::{
    AVOIDED_WORDS, Check, Failures, Report, captures, files, re, read, split_frontmatter,
};

/// The core role files, by name.
const ROLES: [&str; 5] = ["maestro", "scout", "planner", "reviewer", "worker"];
/// The models the core roles may name.
const CURRENT_MODELS: [&str; 3] = ["claude-haiku-5-5", "claude-sonnet-5-5", "claude-opus-5-5"];
/// The tools that run commands or change files.
const SHELL_AND_WRITE_TOOLS: [&str; 3] = ["bash", "edit", "write"];
/// A core role file: its frontmatter fields and its prompt body.
struct Role {
    /// The `key: value` lines of the frontmatter.
    fields: HashMap<String, String>,
    /// The prompt after the frontmatter.
    body: String,
}

impl Role {
    /// Returns the frontmatter value for `key`, or an empty string when absent.
    fn field(&self, key: &str) -> &str {
        self.fields.get(key).map_or("", String::as_str)
    }
}

/// Reads the core role `name`.
fn read_role(name: &str) -> Result<Role, Report> {
    let path = format!("manifests/core/agents/{name}.md");
    let text = read(&path)?;
    let (frontmatter, body) = split_frontmatter(&path, &text)?;
    let fields = frontmatter
        .lines()
        .filter_map(|line| line.split_once(": "))
        .map(|(key, value)| (key.to_owned(), value.to_owned()))
        .collect();
    Ok(Role {
        fields,
        body: body.to_owned(),
    })
}

/// Splits `text` after each sentence-ending `.`, `:`, `!` or `?` that whitespace follows.
fn sentences(text: &str) -> Result<Vec<&str>, Report> {
    let mut parts = Vec::new();
    let mut start = 0;
    for gap in re(r"\s+")?.find_iter(text) {
        if text[..gap.start()].ends_with(['.', ':', '!', '?']) {
            parts.push(&text[start..gap.start()]);
            start = gap.end();
        }
    }
    parts.push(&text[start..]);
    Ok(parts)
}

/// Keeps the stale model names out of every role and guide.
#[test]
fn roles_name_only_current_models() -> Check {
    let mut failures = Failures::default();
    for name in ROLES {
        let model = read_role(name)?.field("model").to_owned();
        failures.require(
            model.is_empty() || CURRENT_MODELS.contains(&model.as_str()),
            name,
            &format!("model {model} is not current"),
        );
    }
    let mut sources = vec!["README.md".to_owned()];
    for dir in ["manifests/core", "docs", "manifests/capabilities"] {
        sources.extend(files(dir, ".md")?);
    }

    for path in &sources {
        failures.avoids(path, &read(path)?, r"claude-[a-z]+-4-5")?;
    }

    failures.finish()
}

/// Holds the reviewer to read tools, with no shell to ask it to limit.
#[test]
fn reviewer_authority_comes_from_its_tool_list() -> Check {
    let mut failures = Failures::default();
    let reviewer = read_role("reviewer")?;

    failures.require_eq("reviewer", reviewer.field("tools"), "read, grep, find, ls");
    failures.avoids("reviewer", &reviewer.body, r"(?i)\b(bash|shell|git)\b")?;

    failures.finish()
}

/// Returns the sentences of `role` that name a shell or write tool the role holds.
fn sentences_naming_held_tools(role: &Role) -> Result<Vec<&str>, Report> {
    let tools = role.field("tools");
    let mut named = Vec::new();
    for tool in SHELL_AND_WRITE_TOOLS {
        if tools.is_empty() || tools.split(", ").any(|held| held == tool) {
            let names_tool = re(&format!(r"\b{tool}\b"))?;
            named.extend(
                sentences(&role.body)?
                    .into_iter()
                    .filter(|sentence| names_tool.is_match(sentence)),
            );
        }
    }
    Ok(named)
}

/// Rejects a prompt sentence that limits a shell or write tool the role has.
#[test]
fn no_role_prohibits_a_tool_it_holds() -> Check {
    let mut failures = Failures::default();
    for name in ROLES {
        let role = read_role(name)?;
        for sentence in sentences_naming_held_tools(&role)? {
            failures.avoids(name, sentence, r"(?i)\b(only|not|never|avoid)\b")?;
        }
    }
    failures.finish()
}

/// Requires each finding's location, cause and fix, and a pass per security rule.
#[test]
fn reviewer_findings_give_location_cause_fix_and_security_pass() -> Check {
    let mut failures = Failures::default();
    let body = read_role("reviewer")?.body;
    for required in ["path:line", "cause", "smallest fix", "not applicable"] {
        failures.contains("reviewer", &body, required);
    }

    let security_page = read("docs/standards/security.md")?;
    let named: BTreeSet<String> = captures(r"\b(SEC-\d{3})\b", &security_page)?
        .into_iter()
        .collect();
    let cited: BTreeSet<String> = captures(r"(SEC-\d{3})", &body)?.into_iter().collect();
    failures.require_eq("reviewer", &cited, &named);

    failures.finish()
}

/// Requires the check that means done and the tests to add first.
#[test]
fn planner_states_how_done_is_checked() -> Check {
    let mut failures = Failures::default();
    let body = read_role("planner")?.body;

    failures.contains("planner", &body, "Done when");
    failures.contains("planner", &body, "Tests first");

    failures.finish()
}

/// Requires the failing test commit first and an exit code for each command.
#[test]
fn worker_works_test_first_and_reports_exit_codes() -> Check {
    let mut failures = Failures::default();
    let body = read_role("worker")?.body;

    failures.matches("worker", &body, r"(?i)commit the failing test")?;
    failures.contains("worker", &body, "exit code");
    failures.matches("worker", &body, r"(?i)never claim")?;

    failures.finish()
}

/// Passes line ranges from the worker so the reviewer can read the change.
#[test]
fn worker_report_lets_a_reviewer_without_a_shell_find_the_change() -> Check {
    let mut failures = Failures::default();

    failures.contains("worker", &read_role("worker")?.body, "line range");
    failures.contains("reviewer", &read_role("reviewer")?.body, "Files changed");

    failures.finish()
}

/// Hands the reviewer the unified diff, with deletions, since it cannot run Git.
#[test]
fn worker_patch_lets_the_reviewer_read_deleted_lines() -> Check {
    let mut failures = Failures::default();
    let worker = read_role("worker")?.body;
    let reviewer = read_role("reviewer")?.body;

    failures.contains("worker", &worker, "## Patch");
    failures.matches("worker", &worker, r"(?i)unified diff")?;
    failures.matches("worker", &worker, r"(?i)deleted")?;
    failures.contains("reviewer", &reviewer, "Patch");
    failures.matches("reviewer", &reviewer, r"(?i)deleted")?;

    failures.finish()
}

/// Leads the persona with the result and the next action, and avoids hype words.
#[test]
fn roles_speak_in_maestro_voice() -> Check {
    let mut failures = Failures::default();
    for name in ROLES {
        let role = read_role(name)?;
        let text = format!("{}{}", role.field("description"), role.body);
        failures.avoids(name, &text, &format!(r"(?i)\b({AVOIDED_WORDS})\b"))?;
    }
    let persona = read_role("maestro")?.body;

    failures.matches("maestro", &persona, r"(?i)result")?;
    failures.matches("maestro", &persona, r"(?i)next action")?;

    failures.finish()
}
