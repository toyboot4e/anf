//! `Batcher` is the main interface to render 2D sprites
//!
//! Corresponds to both `GraphicsDevice` and `SpriteBatch` in FNA.

pub mod batch;
pub use batch::batch_push::{DrawPolicy, SpritePushCommand};

pub mod buffers;

mod batcher;
pub use batcher::Batcher;

// TODO: make a more fluent API.
//       batcher.begin(device).with_target(rt).rect(0,0,230,32).run()
// set render target to null means the draw call goes to screen

// TODO: flush if it's out of capacity

/// Begins a builder to push a sprite to `Batcher`
pub fn push() -> SpritePushCommand {
    SpritePushCommand::default()
}
