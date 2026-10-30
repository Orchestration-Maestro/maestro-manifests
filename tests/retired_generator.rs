//! Checks that no tracked file names the retired generator.

use crate::support::{Check, Failures, git};

/// Judges a scan for the generator's name: only "no match and nothing printed" passes.
///
/// Git exits 1 for no match, so any other exit code, such as 128 for a failed
/// scan, is rejected even when it prints nothing.
fn verdict(code: Option<i32>, stdout: &str, stderr: &str) -> Check {
    let mut failures = Failures::default();

    failures.require(
        code == Some(1),
        "git grep",
        &format!("exited {code:?}: {stderr}"),
    );
    failures.require(stdout.is_empty(), "git grep", &format!("found {stdout}"));

    failures.finish()
}

/// Finds no line that names the retired generator, in any case.
#[test]
fn no_tracked_file_names_the_generator() -> Check {
    // The bracket keeps this file from matching its own pattern.
    let scan = git(&["grep", "-n", "-i", "-e", "rust[-]gate"])?;

    verdict(scan.code, &scan.stdout, &scan.stderr)
}

/// Rejects a Git failure even when it produces no stdout.
#[test]
fn failed_scan_is_rejected() {
    assert!(verdict(Some(128), "", "scan failed").is_err());
}
