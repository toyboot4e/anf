//! `Batcher` is the main interface for users to render 2D sprites
//!
//! Corresponds to both `GraphicsDevice` and `SpriteBatch` in FNA.

pub mod batch;
pub use batch::batch_push::{DrawPolicy, SpritePushCommand};

pub mod buffers;
pub mod shader;

mod batcher;
pub use batcher::Batcher;

// TODO: make a more fluent API
// TODO: add begin guard
// TODO: flush if it's out of capacity

/// Begins a builder to push a sprite to `Batcher`
pub fn push() -> SpritePushCommand {
    SpritePushCommand::default()
}
