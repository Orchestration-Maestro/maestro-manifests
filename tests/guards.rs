//! Checks that every rule in the guard files compiles and that its examples behave as listed.

use std::path::Path;

use maestro_manifests_tests::MANIFEST_ROOT;
use regex::Regex;
use yaml_rust2::{Yaml, YamlLoader};

/// The guard files the core area ships, in file-name order.
const GUARD_FILES: [&str; 4] = [
    "destructive-commands.yaml",
    "pii.yaml",
    "prompt-injection.yaml",
    "protected-paths.yaml",
];

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
/// Returns a message naming the file when it cannot be read or parsed, or when a member the
/// example checks need is missing or has the wrong kind.
fn load_rules() -> Result<Vec<Rule>, String> {
    let directory = format!("{MANIFEST_ROOT}/core/guards");
    let entries = std::fs::read_dir(&directory).map_err(|error| format!("{directory}: {error}"))?;
    let mut names = Vec::new();
    for entry in entries {
        let name = entry
            .map_err(|error| format!("{directory}: {error}"))?
            .file_name();
        if Path::new(&name)
            .extension()
            .is_some_and(|extension| extension == "yaml")
        {
            names.push(name.to_string_lossy().into_owned());
        }
    }
    names.sort();
    let mut rules = Vec::new();
    for name in &names {
        rules.extend(read_file(&directory, name)?);
    }
    Ok(rules)
}

/// Reads the rules of one guard file, taking the members the example checks need from the
/// untyped YAML value; the file's shape is checked by Maestro's schema, not here.
fn read_file(directory: &str, name: &str) -> Result<Vec<Rule>, String> {
    let text = std::fs::read_to_string(format!("{directory}/{name}"))
        .map_err(|error| format!("{name}: {error}"))?;
    let documents = YamlLoader::load_from_str(&text).map_err(|error| format!("{name}: {error}"))?;
    let [document] = documents.as_slice() else {
        return Err(format!("{name}: expected one YAML document"));
    };
    let rules = document["rules"]
        .as_vec()
        .ok_or_else(|| format!("{name}: rules is not a list"))?;
    rules.iter().map(|rule| read_rule(name, rule)).collect()
}

/// Reads one rule's id, expression and example lists from its YAML mapping.
fn read_rule(file: &str, rule: &Yaml) -> Result<Rule, String> {
    let text = |value: &Yaml, member: &str| {
        value
            .as_str()
            .map(str::to_owned)
            .ok_or_else(|| format!("{file}: {member} is not a string"))
    };
    let examples = |member: &str| -> Result<Vec<String>, String> {
        let list = rule["examples"][member]
            .as_vec()
            .ok_or_else(|| format!("{file}: examples.{member} is not a list"))?;
        list.iter().map(|value| text(value, member)).collect()
    };
    Ok(Rule {
        file: file.to_owned(),
        id: text(&rule["id"], "id")?,
        regex: text(&rule["regex"], "regex")?,
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
    let expression = match Regex::new(regex) {
        Ok(expression) => expression,
        Err(error) => {
            found.push(format!(
                "{file}: rule {id}: the expression does not compile: {error}"
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
    let owned = |examples: &[&str]| {
        examples
            .iter()
            .map(|example| (*example).to_owned())
            .collect()
    };
    Rule {
        file: file.to_owned(),
        id: id.to_owned(),
        regex: regex.to_owned(),
        must_match: owned(must_match),
        must_not_match: owned(must_not_match),
    }
}

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
}

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

#[test]
fn a_rule_with_an_empty_example_list_fails_naming_the_list() {
    let found = problems(&rule("pii.yaml", "no-examples", "a", &[], &[]));
    assert_eq!(found.len(), 2, "{found:?}");
    assert!(
        found[0].contains("pii.yaml") && found[0].contains("no-examples"),
        "{found:?}"
    );
    assert!(
        found[0].contains("match") && found[1].contains("noMatch"),
        "{found:?}"
    );
}

#[test]
fn each_guard_file_holds_at_least_one_rule() -> Result<(), String> {
    let rules = load_rules()?;
    for file in GUARD_FILES {
        assert!(
            rules.iter().any(|rule| rule.file == file),
            "{file} holds no rules"
        );
    }
    Ok(())
}

#[test]
fn every_example_of_every_guard_rule_behaves_as_listed() -> Result<(), String> {
    let rules = load_rules()?;
    assert!(!rules.is_empty(), "no guard rules were loaded");
    let found: Vec<String> = rules.iter().flat_map(problems).collect();
    assert!(found.is_empty(), "{}", found.join("\n"));
    Ok(())
}
