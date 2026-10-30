//! Checks the release's package file.

use crate::support::{Check, Failures, read};

/// The package file under the manifest root names a package in the organization scope.
#[test]
fn package_file_names_a_package_in_the_organization_scope() -> Check {
    let mut failures = Failures::default();
    let path = "manifests/package.json";

    failures.matches(
        path,
        &read(path)?,
        r#""name":\s*"@orchestration-maestro/[a-z-]+""#,
    )?;

    failures.finish()
}
