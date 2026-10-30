//! Helpers the check modules share: failure collection, file reading and text splitting.

use std::fmt;
use std::path::Path;

use regex::Regex;

/// The repository root, from which every checked path is relative.
pub const ROOT: &str = env!("CARGO_MANIFEST_DIR");

/// The words Maestro's voice avoids, as a regex alternation.
pub const AVOIDED_WORDS: &str = "just|simply|leverage|utilize|oops|awesome|robust|seamless|magic|\
                                 blazing|revolutionary|supercharge|please note";

/// A failed check: one line per problem, each naming the file it concerns.
pub struct Report(String);

impl Report {
    /// Wraps `message` as a failed check.
    pub fn new(message: String) -> Self {
        Self(message)
    }
}

impl fmt::Debug for Report {
    /// Prints the lines unescaped so the test runner shows them as written.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\n{}", self.0)
    }
}

/// What every check returns: `Ok` or the report of everything that failed.
pub type Check = Result<(), Report>;

/// Collects every failed check so one run reports them all, as a subtest loop does.
#[derive(Default)]
pub struct Failures(Vec<String>);

impl Failures {
    /// Records `subject: problem` when `ok` is false.
    pub fn require(&mut self, ok: bool, subject: &str, problem: &str) {
        if !ok {
            self.0.push(format!("{subject}: {problem}"));
        }
    }

    /// Records both values when they differ.
    pub fn require_eq<T: PartialEq + fmt::Debug + ?Sized>(
        &mut self,
        subject: &str,
        actual: &T,
        expected: &T,
    ) {
        if actual != expected {
            self.0
                .push(format!("{subject}: expected {expected:?}, got {actual:?}"));
        }
    }

    /// Records the subject when `text` does not contain `needle`.
    pub fn contains(&mut self, subject: &str, text: &str, needle: &str) {
        self.require(text.contains(needle), subject, &format!("no `{needle}`"));
    }

    /// Records the subject when `text` contains `needle`.
    pub fn lacks(&mut self, subject: &str, text: &str, needle: &str) {
        self.require(!text.contains(needle), subject, &format!("has `{needle}`"));
    }

    /// Records the subject when no part of `text` matches `pattern`.
    pub fn matches(&mut self, subject: &str, text: &str, pattern: &str) -> Check {
        self.require(
            re(pattern)?.is_match(text),
            subject,
            &format!("nothing matches {pattern}"),
        );
        Ok(())
    }

    /// Records the subject and the offending text when a part of `text` matches `pattern`.
    pub fn avoids(&mut self, subject: &str, text: &str, pattern: &str) -> Check {
        if let Some(found) = re(pattern)?.find(text) {
            self.0.push(format!("{subject}: has `{}`", found.as_str()));
        }
        Ok(())
    }

    /// Records the subject when `guide` does not quote `text` in a fenced block of language `fence`.
    pub fn quoted(&mut self, subject: &str, guide: &str, fence: &str, text: &str) -> Check {
        let block = format!("```{fence}\n{text}```");
        let problem = format!("{guide} does not quote it in a `{fence}` block");
        self.require(read(guide)?.contains(&block), subject, &problem);
        Ok(())
    }

    /// Returns `Ok` when nothing failed, otherwise the report of every failure.
    pub fn finish(self) -> Check {
        if self.0.is_empty() {
            Ok(())
        } else {
            Err(Report(self.0.join("\n")))
        }
    }
}

/// Compiles `pattern`, reporting a bad pattern as a failed check.
pub fn re(pattern: &str) -> Result<Regex, Report> {
    Regex::new(pattern).map_err(|error| Report(format!("pattern {pattern}: {error}")))
}

/// A finished command: its exit code and captured output.
pub struct Run {
    /// The exit code, or `None` when a signal ended the command.
    pub code: Option<i32>,
    /// What the command printed on standard output.
    pub stdout: String,
    /// What the command printed on standard error.
    pub stderr: String,
}

impl Run {
    /// Standard output followed by standard error.
    pub fn text(&self) -> String {
        format!("{}{}", self.stdout, self.stderr)
    }
}

/// Runs `program` with `args` in the directory `dir`, with `env` added to its environment.
pub fn run_in(
    dir: &Path,
    program: &str,
    args: &[&str],
    env: &[(&str, &str)],
) -> Result<Run, Report> {
    let output = std::process::Command::new(program)
        .args(args)
        .current_dir(dir)
        .envs(env.iter().copied())
        .output()
        .map_err(|error| Report(format!("{program} {}: {error}", args.join(" "))))?;
    Ok(Run {
        code: output.status.code(),
        stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
        stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
    })
}

/// Runs `git` with `args` in the repository.
pub fn git(args: &[&str]) -> Result<Run, Report> {
    run_in(Path::new(ROOT), "git", args, &[])
}

/// Reads the file at the repository-relative `path`.
pub fn read(path: &str) -> Result<String, Report> {
    std::fs::read_to_string(Path::new(ROOT).join(path))
        .map_err(|error| Report(format!("{path}: {error}")))
}

/// Reads and parses the JSON file at the repository-relative `path`.
pub fn json(path: &str) -> Result<serde_json::Value, Report> {
    serde_json::from_str(&read(path)?).map_err(|error| Report(format!("{path}: {error}")))
}

/// Copies string literals into owned strings, for comparison with captured text.
pub fn owned(items: &[&str]) -> Vec<String> {
    items.iter().map(|item| (*item).to_owned()).collect()
}

/// Returns the `agent` of each step of a workflow file's chain.
pub fn agents(workflow: &str) -> Result<Vec<String>, Report> {
    captures(r"(?m)^  - agent: (.+)$", workflow)
}

/// Returns the `task` of each step of a workflow file's chain.
pub fn tasks(workflow: &str) -> Result<Vec<String>, Report> {
    captures(r"(?m)^    task: (.+)$", workflow)
}

/// Lists the repository-relative paths Git tracks.
pub fn tracked_files() -> Result<Vec<String>, Report> {
    let listing = git(&["ls-files", "-z"])?;
    if listing.code != Some(0) {
        return Err(Report(format!(
            "git ls-files exited {:?}: {}",
            listing.code, listing.stderr
        )));
    }
    Ok(listing
        .stdout
        .split('\0')
        .filter(|name| !name.is_empty())
        .map(str::to_owned)
        .collect())
}

/// Lists the entries of the repository-relative `dir` as `(path, is_directory)`.
///
/// Git's own directory and the build directory are left out.
pub fn entries(dir: &str) -> Result<Vec<(String, bool)>, Report> {
    let failed = |error: std::io::Error| Report(format!("{dir}: {error}"));
    let mut found = Vec::new();
    for entry in std::fs::read_dir(Path::new(ROOT).join(dir)).map_err(failed)? {
        let entry = entry.map_err(failed)?;
        let name = entry.file_name().to_string_lossy().into_owned();
        if name != ".git" && name != "target" {
            let path = if dir.is_empty() {
                name
            } else {
                format!("{dir}/{name}")
            };
            found.push((path, entry.path().is_dir()));
        }
    }
    Ok(found)
}

/// Lists the files under the repository-relative `dir` whose names end in `suffix`.
///
/// Paths are repository-relative and sorted.
pub fn files(dir: &str, suffix: &str) -> Result<Vec<String>, Report> {
    let mut found = Vec::new();
    let mut pending = vec![dir.to_owned()];
    while let Some(current) = pending.pop() {
        for (path, is_directory) in entries(&current)? {
            if is_directory {
                pending.push(path);
            } else if path.ends_with(suffix) {
                found.push(path);
            }
        }
    }
    found.sort();
    Ok(found)
}

/// Splits a Markdown resource into its `---` frontmatter and its body.
pub fn split_frontmatter<'a>(path: &str, text: &'a str) -> Result<(&'a str, &'a str), Report> {
    let mut parts = text.splitn(3, "---\n");
    match (parts.next(), parts.next(), parts.next()) {
        (Some(_), Some(frontmatter), Some(body)) => Ok((frontmatter, body)),
        _ => Err(Report(format!("{path}: no `---` frontmatter"))),
    }
}

/// Returns groups `1` and `2` of every match of `pattern` in `text`.
pub fn pairs(pattern: &str, text: &str) -> Result<Vec<(String, String)>, Report> {
    let pattern = re(pattern)?;
    Ok(pattern
        .captures_iter(text)
        .filter_map(|found| {
            Some((
                found.get(1)?.as_str().to_owned(),
                found.get(2)?.as_str().to_owned(),
            ))
        })
        .collect())
}

/// Whether the repository-relative `path` exists.
pub fn exists(path: &str) -> bool {
    Path::new(ROOT).join(path).exists()
}

/// Counts the sentences in `text`: each `.`, `!` or `?` followed by whitespace or the end.
pub fn sentence_count(text: &str) -> Result<usize, Report> {
    Ok(re(r"[.!?](?:\s|$)")?.find_iter(text).count())
}

/// Returns `text` with every run of whitespace replaced by one space.
pub fn collapse_whitespace(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Returns `text` without its fenced code blocks.
pub fn strip_fences(text: &str) -> Result<String, Report> {
    Ok(re(r"(?s)```.*?```")?.replace_all(text, "").into_owned())
}

/// Returns the capture group `1` of every match of `pattern` in `text`.
pub fn captures(pattern: &str, text: &str) -> Result<Vec<String>, Report> {
    let pattern = re(pattern)?;
    Ok(pattern
        .captures_iter(text)
        .filter_map(|found| found.get(1).map(|group| group.as_str().to_owned()))
        .collect())
}
