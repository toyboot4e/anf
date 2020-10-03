//! Virtual file system
//!
//! It just locates the root; not so much a file system for now.

use std::path::{Path, PathBuf};

// FIXME:
fn root() -> PathBuf {
    PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap() + "/assets")
}

/// Creates an absolute path to an asset file from a **relative path** to your `assets` directory.
pub fn path(p: impl AsRef<Path>) -> PathBuf {
    root().join(p.as_ref())
}
