//! Checks the opt-in evidence-review capability: its role, skill, workflow, instruction and theme.

use std::collections::BTreeMap;
use std::path::Path;

use serde_json::{Value, json};

use crate::support::{
    AVOIDED_WORDS, Check, Failures, Report, Role, agents, files, owned, re, read, read_json,
    sentence_count, split_frontmatter, tasks,
};

/// The evidence-review capability folder.
const CAPABILITY: &str = "manifests/capabilities/practice/evidence-review";
/// The evidence-review skill file.
const SKILL: &str =
    "manifests/capabilities/practice/evidence-review/skills/review-evidence/SKILL.md";
/// The evidence-review role file.
const ROLE: &str = "manifests/capabilities/practice/evidence-review/agents/evidence-reviewer.md";
/// The evidence-review workflow file.
const WORKFLOW: &str =
    "manifests/capabilities/practice/evidence-review/workflows/review-evidence.yaml";
/// The rule table's rows after its header and separator.
const RULES: [&str; 6] = [
    "Cite",
    "Classify",
    "Report runs",
    "Report gaps",
    "Source honesty",
    "Private evidence",
];

/// The rows of the table in `markdown`, header first, as stripped cells.
fn table_rows(markdown: &str) -> Result<Vec<Vec<String>>, Report> {
    Ok(re(r"(?m)^\|.+\|$")?
        .find_iter(markdown)
        .map(|row| {
            row.as_str()
                .trim_matches('|')
                .split('|')
                .map(|cell| cell.trim().to_owned())
                .collect()
        })
        .collect())
}

/// Returns the column `column` of each rule row, keyed by the rule name.
fn rule_column(skill: &str, column: usize) -> Result<BTreeMap<String, String>, Report> {
    let rows = table_rows(skill)?;
    Ok(rows
        .iter()
        .skip(2)
        .map(|row| {
            (
                row.first().cloned().unwrap_or_default(),
                row.get(column).cloned().unwrap_or_default(),
            )
        })
        .collect())
}

/// Offers a read-only capability workflow without selecting the area.
#[test]
fn evidence_review_workflow_is_opt_in_and_indexed() -> Check {
    let mut failures = Failures::default();
    let text = read(WORKFLOW)?;
    let tasks = tasks(&text)?;
    let link = format!("[`{WORKFLOW}`]({WORKFLOW})");
    let settings = read_json("manifests/settings/settings.json")?;
    let role = Role::parse(ROLE, &read(ROLE)?)?;

    failures.contains(WORKFLOW, &text, "apiVersion: maestro.workflow/v1\n");
    failures.contains(WORKFLOW, &text, "kind: Workflow\nname: review-evidence\n");
    failures.require_eq(WORKFLOW, &agents(&text)?, &owned(&["evidence-reviewer"]));
    failures.require_eq(WORKFLOW, &tasks.len(), &1);
    failures.contains(WORKFLOW, tasks.first().map_or("", String::as_str), "$@");
    failures.lacks(
        WORKFLOW,
        tasks.first().map_or("", String::as_str),
        "{previous}",
    );
    failures.contains("README.md", &read("README.md")?, &link);
    failures.quoted(WORKFLOW, "docs/workflows.md", "yaml", &text)?;
    failures.require_eq(
        "settings.json",
        &settings.get("manifestAreas"),
        &Some(&json!([])),
    );
    failures.require(
        settings.get("theme").is_none(),
        "settings.json",
        "sets a theme",
    );
    failures.require_eq(ROLE, &role.get("tools"), &Some("read, grep, find, ls"));

    failures.finish()
}

/// Keeps the review rule out of sessions that do not review evidence.
#[test]
fn evidence_review_rule_applies_only_to_review_work() -> Check {
    let mut failures = Failures::default();

    for path in files(CAPABILITY, ".instructions.md")? {
        failures.avoids(&path, &read(&path)?, r#"(?m)^applyTo: "?\*\*"?$"#)?;
    }

    failures.finish()
}

/// Keeps one installable instruction, indexed and quoted, with no exception.
#[test]
fn shipped_instruction_is_a_real_example_scoped_to_docs() -> Check {
    let mut failures = Failures::default();
    let path = format!("{CAPABILITY}/instructions/cite-code-claims.instructions.md");
    let security = "manifests/capabilities/security/maestro-security/instructions/security-assessment.instructions.md";
    let text = read(&path)?;
    let index = read("docs/index.md")?;

    let mut expected = vec![path.clone(), security.to_owned()];
    expected.sort();

    failures.require_eq(
        "*.instructions.md",
        &files("", ".instructions.md")?,
        &expected,
    );
    failures.require(
        text.starts_with("---\napplyTo: \"docs/**/*.md\"\n---\n"),
        &path,
        "is not scoped to docs",
    );
    failures.contains(
        "README.md",
        &read("README.md")?,
        &format!("[`{path}`]({path})"),
    );
    for guide in ["instructions", "integrity"] {
        failures.quoted(&path, &format!("docs/{guide}.md"), "markdown", &text)?;
    }
    failures.lacks("docs/index.md", &index, "*.instructions.md");
    failures.contains(
        "docs/index.md",
        &index,
        "skill, instruction, prompt, theme and workflow",
    );

    failures.finish()
}

/// Holds the role and skill to one report whose parts a reviewer can check.
#[test]
fn evidence_review_rules_are_checkable_and_map_to_truthful_evidence() -> Check {
    let mut failures = Failures::default();
    let terms = [
        "Corrected answer",
        "Findings",
        "Checks run",
        "Not run",
        "path:line",
        "no source",
        "Verified failure",
        "Hypothesis",
    ];
    for (path, guide) in [(ROLE, "roles"), (SKILL, "skills")] {
        let text = read(path)?;
        failures.contains(path, &text, "SEC-008");
        for term in terms {
            failures.contains(path, &text, &format!("`{term}`"));
        }
        failures.quoted(path, &format!("docs/{guide}.md"), "markdown", &text)?;
    }

    let rows = table_rows(&read(SKILL)?)?;
    failures.require_eq(
        SKILL,
        &rows.first(),
        &Some(&owned(&["Rule", "Required", "Checked by"])),
    );
    let named: Vec<String> = rows
        .iter()
        .skip(2)
        .filter_map(|row| row.first().cloned())
        .collect();
    failures.require_eq(SKILL, &named, &owned(&RULES));
    for row in rows.iter().skip(2) {
        failures.require(
            row.iter().all(|cell| !cell.is_empty()),
            SKILL,
            &format!("empty cell in {row:?}"),
        );
    }
    let task = tasks(&read(WORKFLOW)?)?
        .into_iter()
        .next()
        .unwrap_or_default();
    failures.contains(WORKFLOW, &task, "report");

    failures.finish()
}

/// Keeps the standalone skill's corrected answer cited, as the role requires.
#[test]
fn evidence_review_skill_requires_cited_corrected_answers() -> Check {
    let mut failures = Failures::default();
    let skill = read(SKILL)?;

    failures.matches(
        SKILL,
        &skill,
        r"(?m)^\d+\. Return .*corrected answer .*cites its sources",
    )?;
    for (column, which) in [(1, "Required"), (2, "Checked by")] {
        let cell = rule_column(&skill, column)?
            .remove("Cite")
            .unwrap_or_default();
        failures.contains(
            &format!("{SKILL} Cite {which}"),
            &cell,
            "`Corrected answer`",
        );
    }
    failures.quoted(SKILL, "docs/skills.md", "markdown", &skill)?;

    failures.finish()
}

/// Fails a report that skips a check without a reason or trusts a claimed result.
#[test]
fn evidence_review_rules_compare_gaps_and_results_with_observations() -> Check {
    let mut failures = Failures::default();
    let skill = read(SKILL)?;
    let role = read(ROLE)?;
    let checked = rule_column(&skill, 2)?;
    let gaps = checked.get("Report gaps").map_or("", String::as_str);
    let honesty = checked.get("Source honesty").map_or("", String::as_str);
    let required = rule_column(&skill, 1)?;
    let compare = "Compare each result the answer reports with the output you observed";

    for (rule, cell) in &checked {
        failures.avoids(rule, cell, r"^The section exists\.?$")?;
    }
    for term in [
        "reason",
        "`None` appears only when",
        "claims",
        "exactly `None`, or a nonempty list",
        "A heading with no entries fails",
    ] {
        failures.contains("Report gaps", gaps, term);
    }
    failures.contains(
        "Report gaps",
        required.get("Report gaps").map_or("", String::as_str),
        "exactly `None`",
    );
    for term in ["`Findings`", "`Corrected answer`", "output you observed"] {
        failures.contains("Source honesty", honesty, term);
    }
    failures.matches(
        SKILL,
        &skill,
        &format!(r"(?m)^\d+\. {compare}, and mark each mismatch\.$"),
    )?;
    failures.contains(
        ROLE,
        &role,
        &format!("{compare}, and report a mismatch as a `Verified failure`."),
    );

    failures.finish()
}

/// Gives each instruction its own line and avoids the words Maestro avoids.
#[test]
fn evidence_review_text_uses_maestro_voice() -> Check {
    let mut failures = Failures::default();
    let numbering = re(r"^\d+\. ")?;

    for name in [
        "agents/evidence-reviewer.md",
        "skills/review-evidence/SKILL.md",
        "prompts/explain-change.md",
        "workflows/review-evidence.yaml",
    ] {
        let path = format!("{CAPABILITY}/{name}");
        let text = read(&path)?;
        failures.avoids(
            &path,
            &text.to_lowercase(),
            &format!(r"\b({AVOIDED_WORDS})\b"),
        )?;
        let lines = if Path::new(name)
            .extension()
            .is_some_and(|ext| ext.eq_ignore_ascii_case("yaml"))
        {
            tasks(&text)?
        } else {
            split_frontmatter(&path, &text)?
                .1
                .lines()
                .map(str::to_owned)
                .collect()
        };
        for line in lines.iter().filter(|line| !line.starts_with('|')) {
            let count = sentence_count(&numbering.replace(line, ""))?;
            failures.require(
                count <= 1,
                &path,
                &format!("more than one sentence in `{line}`"),
            );
        }
    }

    failures.finish()
}

/// The color each group of theme tokens takes, with the tokens that share it.
const FORGE_ROLES: [(&str, &str); 6] = [
    (
        "#D9A066",
        "accent borderAccent customMessageLabel mdHeading mdLink mdCode mdListBullet syntaxKeyword \
         syntaxFunction thinkingLow thinkingMedium warning syntaxNumber muted dim thinkingText \
         toolOutput mdLinkUrl mdQuote toolDiffContext syntaxComment thinkingMinimal",
    ),
    (
        "#5E7383",
        "border borderMuted mdCodeBlockBorder mdQuoteBorder mdHr thinkingOff",
    ),
    (
        "#FF6A1A",
        "success toolDiffAdded syntaxString bashMode error toolDiffRemoved syntaxType thinkingHigh \
         thinkingXhigh",
    ),
    (
        "#F2E8DC",
        "mdCodeBlock syntaxVariable syntaxOperator syntaxPunctuation",
    ),
    (
        "#2A1F1A",
        "selectedBg userMessageBg customMessageBg toolPendingBg toolSuccessBg toolErrorBg",
    ),
    ("", "text userMessageText customMessageText toolTitle"),
];

/// Resolves each color of `theme` through its `vars`, keyed by token.
fn resolved_colors(theme: &Value) -> BTreeMap<String, Value> {
    let vars = theme.get("vars");
    let colors = theme.get("colors").and_then(Value::as_object);
    colors
        .into_iter()
        .flatten()
        .map(|(token, value)| {
            let color = value
                .as_str()
                .and_then(|name| vars.and_then(|vars| vars.get(name)))
                .unwrap_or(value);
            (token.clone(), color.clone())
        })
        .collect()
}

/// Uses Forge dark colors with readable text on every tool panel.
#[test]
fn review_dark_uses_forge_roles_for_all_tokens() -> Check {
    let mut failures = Failures::default();
    let path = format!("{CAPABILITY}/themes/review-dark.json");
    let theme = read_json(&path)?;
    let expected: BTreeMap<String, Value> = FORGE_ROLES
        .iter()
        .flat_map(|(color, tokens)| {
            tokens
                .split_whitespace()
                .map(move |token| (token.to_owned(), json!(color)))
        })
        .collect();

    failures.require_eq(&path, &expected.len(), &51);
    failures.require_eq(&path, &resolved_colors(&theme), &expected);
    failures.require_eq(&path, &theme.get("name"), &Some(&json!("review-dark")));
    failures.quoted(&path, "docs/themes.md", "json", &read(&path)?)?;

    failures.finish()
}

/// Keeps an indexed color as the number the theme holds, so it never equals the empty color.
#[test]
fn an_indexed_color_is_not_resolved_to_an_empty_one() -> Check {
    let mut failures = Failures::default();
    let theme = json!({"colors": {"text": 0}});

    failures.require_eq(
        "text",
        &resolved_colors(&theme).get("text"),
        &Some(&json!(0)),
    );

    failures.finish()
}

/// Resolves a color named in `vars` to the value `vars` holds, and keeps any other as written.
#[test]
fn colors_resolve_through_vars() -> Check {
    let mut failures = Failures::default();
    let theme = json!({
        "vars": {"gold": "#D9A066"},
        "colors": {"accent": "gold", "text": "", "border": "#5E7383"},
    });

    failures.require_eq(
        "colors",
        &resolved_colors(&theme),
        &BTreeMap::from([
            ("accent".to_owned(), json!("#D9A066")),
            ("text".to_owned(), json!("")),
            ("border".to_owned(), json!("#5E7383")),
        ]),
    );

    failures.finish()
}
