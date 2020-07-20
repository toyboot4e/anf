//! Virtual file system which provides absolute path from `assets` directory
//!
//! Not so good yet

use std::path::{Path, PathBuf};

/// Get an absolute path to a file from a **relative path** from `assets` directory.
pub fn get(p: impl AsRef<Path>) -> PathBuf {
    // FIXME:
    let root = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap() + "/assets");
    root.join(p.as_ref())
}

/// Get an absolute path to a file from a **relative path** from `assets` directory.
pub fn default_shader() -> PathBuf {
    self::get("FNAEffects/SpriteEffect.fxb")
}
