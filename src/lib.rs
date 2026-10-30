//! Checks for the manifest resources in this repository.
//!
//! The crate's integration tests read the files under [`MANIFEST_ROOT`] and
//! check them without running Maestro.

/// The manifest root, `manifests/`, which holds every file Maestro reads from a release.
pub const MANIFEST_ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "");
