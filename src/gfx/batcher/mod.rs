//! Internals of quad rendering
//!
//! * TODO: make a more fluent sprite pushing API
//! * TODO: flush on push if it's out of capacity

pub mod bufspecs;
pub mod data;
pub mod primitives;

mod batch_push;
pub use batch_push::{DrawPolicy, SpritePushCommand};

mod batcher;
pub use batcher::Batcher;

/// Begins a builder to push a sprite to `Batcher`
pub fn push() -> SpritePushCommand {
    SpritePushCommand::default()
}
