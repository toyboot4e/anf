//! Sprite batch
//!
//! Corresponds to both `GraphicsDevice` and `SpriteBatch`

// sub modules
pub mod batch_data;
mod batch_internals;
pub mod buffers;
pub mod shader;

// re-exported to root
mod batch_push;
pub use batch_push::{DrawPolicy, SpritePushCommand};
mod batcher;
pub use batcher::Batcher;

// TODO: make a more fluent API
// TODO: add begin guard
// TODO: flush if it's out of capacity

/// Begins a builder to push a sprite to `Batcher`
pub fn push() -> SpritePushCommand {
    batch_push::SpritePushCommand::default()
}
