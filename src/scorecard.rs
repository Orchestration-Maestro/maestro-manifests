//! The scorecard's blocking conflicts: names, capability names and contents that
//! two artifacts of one release must not share.
//!
//! A release is the `manifests/` folder of a root. The check reads the whole
//! release, so a change that adds one side of a conflict fails and the report
//! names both artifacts.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use serde_json::{Map, Value, json};
use yaml_rust2::scanner::ScanError;
use yaml_rust2::{Yaml, YamlLoader};

use crate::reading;

/// The kind of an authored artifact; a name is unique per kind.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum ArtifactKind {
    /// A role, `agents/<file>.md`, named by its frontmatter `name`.
    Role,
    /// A skill found as the skill loader finds it under `skills/`, named by its
    /// frontmatter `name` or else by its folder.
    Skill,
    /// An instruction, `instructions/<name>.instructions.md`, named by its file.
    Instruction,
    /// A prompt template, `prompts/<name>.md`, named by its file.
    Prompt,
    /// A workflow, `workflows/<file>.yaml`, named by its `name`.
    Workflow,
    /// A theme, `themes/<file>.json`, named by its `name`.
    Theme,
    /// A guard rule in `guards/<file>.yaml`, named by its `id`.
    GuardRule,
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
            Self::GuardRule => "guard-rule",
        }
    }
}

/// The rule a conflict breaks.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum Rule {
    /// One kind and name at two places of the release.
    Name,
    /// Two capabilities with one `package.json` name.
    CapabilityName,
    /// Two artifacts of one kind with equal content once the name is removed.
    Content,
}

impl Rule {
    /// The rule as the report spells it.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Name => "same-name",
            Self::CapabilityName => "same-capability-name",
            Self::Content => "same-content",
        }
    }
}

/// Artifacts that cannot stay together in one release.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct Conflict {
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
pub enum Error {
    /// A file or folder could not be read, or a JSON file is not valid JSON.
    Reading(reading::Error),
    /// A frontmatter block or YAML file is not valid YAML.
    Yaml {
        /// The path from the root.
        path: String,
        /// The scanner's failure.
        source: ScanError,
    },
    /// A file that must carry a `name`, or a guard rule that must carry an `id`, has none.
    MissingName {
        /// The path from the root.
        path: String,
        /// The key that is missing or empty: `name` or `id`.
        key: &'static str,
    },
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Reading(error) => fmt::Display::fmt(error, formatter),
            Self::Yaml { path, .. } => write!(formatter, "{path} is not valid YAML"),
            Self::MissingName { path, key } => write!(formatter, "{path} has no {key}"),
        }
    }
}

impl From<reading::Error> for Error {
    fn from(error: reading::Error) -> Self {
        Self::Reading(error)
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Reading(error) => std::error::Error::source(error),
            Self::Yaml { source, .. } => Some(source),
            Self::MissingName { .. } => None,
        }
    }
}

/// What makes two artifacts of one kind duplicates: the frontmatter or document
/// without its name, and the body with whitespace collapsed.
#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Content {
    /// The structured part without its name; null for a theme.
    head: Yaml,
    /// The Markdown body with whitespace collapsed, or a theme's JSON text; empty
    /// for YAML files.
    body: String,
}

/// One artifact of the release.
struct Artifact {
    /// The artifact's kind.
    kind: ArtifactKind,
    /// The artifact's name, unique per kind.
    name: String,
    /// The path from the root; a guard rule adds `#` and its id.
    path: String,
    /// The part compared for duplicates.
    content: Content,
}

/// The kind of artifact a release path holds, with the name its path gives it.
struct Located {
    /// The artifact's kind.
    kind: ArtifactKind,
    /// The file stem for an instruction or a prompt, which the file name names;
    /// the folder for a skill, which names it when its frontmatter does not.
    path_name: Option<String>,
}

/// Describes the conflicts of the release under `root` as a JSON report with a
/// `status` (`pass` or `fail`) and the `conflicts`.
///
/// # Errors
///
/// Returns [`Error`] when a file of the release cannot be read or parsed.
pub fn report(root: &str) -> Result<Value, Error> {
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
/// Returns [`Error`] when a file of the release cannot be read or parsed.
pub(crate) fn conflicts(root: &str) -> Result<Vec<Conflict>, Error> {
    let files = reading::files(root, "manifests", &[])?;
    let known: BTreeSet<&str> = files.iter().map(String::as_str).collect();

    let mut artifacts = Vec::new();
    let mut capabilities = Vec::new();
    for path in &files {
        let parts: Vec<&str> = path.split('/').collect();
        if let Some(located) = classify(&parts, &known) {
            artifacts.extend(load_artifacts(root, path, located)?);
        } else if let ["manifests", "capabilities", _, _, "package.json"] = parts.as_slice() {
            capabilities.push((capability_name(root, path)?, path.as_str()));
        }
    }

    let mut found = artifact_conflicts(&artifacts);
    found.extend(
        duplicates(
            capabilities
                .iter()
                .map(|(name, path)| (name.as_str(), *path)),
        )
        .into_iter()
        .map(|(name, paths)| Conflict {
            rule: Rule::CapabilityName,
            artifact_kind: None,
            name: Some(name.to_owned()),
            artifacts: paths,
        }),
    );
    found.sort();

    Ok(found)
}

/// Finds the artifacts that share a kind and name, and those that share a kind
/// and content.
fn artifact_conflicts(artifacts: &[Artifact]) -> Vec<Conflict> {
    let same_name = duplicates(
        artifacts
            .iter()
            .map(|a| ((a.kind, a.name.as_str()), a.path.as_str())),
    )
    .into_iter()
    .map(|((kind, name), paths)| Conflict {
        rule: Rule::Name,
        artifact_kind: Some(kind),
        name: Some(name.to_owned()),
        artifacts: paths,
    });
    let same_content = duplicates(
        artifacts
            .iter()
            .map(|a| ((a.kind, &a.content), a.path.as_str())),
    )
    .into_iter()
    .map(|((kind, _), paths)| Conflict {
        rule: Rule::Content,
        artifact_kind: Some(kind),
        name: None,
        artifacts: paths,
    });

    same_name.chain(same_content).collect()
}

/// Groups `(key, path)` pairs by key and keeps the keys that two or more paths
/// share, with each key's paths sorted.
fn duplicates<'a, K: Ord>(items: impl Iterator<Item = (K, &'a str)>) -> Vec<(K, Vec<String>)> {
    let mut groups: BTreeMap<K, Vec<String>> = BTreeMap::new();
    for (key, path) in items {
        groups.entry(key).or_default().push(path.to_owned());
    }

    groups
        .into_iter()
        .filter(|(_, paths)| paths.len() > 1)
        .map(|(key, mut paths)| {
            paths.sort();
            (key, paths)
        })
        .collect()
}

/// The kind of artifact a release path holds, if it holds one. Every kind starts
/// at the root of an area, `core/` or `capabilities/<group>/<name>/`, as the
/// loaders find it; `known` holds every file of the release.
fn classify(parts: &[&str], known: &BTreeSet<&str>) -> Option<Located> {
    let area = match parts {
        ["manifests", "core", ..] => 2,
        ["manifests", "capabilities", _, _, ..] => 4,
        _ => return None,
    };
    let located = |kind, path_name: Option<&str>| Located {
        kind,
        path_name: path_name.map(str::to_owned),
    };
    match &parts[area..] {
        ["agents", file] => file
            .strip_suffix(".md")
            .map(|_| located(ArtifactKind::Role, None)),
        ["instructions", file] => file
            .strip_suffix(".instructions.md")
            .map(|stem| located(ArtifactKind::Instruction, Some(stem))),
        ["prompts", file] => file
            .strip_suffix(".md")
            .map(|stem| located(ArtifactKind::Prompt, Some(stem))),
        ["workflows", file] => file
            .strip_suffix(".yaml")
            .map(|_| located(ArtifactKind::Workflow, None)),
        ["themes", file] => file
            .strip_suffix(".json")
            .map(|_| located(ArtifactKind::Theme, None)),
        ["guards", file] => file
            .strip_suffix(".yaml")
            .map(|_| located(ArtifactKind::GuardRule, None)),
        ["skills", inside @ ..] => skill_folder(&parts[..=area], inside, known)
            .map(|folder| located(ArtifactKind::Skill, Some(folder))),
        _ => None,
    }
}

/// The folder that names the skill at `inside`, the path under the skills root
/// `root`, when the skill loader finds a skill there: a `SKILL.md` that no
/// ancestor `SKILL.md` encloses, or a Markdown file directly in the skills root
/// when the root has no `SKILL.md`. The loader skips hidden entries and
/// `node_modules`.
fn skill_folder<'a>(
    root: &[&'a str],
    inside: &[&'a str],
    known: &BTreeSet<&str>,
) -> Option<&'a str> {
    let hidden = |part: &str| part.starts_with('.') || part == "node_modules";
    let (file, folders) = inside.split_last()?;
    let mut directory = root.join("/");
    if *file != "SKILL.md" {
        let direct = folders.is_empty() && !hidden(file) && file.strip_suffix(".md").is_some();
        let enclosed = known.contains(format!("{directory}/SKILL.md").as_str());
        return if direct && !enclosed {
            root.last().copied()
        } else {
            None
        };
    }
    for folder in folders {
        if hidden(folder) || known.contains(format!("{directory}/SKILL.md").as_str()) {
            return None;
        }
        directory = format!("{directory}/{folder}");
    }

    folders.last().or(root.last()).copied()
}

/// Reads the `name` of the capability whose `package.json` is at `path`.
fn capability_name(root: &str, path: &str) -> Result<String, Error> {
    let package = reading::parse_json(path, &reading::read(root, path)?)?;

    package["name"]
        .as_str()
        .and_then(name_of)
        .ok_or_else(|| Error::MissingName {
            path: path.to_owned(),
            key: "name",
        })
}

/// Reads the artifacts at `path`: one, or for a guard file one per rule.
fn load_artifacts(root: &str, path: &str, located: Located) -> Result<Vec<Artifact>, Error> {
    let text = reading::read(root, path)?;
    if located.kind == ArtifactKind::GuardRule {
        return guard_rules(path, &text);
    }

    Ok(vec![load_artifact(path, &text, located)?])
}

/// Reads the one artifact at `path` from its `text`, with its name and comparable content.
fn load_artifact(path: &str, text: &str, located: Located) -> Result<Artifact, Error> {
    let yaml = |source| Error::Yaml {
        path: path.to_owned(),
        source,
    };
    let (named, content) = match located.kind {
        ArtifactKind::Theme => theme_content(path, text)?,
        ArtifactKind::Workflow => {
            let (name, head) = take_key(first_document(text).map_err(yaml)?, "name");
            (
                name,
                Content {
                    head,
                    body: String::new(),
                },
            )
        }
        _ => {
            // A file with no frontmatter block has an empty frontmatter.
            let (frontmatter, body) = reading::split_frontmatter(text).unwrap_or(("", text));
            let (name, head) = take_key(first_document(frontmatter).map_err(yaml)?, "name");
            let body = reading::collapse_whitespace(body);
            (name, Content { head, body })
        }
    };
    let named_by_file = matches!(
        located.kind,
        ArtifactKind::Instruction | ArtifactKind::Prompt
    );
    let name = if named_by_file {
        located.path_name
    } else {
        named.or(located.path_name)
    };

    Ok(Artifact {
        kind: located.kind,
        name: name.ok_or_else(|| Error::MissingName {
            path: path.to_owned(),
            key: "name",
        })?,
        path: path.to_owned(),
        content,
    })
}

/// Reads the rules of the guard file at `path`; each is named by its `id`.
fn guard_rules(path: &str, text: &str) -> Result<Vec<Artifact>, Error> {
    let document = first_document(text).map_err(|source| Error::Yaml {
        path: path.to_owned(),
        source,
    })?;

    document["rules"]
        .as_vec()
        .into_iter()
        .flatten()
        .map(|rule| {
            let (id, head) = take_key(rule.clone(), "id");
            let id = id.ok_or_else(|| Error::MissingName {
                path: path.to_owned(),
                key: "id",
            })?;
            Ok(Artifact {
                kind: ArtifactKind::GuardRule,
                path: format!("{path}#{id}"),
                name: id,
                content: Content {
                    head,
                    body: String::new(),
                },
            })
        })
        .collect()
}

/// Reads a theme's `name` and the rest of its JSON as comparable content.
fn theme_content(path: &str, text: &str) -> Result<(Option<String>, Content), Error> {
    let mut theme = reading::parse_json(path, text)?;
    let name = theme
        .as_object_mut()
        .and_then(|fields| fields.shift_remove("name"))
        .and_then(|name| name.as_str().and_then(name_of));

    Ok((
        name,
        Content {
            head: Yaml::Null,
            body: theme.to_string(),
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

/// The name `value` gives an artifact; an empty value gives none, for every
/// kind and every reader.
fn name_of(value: &str) -> Option<String> {
    (!value.is_empty()).then(|| value.to_owned())
}

/// Removes the string `key` from a YAML mapping and returns it with the rest;
/// a missing, non-string or empty value comes back as `None`.
fn take_key(document: Yaml, key: &str) -> (Option<String>, Yaml) {
    match document {
        Yaml::Hash(mut fields) => {
            let value = fields
                .remove(&Yaml::String(key.to_owned()))
                .and_then(|value| value.as_str().and_then(name_of));
            (value, Yaml::Hash(fields))
        }
        other => (None, other),
    }
}
