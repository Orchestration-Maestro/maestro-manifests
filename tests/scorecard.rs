//! Checks that the scorecard blocks conflicts and duplicates, over the mini
//! releases in `tests/scorecard_cases` and over this repository's own release.

use maestro_manifests_tests::scorecard::report;
use serde_json::{Value, json};
use std::fs;

/// The folder holding one mini release and its `expected.json` per case.
const CASES: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/scorecard_cases");

#[test]
fn every_case_reports_the_conflicts_its_expected_file_names() {
    let mut mismatches = Vec::new();

    for entry in fs::read_dir(CASES).expect("the cases folder exists") {
        let case = entry.expect("a case entry reads").path();
        let case = case.to_str().expect("a case path is UTF-8");
        let expected =
            fs::read_to_string(format!("{case}/expected.json")).expect("expected.json reads");
        let expected: Value = serde_json::from_str(&expected).expect("expected.json parses");
        let actual = report(case).unwrap_or_else(|error| json!({ "error": error.to_string() }));
        if actual != expected {
            mismatches.push(format!(
                "{case}\n  expected {expected}\n  actual   {actual}"
            ));
        }
    }

    assert!(mismatches.is_empty(), "{}", mismatches.join("\n"));
}

#[test]
fn a_root_with_no_manifests_folder_cannot_be_read() {
    let error = report(&format!("{CASES}/no-such-case")).expect_err("the root has no release");

    assert_eq!(error.to_string(), "cannot read manifests");
}

#[test]
fn this_release_has_no_conflicts() {
    let actual = report(env!("CARGO_MANIFEST_DIR")).expect("the release reads");

    assert_eq!(actual["status"], "pass", "{actual}");
}
