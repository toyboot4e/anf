//! Quad rendering internals
//!
//! * TODO: make a more fluent sprite push API
//! * TODO: flush on push if it's out of capacity

pub mod batch;
pub mod buffers;
pub mod bufspecs;
pub mod primitives;

mod batch_push;
pub use batch_push::{DrawPolicy, SpritePushCommand};

mod batcher;
pub use batcher::Batcher;

/// Begins a builder to push a sprite to `Batcher`
pub fn push() -> SpritePushCommand {
    SpritePushCommand::default()
}
