//! Virtual file system
//!
//! It just locates the root; it's not so much a file system (for now).

use std::path::{Path, PathBuf};

// FIXME:
fn root() -> PathBuf {
    PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap() + "/assets")
}

/// Returns an absolute path to a file from a **relative path** from `assets` directory.
pub fn get(p: impl AsRef<Path>) -> PathBuf {
    root().join(p.as_ref())
}

/// Returns the path to the default shader file
pub fn default_shader() -> PathBuf {
    self::get("FNAEffects/SpriteEffect.fxb")
}
