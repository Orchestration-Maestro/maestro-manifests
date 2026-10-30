//! Checks the release's package file.

use serde_json::Value;

use crate::support::{Check, Failures, Report, parse_json, re, read};

/// The package file under the manifest root.
const PACKAGE: &str = "manifests/package.json";

/// Returns the package file's top-level `name` when it is in the organization scope.
fn organization_package_name(text: &str) -> Result<String, Report> {
    let package = parse_json(PACKAGE, text)?;
    let scoped = re(r"^@orchestration-maestro/[a-z-]+$")?;
    package
        .get("name")
        .and_then(Value::as_str)
        .filter(|name| scoped.is_match(name))
        .map(str::to_owned)
        .ok_or_else(|| {
            Report::new(format!(
                "{PACKAGE}: names no @orchestration-maestro package"
            ))
        })
}

/// The package file under the manifest root names `@orchestration-maestro/maestro-manifests`.
#[test]
fn package_file_names_maestro_manifests() -> Check {
    let mut failures = Failures::default();

    failures.require_eq(
        PACKAGE,
        &organization_package_name(&read(PACKAGE)?)?.as_str(),
        &"@orchestration-maestro/maestro-manifests",
    );

    failures.finish()
}

/// A scoped `name` nested below the top level does not make the package scoped.
#[test]
fn nested_name_does_not_name_the_package() -> Check {
    let mut failures = Failures::default();
    let text = r#"{"name": "unscoped", "maestro": {"name": "@orchestration-maestro/nested"}}"#;

    failures.require(
        organization_package_name(text).is_err(),
        PACKAGE,
        "takes a nested name",
    );

    failures.finish()
}

/// A package file that is not JSON is reported as such, not searched as text.
#[test]
fn text_that_is_not_json_is_rejected() -> Check {
    let mut failures = Failures::default();
    let text = r#"name = "@orchestration-maestro/not-json""#;

    failures.require(
        organization_package_name(text)
            .is_err_and(|report| format!("{report:?}").contains("not valid JSON")),
        PACKAGE,
        "searches the text",
    );

    failures.finish()
}
