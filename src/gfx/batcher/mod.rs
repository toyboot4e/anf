//! The main interface to render 2D sprites

pub mod batch_data;

mod batch_push;
pub use batch_push::{DrawPolicy, SpritePushCommand};

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
