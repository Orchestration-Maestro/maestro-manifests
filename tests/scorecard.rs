//! Checks that the scorecard blocks conflicts and duplicates, over the mini
//! releases in `tests/scorecard_cases` and over this repository's own release.

use maestro_manifests_tests::ROOT;
use maestro_manifests_tests::scorecard::report;
use serde_json::json;

use crate::support::{Check, Failures, Report, entries, read_json};

/// The folder holding one mini release and its `expected.json` per case.
const CASES: &str = "tests/scorecard_cases";

/// Each case reports the conflicts its `expected.json` names, or the error it names.
#[test]
fn every_case_reports_the_conflicts_its_expected_file_names() -> Check {
    let mut failures = Failures::default();

    for (case, _) in entries(CASES)? {
        let expected = read_json(&format!("{case}/expected.json"))?;
        let actual = report(&format!("{ROOT}/{case}"))
            .unwrap_or_else(|error| json!({ "error": error.to_string() }));
        failures.require_eq(&case, &actual, &expected);
    }

    failures.finish()
}

/// A root with no `manifests/` folder fails with the folder's name.
#[test]
fn a_root_with_no_manifests_folder_cannot_be_read() -> Check {
    let mut failures = Failures::default();

    match report(&format!("{ROOT}/{CASES}/no-such-case")) {
        Ok(found) => failures.require(false, CASES, &format!("read a missing root: {found}")),
        Err(error) => failures.require_eq(
            CASES,
            &error.to_string(),
            &"cannot read manifests".to_owned(),
        ),
    }

    failures.finish()
}

/// This repository's own release has no conflict.
#[test]
fn this_release_has_no_conflicts() -> Check {
    let actual = report(ROOT).map_err(|error| Report::new(error.to_string()))?;
    let mut failures = Failures::default();

    failures.require_eq("scorecard status", &actual["status"], &json!("pass"));

    failures.finish()
}
