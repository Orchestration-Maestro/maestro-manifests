//! Checks that the standards pages explain each principle family and map nothing in tables.

use crate::support::{Check, Failures, Report, captures, exists, pairs, re, read};

/// The topics every principle family explains.
const TOPICS: [&str; 3] = [
    "What it is",
    "Why this repository applies it",
    "How it is applied here",
];
/// The pillars of the Northstar page, each with a measured indicator.
const PILLARS: [&str; 4] = ["Speed", "Quality", "Maintainability", "Security"];
/// The pages that link the standards and must not call them generated.
const LINKING_PAGES: [&str; 3] = ["README.md", "docs/development.md", "docs/index.md"];
/// The heading the shared policy explanation lives under.
const LOCK_HEADING: &str = "What the organization can lock";
/// The anchor of that heading on the engineering page.
const LOCK_LINK: &str = "[Engineering standards](engineering.md#what-the-organization-can-lock)";

/// A principle family's heading with the rule IDs it names.
type Family = (&'static str, &'static [&'static str]);
/// A standards page's name with its principle families.
type Page = (&'static str, &'static [Family]);

/// Each standards page with its principle families and the rule IDs each family names.
const FAMILIES: [Page; 3] = [
    (
        "engineering",
        &[
            (
                "Think first, then change only what the goal needs",
                &["FND-001", "FND-002", "FND-003", "FND-004"],
            ),
            (
                "Keep the design small and plain",
                &[
                    "P-001", "P-002", "P-008", "P-009", "P-010", "P-015", "P-016", "P-017",
                ],
            ),
            (
                "Say a thing once, but not too early",
                &["P-003", "P-004", "P-005"],
            ),
            ("Respect what is already here", &["P-006", "P-007", "P-018"]),
            (
                "Refuse bad input and bad states",
                &["P-011", "P-012", "P-013"],
            ),
            (
                "Grant only the authority a task needs",
                &["P-014", "ENF-011"],
            ),
            (
                "Let tests and gates carry the rules",
                &["ENF-005", "ENF-006", "ENF-008", "ENF-009", "ENF-010"],
            ),
            (
                "Deliver through signed pull requests",
                &["ENF-004", "ENF-007"],
            ),
            (
                "Pin inputs and keep secrets out",
                &["ENF-012", "ENF-013", "ENF-014"],
            ),
            (
                "Write portable plain text",
                &["ENF-001", "ENF-002", "ENF-003"],
            ),
            (
                "Adopt the rules and prove it",
                &["C-001", "C-004", "C-005", "C-006"],
            ),
        ],
    ),
    (
        "security",
        &[
            ("Keep sensitive data small", &["SEC-001"]),
            ("Treat everything read as data", &["SEC-002", "SEC-003"]),
            ("Take authority from enforcement", &["SEC-004", "SEC-005"]),
            ("Inspect code without running it unsafely", &["SEC-006"]),
            (
                "Stop, escalate and report vulnerabilities",
                &["SEC-007", "SEC-010"],
            ),
            ("Keep evidence truthful", &["SEC-008", "SEC-009"]),
            ("Sign every release", &["SEC-011"]),
        ],
    ),
    (
        "northstar",
        &[
            ("Speed", &[]),
            ("Quality", &[]),
            ("Maintainability", &[]),
            ("Security", &[]),
        ],
    ),
];

/// A page split at its headings: each heading with the text up to the next one.
struct Sections(Vec<(String, String)>);

impl Sections {
    /// Splits `text` at each line that opens with `marker` and a space.
    fn split(text: &str, marker: &str) -> Result<Self, Report> {
        let headings = re(&format!(r"(?m)^{marker} (.+)\n"))?;
        let found: Vec<_> = headings.captures_iter(text).collect();
        let mut parts = Vec::new();
        for (index, heading) in found.iter().enumerate() {
            let (Some(whole), Some(title)) = (heading.get(0), heading.get(1)) else {
                continue;
            };
            let end = found
                .get(index + 1)
                .and_then(|next| next.get(0))
                .map_or(text.len(), |next| next.start());
            parts.push((title.as_str().to_owned(), text[whole.end()..end].to_owned()));
        }
        Ok(Self(parts))
    }

    /// Returns the text under `title`, if the page has that heading.
    fn get(&self, title: &str) -> Option<&str> {
        self.0
            .iter()
            .find(|(name, _)| name == title)
            .map(|(_, body)| body.as_str())
    }

    /// Returns the text under `title`, or an empty string when the heading is missing.
    fn text(&self, title: &str) -> &str {
        self.get(title).unwrap_or_default()
    }

    /// Returns every heading in order.
    fn titles(&self) -> Vec<&str> {
        self.0.iter().map(|(title, _)| title.as_str()).collect()
    }
}

/// Returns the text of the standards page `name`.
fn page(name: &str) -> Result<String, Report> {
    read(&format!("docs/standards/{name}.md"))
}

/// Returns the anchor GitHub gives `heading`.
fn slug(heading: &str) -> Result<String, Report> {
    Ok(re(r"[^\w -]")?
        .replace_all(&heading.to_lowercase(), "")
        .replace(' ', "-"))
}

/// Gives each principle family a section for what it is, why and how.
#[test]
fn every_family_explains_the_concept_the_reason_and_the_practice() -> Check {
    let mut failures = Failures::default();
    for (name, families) in FAMILIES {
        let sections = Sections::split(&page(name)?, "##")?;
        for (family, _) in families {
            let subject = format!("docs/standards/{name}.md `{family}`");
            let Some(section) = sections.get(family) else {
                failures.require(false, &subject, "has no section");
                continue;
            };
            let topics = Sections::split(section, "###")?;
            for topic in TOPICS {
                let words = topics.text(topic).split_whitespace().count();
                failures.require(
                    topics.get(topic).is_some(),
                    &subject,
                    &format!("has no `{topic}`"),
                );
                failures.require(
                    topics.get(topic).is_none() || words >= 20,
                    &subject,
                    &format!("`{topic}` has {words} words"),
                );
            }
        }
    }
    failures.finish()
}

/// Keeps each organization rule reachable from the family that explains it.
#[test]
fn every_golden_rule_is_named_in_its_family() -> Check {
    let mut failures = Failures::default();
    for (name, families) in FAMILIES {
        let sections = Sections::split(&page(name)?, "##")?;
        for (family, rules) in families {
            for rule in *rules {
                let subject = format!("docs/standards/{name}.md `{family}`");
                failures.matches(&subject, sections.text(family), &format!(r"\b{rule}\b"))?;
            }
        }
    }
    failures.finish()
}

/// Refuses tables and lines that open with a rule ID.
#[test]
fn no_page_has_a_mapping_table_or_rule_by_rule_rows() -> Check {
    let mut failures = Failures::default();
    for (name, _) in FAMILIES {
        let text = page(name)?;
        let subject = format!("docs/standards/{name}.md");
        failures.avoids(&subject, &text, r"(?m)^\s*\|")?;
        failures.avoids(
            &subject,
            &text,
            r"(?m)^[\s>*_-]*(?:FND|P|ENF|SEC|C)-\d{3}(?:\*\*|:|\s+[A-Z—–-])",
        )?;
        failures.lacks(&subject, &text.to_lowercase(), "not mapped yet");
    }
    failures.finish()
}

/// Opens with a title, a purpose paragraph and numbered links to each section.
#[test]
fn every_page_opens_as_a_guide_with_contents_for_its_sections() -> Check {
    let mut failures = Failures::default();
    for (name, _) in FAMILIES {
        let text = page(name)?;
        let sections = Sections::split(&text, "##")?;
        let subject = format!("docs/standards/{name}.md");
        let links = pairs(
            r"(?m)^\d+\. \[([^\]]+)\]\(#([^)]+)\)$",
            sections.text("Contents"),
        )?;
        let mut expected = Vec::new();
        for title in sections
            .titles()
            .into_iter()
            .filter(|title| *title != "Contents")
        {
            expected.push((title.to_owned(), slug(title)?));
        }

        failures.matches(&subject, &text, r"\A# [^\n]+\n\n[^#\n]")?;
        failures.require_eq(&subject, &links, &expected);
    }
    failures.finish()
}

/// Keeps shared controls in engineering and links the other standards to them.
#[test]
fn policy_control_explanation_has_one_source() -> Check {
    let mut failures = Failures::default();
    let explanation = "These standards govern contributions; they add no runtime policy keys.";
    let engineering = Sections::split(&page("engineering")?, "##")?;

    failures.contains("engineering", engineering.text(LOCK_HEADING), explanation);
    for name in ["northstar", "security"] {
        let text = page(name)?;
        failures.lacks(name, &text, explanation);
        failures.contains(
            name,
            Sections::split(&text, "##")?.text(LOCK_HEADING),
            LOCK_LINK,
        );
    }

    failures.finish()
}

/// Refuses the generator that no longer writes these pages.
#[test]
fn no_page_names_a_retired_mechanism() -> Check {
    let mut failures = Failures::default();
    for (name, _) in FAMILIES {
        failures.avoids(name, &page(name)?, r"(?i)rust[-]gate")?;
    }
    failures.finish()
}

/// Records every gate `text` names that this repository does not have.
fn check_gates(
    failures: &mut Failures,
    name: &str,
    text: &str,
    recipes: &[String],
    hooks: &[String],
) -> Check {
    let file = re(r"^[\w./-]+\.(?:md|json|yaml|yml|toml|lock|txt|py)$")?;
    for token in captures(r"`([^`]+)`", text)? {
        let subject = format!("{name} `{token}`");
        if file.is_match(&token) || token.ends_with('/') {
            failures.require(exists(&token), &subject, "does not exist");
        }
        if let Some(recipe) = token.strip_prefix("just ") {
            let recipe = recipe.split(' ').next().unwrap_or_default();
            failures.require(
                recipes.iter().any(|known| known == recipe),
                &subject,
                "is not a justfile recipe",
            );
        }
    }
    for group in captures(r"\bhooks? ((?:`[a-z-]+`(?:, | and )?)+)", text)? {
        for hook in captures(r"`([a-z-]+)`", &group)? {
            failures.require(
                hooks.contains(&hook),
                &format!("{name} hook {hook}"),
                "is not in .pre-commit-config.yaml",
            );
        }
    }
    Ok(())
}

/// Resolves each named file, recipe and hook in this checkout.
#[test]
fn pages_name_only_gates_this_repository_has() -> Check {
    let mut failures = Failures::default();
    let recipes = captures(r"(?m)^([a-z-]+):", &read("justfile")?)?;
    let hooks = captures(r"- id: ([\w-]+)", &read(".pre-commit-config.yaml")?)?;

    for (name, _) in FAMILIES {
        check_gates(&mut failures, name, &page(name)?, &recipes, &hooks)?;
    }

    failures.finish()
}

/// Names the asset and the three places untrusted input enters.
#[test]
fn security_page_says_what_it_protects() -> Check {
    let mut failures = Failures::default();
    let sections = Sections::split(&page("security")?, "##")?;
    let text = sections
        .text("What this repository protects")
        .to_lowercase();

    for entry in ["pull requests", "capability packages", "fetched content"] {
        failures.contains("security `What this repository protects`", &text, entry);
    }

    failures.finish()
}

/// Writes the point, then a current value, target and measuring job per pillar.
#[test]
fn northstar_page_states_the_point_and_a_measured_kpi_per_pillar() -> Check {
    let mut failures = Failures::default();
    let sections = Sections::split(&page("northstar")?, "##")?;
    let words = sections.text("The point").split_whitespace().count();

    failures.require(
        words >= 20,
        "northstar `The point`",
        &format!("has {words} words"),
    );
    for pillar in PILLARS {
        let kpi = Sections::split(sections.text(pillar), "###")?
            .text("KPI")
            .to_owned();
        let subject = format!("northstar `{pillar}` KPI");
        for label in ["Current", "Target", "Measured by"] {
            failures.matches(&subject, &kpi, &format!(r"(?m)^- \*\*{label}:\*\* \S"))?;
        }
        failures.lacks(&subject, &kpi.to_lowercase(), "none yet");
    }

    failures.finish()
}

/// Describes the standards pages as written by hand where the guides mention them.
#[test]
fn pages_that_link_the_standards_do_not_call_them_generated() -> Check {
    let mut failures = Failures::default();
    for name in LINKING_PAGES {
        let text = read(name)?;
        failures.avoids(name, &text, r"(?i)generated[- ]rule[- ]maps?")?;
        for paragraph in text
            .split("\n\n")
            .filter(|paragraph| paragraph.contains("standards/"))
        {
            failures.avoids(name, paragraph, r"(?i)generated|rust[-]gate")?;
        }
    }
    failures.finish()
}
