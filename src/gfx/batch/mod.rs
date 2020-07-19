//! Sprite batch
//!
//! Corresponds to both `GraphicsDevice` and `SpriteBatch`

pub mod batch_data;
mod batch_internals;
pub mod draw;
pub mod shader;

pub mod batch_push;
pub use batch_push::DrawPolicy;

pub mod batcher;
pub use batcher::Batcher;

// TODO: make a more fluent API
// TODO: add begin guard
// TODO: flush if it's out of capacity
pub fn push() -> batch_push::SpritePushCommand {
    batch_push::SpritePushCommand::default()
}
