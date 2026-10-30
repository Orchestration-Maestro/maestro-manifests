//! Reading the files under a root: the one owner of listing a folder, reading
//! a file, parsing JSON and YAML, splitting frontmatter and collapsing whitespace, for
//! the scorecard and for the checks in `tests`.

use std::fmt;
use std::fs;
use std::io;
use std::path::Path;

use serde_json::Value;
use yaml_rust2::scanner::ScanError;
use yaml_rust2::{Yaml, YamlLoader};

/// Why a file or folder under a root could not be read.
#[derive(Debug)]
pub enum Error {
    /// A file or folder could not be read.
    Read {
        /// The path from the root.
        path: String,
        /// The failure from the operating system.
        source: io::Error,
    },
    /// A JSON file is not valid JSON.
    Json {
        /// The path from the root.
        path: String,
        /// The parser's failure.
        source: serde_json::Error,
    },
    /// A YAML file or frontmatter block is not valid YAML.
    Yaml {
        /// The path from the root.
        path: String,
        /// The scanner's failure.
        source: ScanError,
    },
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Read { path, .. } => write!(formatter, "cannot read {path}"),
            Self::Json { path, .. } => write!(formatter, "{path} is not valid JSON"),
            Self::Yaml { path, .. } => write!(formatter, "{path} is not valid YAML"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Read { source, .. } => Some(source),
            Self::Json { source, .. } => Some(source),
            Self::Yaml { source, .. } => Some(source),
        }
    }
}

/// Reads the file at `path`, a `/`-separated path from `root`.
///
/// # Errors
///
/// Returns [`Error::Read`] when the file cannot be read.
pub fn read(root: &str, path: &str) -> Result<String, Error> {
    fs::read_to_string(Path::new(root).join(path)).map_err(|source| Error::Read {
        path: path.to_owned(),
        source,
    })
}

/// Parses `text`, the content of the file `path`, as JSON.
///
/// # Errors
///
/// Returns [`Error::Json`] when `text` is not valid JSON.
pub fn parse_json(path: &str, text: &str) -> Result<Value, Error> {
    serde_json::from_str(text).map_err(|source| Error::Json {
        path: path.to_owned(),
        source,
    })
}

/// Parses `text`, the content of the file `path`, as YAML, one value per
/// document; empty text has no document.
///
/// # Errors
///
/// Returns [`Error::Yaml`] when `text` is not valid YAML.
pub fn parse_yaml(path: &str, text: &str) -> Result<Vec<Yaml>, Error> {
    YamlLoader::load_from_str(text).map_err(|source| Error::Yaml {
        path: path.to_owned(),
        source,
    })
}

/// Lists the entries of the folder `dir` of `root` as `(path, is_directory)`,
/// sorted, with `/`-separated paths from the root; an entry named in `skipped`
/// is left out. A symbolic link is not followed.
///
/// # Errors
///
/// Returns [`Error::Read`] naming `dir` when the folder or one of its entries
/// cannot be read.
pub fn entries(root: &str, dir: &str, skipped: &[&str]) -> Result<Vec<(String, bool)>, Error> {
    let failed = |source| Error::Read {
        path: dir.to_owned(),
        source,
    };
    let mut found = Vec::new();
    for entry in fs::read_dir(Path::new(root).join(dir)).map_err(failed)? {
        let entry = entry.map_err(failed)?;
        let name = entry.file_name().to_string_lossy().into_owned();
        if !skipped.contains(&name.as_str()) {
            let path = if dir.is_empty() {
                name
            } else {
                format!("{dir}/{name}")
            };
            found.push((path, entry.file_type().map_err(failed)?.is_dir()));
        }
    }
    found.sort();

    Ok(found)
}

/// Lists every file under the folder `dir` of `root`, sorted, as `/`-separated
/// paths from the root; an entry named in `skipped` is left out at any depth.
///
/// # Errors
///
/// Returns [`Error::Read`] when a folder or one of its entries cannot be read.
pub fn files(root: &str, dir: &str, skipped: &[&str]) -> Result<Vec<String>, Error> {
    let mut found = Vec::new();
    let mut pending = vec![dir.to_owned()];
    while let Some(current) = pending.pop() {
        for (path, is_directory) in entries(root, &current, skipped)? {
            if is_directory {
                pending.push(path);
            } else {
                found.push(path);
            }
        }
    }
    found.sort();

    Ok(found)
}

/// Splits a Markdown file into its frontmatter and body, or returns `None`
/// when the text does not open with a `---` line that a later `---` line closes.
#[must_use]
pub fn split_frontmatter(text: &str) -> Option<(&str, &str)> {
    let rest = text.strip_prefix("---\n")?;
    let mut end = 0;
    for line in rest.split_inclusive('\n') {
        if line.trim_end() == "---" {
            return Some((&rest[..end], &rest[end + line.len()..]));
        }
        end += line.len();
    }

    None
}

/// Returns `text` with every run of whitespace replaced by one space.
#[must_use]
pub fn collapse_whitespace(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}
