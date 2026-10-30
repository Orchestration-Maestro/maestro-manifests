//! Checks the release's package file.

use std::fs;
use std::path::Path;

use maestro_manifests_tests::MANIFEST_ROOT;
use regex::Regex;

/// The package file under the manifest root names a package in the organization scope.
#[test]
fn package_file_names_a_package_in_the_organization_scope() -> Result<(), String> {
    let path = Path::new(MANIFEST_ROOT).join("package.json");
    let text = fs::read_to_string(&path).map_err(|error| format!("{}: {error}", path.display()))?;
    let name = Regex::new(r#""name":\s*"@orchestration-maestro/[a-z-]+""#)
        .map_err(|error| error.to_string())?;
    assert!(
        name.is_match(&text),
        "package.json names no @orchestration-maestro package"
    );
    Ok(())
}
