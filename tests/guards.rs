//! Checks that every rule in the guard files compiles and that its examples behave as listed.

use yaml_rust2::{Yaml, YamlLoader};

use crate::support::{Check, Failures, Report, files, owned, re, read};

/// The directory of the guard files, relative to the repository root.
const GUARDS: &str = "manifests/core/guards";

/// One rule of a guard file, as far as the example checks read it.
struct Rule {
    /// The name of the file the rule came from, for example `pii.yaml`.
    file: String,
    /// The rule's `id` member.
    id: String,
    /// The rule's `regex` member.
    regex: String,
    /// The strings the expression must match.
    must_match: Vec<String>,
    /// The strings the expression must not match.
    must_not_match: Vec<String>,
}

/// Reads every rule of every guard file under `core/guards/` in the manifest root.
///
/// # Errors
/// Returns a report naming the file when it cannot be read or parsed, or when a member the
/// example checks need is missing or has the wrong kind.
fn load_rules() -> Result<Vec<Rule>, Report> {
    let mut rules = Vec::new();
    for path in files(GUARDS, ".yaml")? {
        rules.extend(read_file(&path)?);
    }
    Ok(rules)
}

/// Reads the rules of the guard file at the repository-relative `path`, taking the members the
/// example checks need from the untyped YAML value; the file's shape is checked by Maestro's
/// schema, not here.
fn read_file(path: &str) -> Result<Vec<Rule>, Report> {
    let name = path.rsplit('/').next().unwrap_or(path);
    let text = read(path)?;
    let documents = YamlLoader::load_from_str(&text)
        .map_err(|error| Report::new(format!("{name}: {error}")))?;
    let [document] = documents.as_slice() else {
        return Err(Report::new(format!("{name}: expected one YAML document")));
    };
    let rules = document["rules"]
        .as_vec()
        .ok_or_else(|| Report::new(format!("{name}: rules is not a list")))?;
    rules.iter().map(|rule| read_rule(name, rule)).collect()
}

/// Reads one rule's id, expression and example lists from its YAML mapping.
fn read_rule(file: &str, rule: &Yaml) -> Result<Rule, Report> {
    let id = rule["id"]
        .as_str()
        .ok_or_else(|| Report::new(format!("{file}: id is not a string")))?;
    let text = |member: &str| {
        rule[member]
            .as_str()
            .map(str::to_owned)
            .ok_or_else(|| Report::new(format!("{file}: rule {id}: {member} is not a string")))
    };
    let examples = |member: &str| -> Result<Vec<String>, Report> {
        let list = rule["examples"][member].as_vec().ok_or_else(|| {
            Report::new(format!(
                "{file}: rule {id}: examples.{member} is not a list"
            ))
        })?;
        list.iter()
            .map(|value| {
                value.as_str().map(str::to_owned).ok_or_else(|| {
                    Report::new(format!(
                        "{file}: rule {id}: examples.{member} is not a string"
                    ))
                })
            })
            .collect()
    };
    Ok(Rule {
        file: file.to_owned(),
        id: id.to_owned(),
        regex: text("regex")?,
        must_match: examples("match")?,
        must_not_match: examples("noMatch")?,
    })
}

/// Returns one line for each way the rule fails its checks, each naming the file and the rule id.
fn problems(rule: &Rule) -> Vec<String> {
    let Rule {
        file,
        id,
        regex,
        must_match,
        must_not_match,
    } = rule;
    let mut found = Vec::new();
    for (list, member) in [(must_match, "match"), (must_not_match, "noMatch")] {
        if list.is_empty() {
            found.push(format!("{file}: rule {id}: examples.{member} is empty"));
        }
    }
    let expression = match re(regex) {
        Ok(expression) => expression,
        Err(report) => {
            found.push(format!(
                "{file}: rule {id}: the expression does not compile: {}",
                report.message()
            ));
            return found;
        }
    };
    found.extend(
        must_match
            .iter()
            .filter(|example| !expression.is_match(example))
            .map(|example| format!("{file}: rule {id}: must match {example:?}")),
    );
    found.extend(
        must_not_match
            .iter()
            .filter(|example| expression.is_match(example))
            .map(|example| format!("{file}: rule {id}: must not match {example:?}")),
    );
    found
}

/// Builds a rule from a file name, an id, an expression and its two example lists.
fn rule(file: &str, id: &str, regex: &str, must_match: &[&str], must_not_match: &[&str]) -> Rule {
    Rule {
        file: file.to_owned(),
        id: id.to_owned(),
        regex: regex.to_owned(),
        must_match: owned(must_match),
        must_not_match: owned(must_not_match),
    }
}

/// A look-around expression is reported as not compiling, with its file and rule id.
#[test]
fn look_around_fails_to_compile_and_names_its_file_and_rule() {
    let found = problems(&rule(
        "pii.yaml",
        "bad-look-ahead",
        "a(?=b)",
        &["ab"],
        &["ac"],
    ));
    assert_eq!(found.len(), 1, "{found:?}");
    assert!(found[0].contains("pii.yaml"), "{found:?}");
    assert!(found[0].contains("bad-look-ahead"), "{found:?}");
    assert!(found[0].contains("does not compile"), "{found:?}");
}

/// A back-reference expression is reported as not compiling, with its file and rule id.
#[test]
fn back_reference_fails_to_compile_and_names_its_file_and_rule() {
    let found = problems(&rule(
        "pii.yaml",
        "bad-back-reference",
        r"(a)\1",
        &["aa"],
        &["ab"],
    ));
    assert_eq!(found.len(), 1, "{found:?}");
    assert!(found[0].contains("pii.yaml"), "{found:?}");
    assert!(found[0].contains("bad-back-reference"), "{found:?}");
    assert!(found[0].contains("does not compile"), "{found:?}");
}

/// A wrong `match` or `noMatch` example is reported with its file, rule id and string.
#[test]
fn examples_that_do_not_behave_as_listed_are_named_with_their_rule() {
    let found = problems(&rule(
        "protected-paths.yaml",
        "wrong-examples",
        "^a$",
        &["b"],
        &["a"],
    ));
    assert_eq!(found.len(), 2, "{found:?}");
    assert!(found[0].contains("protected-paths.yaml") && found[0].contains("wrong-examples"));
    assert!(
        found[0].contains("must match") && found[0].contains("\"b\""),
        "{found:?}"
    );
    assert!(
        found[1].contains("must not match") && found[1].contains("\"a\""),
        "{found:?}"
    );
}

/// An empty example list is reported by the member it is missing from.
#[test]
fn a_rule_with_an_empty_example_list_fails_naming_the_list() {
    let found = problems(&rule("pii.yaml", "no-examples", "a", &[], &[]));
    assert_eq!(found.len(), 2, "{found:?}");
    assert!(
        found[0].contains("pii.yaml") && found[0].contains("no-examples"),
        "{found:?}"
    );
    assert!(found[0].contains("examples.match is empty"), "{found:?}");
    assert!(found[1].contains("examples.noMatch is empty"), "{found:?}");
}

/// Parses one rule mapping the way `read_file` hands it to `read_rule`.
fn parsed(text: &str) -> Result<Yaml, Report> {
    let documents =
        YamlLoader::load_from_str(text).map_err(|error| Report::new(error.to_string()))?;
    documents
        .into_iter()
        .next()
        .ok_or_else(|| Report::new("the rule text holds no YAML document".to_owned()))
}

/// Returns the message `read_rule` gives for the rule `text`, or a report when it accepts it.
fn rejection(text: &str) -> Result<String, Report> {
    match read_rule("pii.yaml", &parsed(text)?) {
        Err(report) => Ok(report.message().to_owned()),
        Ok(rule) => Err(Report::new(format!("rule {} was accepted", rule.id))),
    }
}

/// A non-string example is reported with its file, rule id and example list.
#[test]
fn a_non_string_example_names_its_file_rule_and_list() -> Check {
    let error = rejection("id: bad-example\nregex: a\nexamples:\n  match: [1]\n  noMatch: [b]\n")?;
    assert_eq!(
        error,
        "pii.yaml: rule bad-example: examples.match is not a string"
    );
    Ok(())
}

/// A missing example list is reported with its file, rule id and list name.
#[test]
fn a_missing_example_list_names_its_file_rule_and_list() -> Check {
    let error = rejection("id: no-list\nregex: a\nexamples:\n  match: [a]\n")?;
    assert_eq!(
        error,
        "pii.yaml: rule no-list: examples.noMatch is not a list"
    );
    Ok(())
}

/// A non-string expression is reported with its file and rule id.
#[test]
fn a_non_string_regex_names_its_file_and_rule() -> Check {
    let error = rejection("id: bad-regex\nregex: [a]\nexamples:\n  match: [a]\n  noMatch: [b]\n")?;
    assert_eq!(error, "pii.yaml: rule bad-regex: regex is not a string");
    Ok(())
}

/// Text that is not YAML is reported as a parse error, not as a missing member.
#[test]
fn rule_text_that_is_not_yaml_reports_the_parse_error() {
    let report = parsed("id: [unclosed\n");
    assert!(report.is_err(), "the broken text parsed");
}

/// A guard file that cannot be read is reported by name.
#[test]
fn a_missing_guard_file_is_reported_by_name() {
    let report = read_file("manifests/core/guards/absent.yaml")
        .err()
        .map(|report| report.message().to_owned())
        .unwrap_or_default();
    assert!(
        report.starts_with("manifests/core/guards/absent.yaml: "),
        "{report}"
    );
}

/// Every guard file under `core/guards/` holds at least one rule.
#[test]
fn each_guard_file_holds_at_least_one_rule() -> Check {
    let rules = load_rules()?;
    let mut failures = Failures::default();
    for path in files(GUARDS, ".yaml")? {
        let name = path.rsplit('/').next().unwrap_or(&path);
        failures.require(
            rules.iter().any(|rule| rule.file == name),
            name,
            "holds no rules",
        );
    }
    failures.finish()
}

/// Every example of every shipped rule behaves as its list says.
#[test]
fn every_example_of_every_guard_rule_behaves_as_listed() -> Check {
    let rules = load_rules()?;
    assert!(!rules.is_empty(), "no guard rules were loaded");
    let found: Vec<String> = rules.iter().flat_map(problems).collect();
    assert!(found.is_empty(), "{}", found.join("\n"));
    Ok(())
}
