//! Internals of quad rendering
//!
//! * TODO: flush on push if it's out of capacity

pub mod batch;
pub mod bufspecs;
pub mod primitives;

mod batch_push;
pub use batch_push::{DrawPolicy, SpritePush};

mod batcher;
pub use batcher::Batcher;

/// Begins a builder to push a sprite to `Batcher`
pub fn push() -> SpritePush {
    SpritePush::default()
}
