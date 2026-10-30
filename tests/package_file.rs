//! Checks the release's package file.

use std::fs;
use std::path::Path;

use maestro_manifests_tests::MANIFEST_ROOT;
use regex::Regex;
use serde_json::Value;

/// Returns the package file's top-level `name` when it is in the organization scope.
fn organization_package_name(text: &str) -> Result<String, String> {
    let package: Value = serde_json::from_str(text)
        .map_err(|error| format!("package.json is not valid JSON: {error}"))?;
    let scoped =
        Regex::new(r"^@orchestration-maestro/[a-z-]+$").map_err(|error| error.to_string())?;
    package
        .get("name")
        .and_then(Value::as_str)
        .filter(|name| scoped.is_match(name))
        .map(str::to_owned)
        .ok_or_else(|| "package.json names no @orchestration-maestro package".to_owned())
}

/// The package file under the manifest root names `@orchestration-maestro/maestro-manifests`.
#[test]
fn package_file_names_maestro_manifests() -> Result<(), String> {
    let path = Path::new(MANIFEST_ROOT).join("package.json");
    let text = fs::read_to_string(&path).map_err(|error| format!("{}: {error}", path.display()))?;

    let name = organization_package_name(&text)?;

    assert_eq!(name, "@orchestration-maestro/maestro-manifests");
    Ok(())
}

/// A scoped `name` nested below the top level does not make the package scoped.
#[test]
fn nested_name_does_not_name_the_package() {
    let text = r#"{"name": "unscoped", "maestro": {"name": "@orchestration-maestro/nested"}}"#;

    let result = organization_package_name(text);

    assert_eq!(
        result,
        Err("package.json names no @orchestration-maestro package".to_owned())
    );
}

/// A package file that is not JSON is reported as such, not searched as text.
#[test]
fn text_that_is_not_json_is_rejected() {
    let text = r#"name = "@orchestration-maestro/not-json""#;

    let result = organization_package_name(text);

    assert!(result.is_err_and(|message| message.contains("not valid JSON")));
}
