//! The scorecard's blocking conflicts: names, capability names and contents that
//! two artifacts of one release must not share.
//!
//! A release is the `manifests/` folder of a root. The check reads the whole
//! release, so a change that adds one side of a conflict fails and the report
//! names both artifacts.

use std::collections::BTreeMap;
use std::fmt;
use std::fs;
use std::io;
use std::path::Path;

use serde_json::{Map, Value, json};
use yaml_rust2::scanner::ScanError;
use yaml_rust2::{Yaml, YamlLoader};

/// The kind of an authored artifact; a name is unique per kind.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ArtifactKind {
    /// A role, `agents/<file>.md`, named by its frontmatter `name`.
    Role,
    /// A skill, `skills/<folder>/SKILL.md`, named by its frontmatter `name`.
    Skill,
    /// An instruction, `instructions/<name>.instructions.md`, named by its file.
    Instruction,
    /// A prompt template, `prompts/<name>.md`, named by its file.
    Prompt,
    /// A workflow, `workflows/<file>.yaml`, named by its `name`.
    Workflow,
    /// A theme, `themes/<file>.json`, named by its `name`.
    Theme,
}

impl ArtifactKind {
    /// The kind as the report spells it.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Role => "role",
            Self::Skill => "skill",
            Self::Instruction => "instruction",
            Self::Prompt => "prompt",
            Self::Workflow => "workflow",
            Self::Theme => "theme",
        }
    }
}

/// The rule a conflict breaks.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Rule {
    /// One kind and name at two places of the release.
    SameName,
    /// Two capabilities with one `package.json` name.
    SameCapabilityName,
    /// Two artifacts of one kind with equal content once the name is removed.
    SameContent,
}

impl Rule {
    /// The rule as the report spells it.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SameName => "same-name",
            Self::SameCapabilityName => "same-capability-name",
            Self::SameContent => "same-content",
        }
    }
}

/// Artifacts that cannot stay together in one release.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Conflict {
    /// The rule the artifacts break.
    pub rule: Rule,
    /// The kind of the artifacts; absent for capabilities.
    pub artifact_kind: Option<ArtifactKind>,
    /// The shared name; absent when the rule is about content.
    pub name: Option<String>,
    /// Every artifact in the conflict, as paths from the root, sorted.
    pub artifacts: Vec<String>,
}

impl Conflict {
    /// The conflict as one entry of the report.
    fn to_json(&self) -> Value {
        let mut entry = Map::new();
        entry.insert("kind".to_owned(), self.rule.as_str().into());
        if let Some(kind) = self.artifact_kind {
            entry.insert("artifactKind".to_owned(), kind.as_str().into());
        }
        if let Some(name) = &self.name {
            entry.insert("name".to_owned(), name.as_str().into());
        }
        entry.insert("artifacts".to_owned(), json!(self.artifacts));
        Value::Object(entry)
    }
}

/// Why a release could not be read.
#[derive(Debug)]
pub enum ScorecardError {
    /// A file or folder could not be read.
    Read {
        /// The path from the root.
        path: String,
        /// The failure from the operating system.
        source: io::Error,
    },
    /// A frontmatter block or YAML file is not valid YAML.
    Yaml {
        /// The path from the root.
        path: String,
        /// The scanner's failure.
        source: ScanError,
    },
    /// A JSON file is not valid JSON.
    Json {
        /// The path from the root.
        path: String,
        /// The parser's failure.
        source: serde_json::Error,
    },
    /// A file that must carry a `name` has none.
    MissingName {
        /// The path from the root.
        path: String,
    },
}

impl fmt::Display for ScorecardError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Read { path, .. } => write!(formatter, "cannot read {path}"),
            Self::Yaml { path, .. } => write!(formatter, "{path} is not valid YAML"),
            Self::Json { path, .. } => write!(formatter, "{path} is not valid JSON"),
            Self::MissingName { path } => write!(formatter, "{path} has no name"),
        }
    }
}

impl std::error::Error for ScorecardError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Read { source, .. } => Some(source),
            Self::Yaml { source, .. } => Some(source),
            Self::Json { source, .. } => Some(source),
            Self::MissingName { .. } => None,
        }
    }
}

/// What makes two artifacts of one kind duplicates: the frontmatter or document
/// without its `name`, and the body with whitespace collapsed.
#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Content {
    /// The structured part without `name`; a theme keeps its JSON text here.
    head: Yaml,
    /// The Markdown body with whitespace collapsed; empty for YAML and JSON files.
    body: String,
}

/// One artifact of the release.
struct Artifact {
    /// The artifact's kind.
    kind: ArtifactKind,
    /// The artifact's name, unique per kind.
    name: String,
    /// The path from the root.
    path: String,
    /// The part compared for duplicates.
    content: Content,
}

/// Describes the conflicts of the release under `root` as a JSON report with a
/// `status` (`pass` or `fail`) and the `conflicts`.
///
/// # Errors
///
/// Returns [`ScorecardError`] when a file of the release cannot be read or parsed.
pub fn report(root: &str) -> Result<Value, ScorecardError> {
    let found = conflicts(root)?;
    let status = if found.is_empty() { "pass" } else { "fail" };

    Ok(json!({
        "status": status,
        "conflicts": found.iter().map(Conflict::to_json).collect::<Vec<_>>(),
    }))
}

/// Finds every conflict of the release under `root`, sorted.
///
/// # Errors
///
/// Returns [`ScorecardError`] when a file of the release cannot be read or parsed.
pub fn conflicts(root: &str) -> Result<Vec<Conflict>, ScorecardError> {
    let mut files = Vec::new();
    collect_files(root, "manifests", &mut files)?;
    files.sort();

    let mut artifacts = Vec::new();
    let mut capabilities = Vec::new();
    for path in files {
        if let Some(kind) = classify(&path) {
            artifacts.push(load_artifact(root, &path, kind)?);
        } else if is_capability_package(&path) {
            capabilities.push((capability_name(root, &path)?, path));
        }
    }

    let mut found = Vec::new();
    for ((kind, name), paths) in duplicates(
        artifacts
            .iter()
            .map(|a| ((a.kind, a.name.as_str()), a.path.as_str())),
    ) {
        found.push(Conflict {
            rule: Rule::SameName,
            artifact_kind: Some(kind),
            name: Some(name.to_owned()),
            artifacts: paths,
        });
    }
    for ((kind, _), paths) in duplicates(
        artifacts
            .iter()
            .map(|a| ((a.kind, &a.content), a.path.as_str())),
    ) {
        found.push(Conflict {
            rule: Rule::SameContent,
            artifact_kind: Some(kind),
            name: None,
            artifacts: paths,
        });
    }
    for (name, paths) in duplicates(
        capabilities
            .iter()
            .map(|(name, path)| (name.as_str(), path.as_str())),
    ) {
        found.push(Conflict {
            rule: Rule::SameCapabilityName,
            artifact_kind: None,
            name: Some(name.to_owned()),
            artifacts: paths,
        });
    }
    found.sort();

    Ok(found)
}

/// Groups `(key, path)` pairs by key and keeps the keys that two or more paths share.
fn duplicates<'a, K: Ord>(items: impl Iterator<Item = (K, &'a str)>) -> Vec<(K, Vec<String>)> {
    let mut groups: BTreeMap<K, Vec<String>> = BTreeMap::new();
    for (key, path) in items {
        groups.entry(key).or_default().push(path.to_owned());
    }

    groups
        .into_iter()
        .filter(|(_, paths)| paths.len() > 1)
        .collect()
}

/// Lists every file under `relative` in `root`, as `/`-separated paths from the root.
fn collect_files(
    root: &str,
    relative: &str,
    files: &mut Vec<String>,
) -> Result<(), ScorecardError> {
    let read_failure = |source| ScorecardError::Read {
        path: relative.to_owned(),
        source,
    };
    for entry in fs::read_dir(format!("{root}/{relative}")).map_err(read_failure)? {
        let entry = entry.map_err(read_failure)?;
        let path = format!("{relative}/{}", entry.file_name().to_string_lossy());
        if entry.file_type().map_err(read_failure)?.is_dir() {
            collect_files(root, &path, files)?;
        } else {
            files.push(path);
        }
    }

    Ok(())
}

/// The kind of artifact a release path holds, if it holds one.
fn classify(path: &str) -> Option<ArtifactKind> {
    let parts: Vec<&str> = path.split('/').collect();
    match parts.as_slice() {
        [.., "skills", _, "SKILL.md"] => Some(ArtifactKind::Skill),
        [.., "agents", file] if has_extension(file, "md") => Some(ArtifactKind::Role),
        [.., "instructions", file] if file.ends_with(".instructions.md") => {
            Some(ArtifactKind::Instruction)
        }
        [.., "prompts", file] if has_extension(file, "md") => Some(ArtifactKind::Prompt),
        [.., "workflows", file] if has_extension(file, "yaml") => Some(ArtifactKind::Workflow),
        [.., "themes", file] if has_extension(file, "json") => Some(ArtifactKind::Theme),
        _ => None,
    }
}

/// Whether `file` ends in `.extension`, whatever the case of the extension.
fn has_extension(file: &str, extension: &str) -> bool {
    Path::new(file)
        .extension()
        .is_some_and(|found| found.eq_ignore_ascii_case(extension))
}

/// Whether a release path is the `package.json` of a capability,
/// `capabilities/<group>/<name>/package.json`.
fn is_capability_package(path: &str) -> bool {
    matches!(
        path.split('/').collect::<Vec<_>>().as_slice(),
        ["manifests", "capabilities", _, _, "package.json"]
    )
}

/// Reads the `name` of the capability whose `package.json` is at `path`.
fn capability_name(root: &str, path: &str) -> Result<String, ScorecardError> {
    let text = read(root, path)?;
    let package: Value = serde_json::from_str(&text).map_err(|source| ScorecardError::Json {
        path: path.to_owned(),
        source,
    })?;

    package["name"]
        .as_str()
        .map(str::to_owned)
        .ok_or_else(|| ScorecardError::MissingName {
            path: path.to_owned(),
        })
}

/// Reads the file at `path` in `root`.
fn read(root: &str, path: &str) -> Result<String, ScorecardError> {
    fs::read_to_string(format!("{root}/{path}")).map_err(|source| ScorecardError::Read {
        path: path.to_owned(),
        source,
    })
}

/// Reads the artifact of `kind` at `path`, with its name and comparable content.
fn load_artifact(root: &str, path: &str, kind: ArtifactKind) -> Result<Artifact, ScorecardError> {
    let text = read(root, path)?;
    let yaml = |source| ScorecardError::Yaml {
        path: path.to_owned(),
        source,
    };
    let (named, content) = match kind {
        ArtifactKind::Theme => theme_content(path, &text)?,
        ArtifactKind::Workflow => {
            let (name, head) = take_name(first_document(&text).map_err(yaml)?);
            (
                name,
                Content {
                    head,
                    body: String::new(),
                },
            )
        }
        _ => {
            let (frontmatter, body) = split_frontmatter(&text);
            let (name, head) = take_name(first_document(frontmatter).map_err(yaml)?);
            (
                name,
                Content {
                    head,
                    body: body.split_whitespace().collect::<Vec<_>>().join(" "),
                },
            )
        }
    };
    let file = path.rsplit('/').next().unwrap_or(path);
    let name = match kind {
        ArtifactKind::Instruction => file.strip_suffix(".instructions.md").map(str::to_owned),
        ArtifactKind::Prompt => file.strip_suffix(".md").map(str::to_owned),
        _ => named,
    };

    Ok(Artifact {
        kind,
        name: name.ok_or_else(|| ScorecardError::MissingName {
            path: path.to_owned(),
        })?,
        path: path.to_owned(),
        content,
    })
}

/// Reads a theme's `name` and the rest of its JSON as comparable content.
fn theme_content(path: &str, text: &str) -> Result<(Option<String>, Content), ScorecardError> {
    let mut theme: Value = serde_json::from_str(text).map_err(|source| ScorecardError::Json {
        path: path.to_owned(),
        source,
    })?;
    let name = theme
        .as_object_mut()
        .and_then(|fields| fields.shift_remove("name"))
        .and_then(|name| name.as_str().map(str::to_owned));

    Ok((
        name,
        Content {
            head: Yaml::String(theme.to_string()),
            body: String::new(),
        },
    ))
}

/// Parses the first YAML document of `text`; empty text is a null document.
fn first_document(text: &str) -> Result<Yaml, ScanError> {
    Ok(YamlLoader::load_from_str(text)?
        .into_iter()
        .next()
        .unwrap_or(Yaml::Null))
}

/// Removes the `name` key from a YAML mapping and returns it with the rest.
fn take_name(document: Yaml) -> (Option<String>, Yaml) {
    match document {
        Yaml::Hash(mut fields) => {
            let name = fields
                .remove(&Yaml::String("name".to_owned()))
                .and_then(|name| name.as_str().map(str::to_owned));
            (name, Yaml::Hash(fields))
        }
        other => (None, other),
    }
}

/// Splits a Markdown file into its frontmatter and body; a file with no
/// frontmatter block has an empty frontmatter.
fn split_frontmatter(text: &str) -> (&str, &str) {
    let Some(rest) = text.strip_prefix("---\n") else {
        return ("", text);
    };
    let mut end = 0;
    for line in rest.split_inclusive('\n') {
        if line.trim_end() == "---" {
            return (&rest[..end], &rest[end + line.len()..]);
        }
        end += line.len();
    }

    ("", text)
}
