//! Helpers the check modules share: failure collection, file reading and text splitting.

use std::fmt;
use std::path::Path;

use regex::Regex;

/// The repository root, from which every checked path is relative.
const ROOT: &str = env!("CARGO_MANIFEST_DIR");

/// A failed check: one line per problem, each naming the file it concerns.
pub struct Report(String);

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

/// Reads the file at the repository-relative `path`.
pub fn read(path: &str) -> Result<String, Report> {
    std::fs::read_to_string(Path::new(ROOT).join(path))
        .map_err(|error| Report(format!("{path}: {error}")))
}

/// Lists the entries of the repository-relative `dir` as `(path, is_directory)`.
///
/// Git's own directory and the build directory are left out.
fn entries(dir: &str) -> Result<Vec<(String, bool)>, Report> {
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
