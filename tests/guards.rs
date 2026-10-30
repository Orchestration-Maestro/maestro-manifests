//! Checks that every rule in the guard files compiles and that its examples behave as listed.

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
    Ok(Vec::new())
}

/// Returns one line for each way the rule fails its checks, each naming the file and the rule id.
fn problems(rule: &Rule) -> Vec<String> {
    let _ = rule;
    Vec::new()
}

/// Builds a rule from a file name, an id, an expression and its two example lists.
fn rule(file: &str, id: &str, regex: &str, must_match: &[&str], must_not_match: &[&str]) -> Rule {
    let owned = |examples: &[&str]| examples.iter().map(|example| (*example).to_owned()).collect();
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
    let found = problems(&rule("pii.yaml", "bad-look-ahead", "a(?=b)", &["ab"], &["ac"]));
    assert_eq!(found.len(), 1, "{found:?}");
    assert!(found[0].contains("pii.yaml"), "{found:?}");
    assert!(found[0].contains("bad-look-ahead"), "{found:?}");
    assert!(found[0].contains("does not compile"), "{found:?}");
}

#[test]
fn back_reference_fails_to_compile_and_names_its_file_and_rule() {
    let found = problems(&rule("pii.yaml", "bad-back-reference", r"(a)\1", &["aa"], &["ab"]));
    assert_eq!(found.len(), 1, "{found:?}");
    assert!(found[0].contains("pii.yaml"), "{found:?}");
    assert!(found[0].contains("bad-back-reference"), "{found:?}");
}

#[test]
fn examples_that_do_not_behave_as_listed_are_named_with_their_rule() {
    let found = problems(&rule("protected-paths.yaml", "wrong-examples", "^a$", &["b"], &["a"]));
    assert_eq!(found.len(), 2, "{found:?}");
    assert!(found[0].contains("protected-paths.yaml") && found[0].contains("wrong-examples"));
    assert!(found[0].contains("must match") && found[0].contains("\"b\""), "{found:?}");
    assert!(found[1].contains("must not match") && found[1].contains("\"a\""), "{found:?}");
}

#[test]
fn a_rule_with_an_empty_example_list_fails_naming_the_list() {
    let found = problems(&rule("pii.yaml", "no-examples", "a", &[], &[]));
    assert_eq!(found.len(), 2, "{found:?}");
    assert!(found[0].contains("pii.yaml") && found[0].contains("no-examples"), "{found:?}");
    assert!(found[0].contains("match") && found[1].contains("noMatch"), "{found:?}");
}

#[test]
fn each_guard_file_holds_at_least_one_rule() -> Result<(), String> {
    let rules = load_rules()?;
    for file in GUARD_FILES {
        assert!(rules.iter().any(|rule| rule.file == file), "{file} holds no rules");
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
