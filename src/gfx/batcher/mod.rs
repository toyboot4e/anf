//! The main interface to render 2D sprites
//!
//! # Internals
//!
//! A sprite is a rectangle that is represented as two triangles. And they are four vertices and
//! six indices.
//!
//! * TODO: make a more fluent sprite push API
//! * TODO: flush on push if it's out of capacity

pub mod batch_data;

mod batch_push;
pub use batch_push::{DrawPolicy, SpritePushCommand};

pub mod buffers;

mod batcher;
pub use batcher::Batcher;

/// Begins a builder to push a sprite to `Batcher`
pub fn push() -> SpritePushCommand {
    SpritePushCommand::default()
}
