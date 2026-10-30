//! Checks for the manifest resources in this repository.
//!
//! The crate's integration tests read the files under [`ROOT`] and check them
//! without running Maestro.

/// The repository root, from which every checked path is relative.
pub const ROOT: &str = env!("CARGO_MANIFEST_DIR");
