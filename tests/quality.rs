//! Exercises the contributor commands in disposable Git indexes.

use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicUsize, Ordering};

use crate::support::{Check, Failures, Report, tracked_files};

/// Numbers the checkouts of one test run, so no two share a directory.
static CHECKOUTS: AtomicUsize = AtomicUsize::new(0);

/// A command's exit code and its output.
struct Run {
    /// The exit code, or `None` when a signal ended the command.
    code: Option<i32>,
    /// Standard output followed by standard error.
    text: String,
}

/// A copy of the tracked files in a fresh Git repository, removed when dropped.
///
/// Running the contributor commands here leaves the contributor's own index alone.
struct Checkout {
    /// The directory that holds the copy.
    root: PathBuf,
}

impl Checkout {
    /// Copies every tracked file into a new directory, then runs `git init` and `git add .`.
    fn new() -> Result<Self, Report> {
        let number = CHECKOUTS.fetch_add(1, Ordering::Relaxed);
        let root =
            std::env::temp_dir().join(format!("maestro-manifests-{}-{number}", std::process::id()));
        std::fs::create_dir(&root)
            .map_err(|error| Report::new(format!("{}: {error}", root.display())))?;
        let checkout = Self { root };
        for name in tracked_files()? {
            let target = checkout.root.join(&name);
            let copied = target
                .parent()
                .map_or(Ok(()), std::fs::create_dir_all)
                .and_then(|()| {
                    std::fs::copy(Path::new(env!("CARGO_MANIFEST_DIR")).join(&name), &target)
                        .map(drop)
                });
            copied.map_err(|error| Report::new(format!("{name}: {error}")))?;
        }
        checkout.run("git", &["init", "-q"])?;
        checkout.run("git", &["add", "."])?;
        Ok(checkout)
    }

    /// Runs `program` in the copy and captures its output.
    fn run(&self, program: &str, args: &[&str]) -> Result<Run, Report> {
        let output = Command::new(program)
            .args(args)
            .current_dir(&self.root)
            .env("PREK_ALLOW_NO_CONFIG", "0")
            .output()
            .map_err(|error| Report::new(format!("{program} {}: {error}", args.join(" "))))?;
        let text = format!(
            "{}{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        Ok(Run {
            code: output.status.code(),
            text,
        })
    }

    /// Writes `content` to `name` in the copy.
    fn write(&self, name: &str, content: &str) -> Result<(), Report> {
        std::fs::write(self.root.join(name), content)
            .map_err(|error| Report::new(format!("{name}: {error}")))
    }
}

impl Drop for Checkout {
    /// Removes the copy, reporting a failure because a drop cannot return one.
    fn drop(&mut self) {
        if let Err(error) = std::fs::remove_dir_all(&self.root) {
            eprintln!("could not remove {}: {error}", self.root.display());
        }
    }
}

/// Runs only the Rust tests in `check`, which CI runs, and no tests in `content`, which they run.
#[test]
fn check_runs_the_content_checks_and_the_tests_without_recursion() -> Check {
    let mut failures = Failures::default();
    let checkout = Checkout::new()?;

    let check = checkout.run("just", &["--dry-run", "check"])?;
    failures.require(check.code == Some(0), "just --dry-run check", &check.text);
    failures.contains(
        "just --dry-run check",
        &check.text,
        "prek run --all-files --stage manual",
    );
    failures.contains("just --dry-run check", &check.text, "cargo test");
    failures.lacks("just --dry-run check", &check.text, "unittest");
    let content = checkout.run("just", &["--dry-run", "content"])?;
    failures.require(
        content.code == Some(0),
        "just --dry-run content",
        &content.text,
    );
    failures.lacks("just --dry-run content", &content.text, "cargo test");

    failures.finish()
}

/// Accepts the checkout and rejects malformed structured content.
#[test]
fn check_accepts_the_checkout_and_rejects_malformed_structured_content() -> Check {
    let mut failures = Failures::default();
    let checkout = Checkout::new()?;

    let clean = checkout.run("just", &["content"])?;
    failures.require(clean.code == Some(0), "just content", &clean.text);
    for (name, content, hook) in [
        ("broken.yaml", "value: [\n", "check-yaml"),
        ("broken.json", "{\"value\":\n", "check-json"),
        ("broken.toml", "value = [\n", "check-toml"),
    ] {
        checkout.write(name, content)?;
        checkout.run("git", &["add", name])?;

        let broken = checkout.run("just", &["content"])?;

        failures.require(broken.code != Some(0), name, "just content accepted it");
        failures.contains(name, &broken.text, &format!("- hook id: {hook}"));
        checkout.run("git", &["rm", "--cached", name])?;
        std::fs::remove_file(checkout.root.join(name))
            .map_err(|error| Report::new(format!("{name}: {error}")))?;
    }

    failures.finish()
}

/// Checks staged files without rejecting unrelated untracked content.
#[test]
fn pre_commit_checks_staged_files_without_rejecting_untracked_content() -> Check {
    let mut failures = Failures::default();
    let checkout = Checkout::new()?;
    let staged = ["run", "--stage", "pre-commit"];

    checkout.write("broken.yaml", "value: [\n")?;
    let untracked = checkout.run("just", &["pre-commit"])?;
    checkout.run("git", &["add", "broken.yaml"])?;
    let broken = checkout.run("prek", &staged)?;
    checkout.write("broken.yaml", "value: []\n")?;
    checkout.run("git", &["add", "broken.yaml"])?;
    let fixed = checkout.run("prek", &staged)?;

    failures.require(
        untracked.code == Some(0),
        "just pre-commit with an untracked file",
        &untracked.text,
    );
    failures.require(
        broken.code != Some(0),
        "prek with broken.yaml staged",
        "accepted it",
    );
    failures.contains(
        "prek with broken.yaml staged",
        &broken.text,
        "- hook id: check-yaml",
    );
    failures.require(
        fixed.code == Some(0),
        "prek with broken.yaml fixed",
        &fixed.text,
    );

    failures.finish()
}
