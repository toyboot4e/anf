//! 2D quad rendering
//!
//! * TODO: consider using a matrix crate e.g. [mint](https://docs.rs/mint/) or glam
//! * TODO: use callback to create user data: F: 'static + FnOnce(Context) -> UserData
//! * TODO: event handling

pub mod batcher;
pub mod buffers;
pub mod pipeline;

mod texture;
pub use texture::Texture2D;

/// Render sprites! Often referred to as `dcx`
///
/// The name `dcx` follows the rustc [naming convension] (though I often see `ctx` even in rustc).
///
/// [naming convension]: https://rustc-dev-guide.rust-lang.org/conventions.html#naming-conventions
///
/// * TODO: drop `Device`
/// * TODO: better push API
pub struct DrawContext {
    pub(crate) device: fna3d::Device,
    pub batcher: batcher::Batcher,
    pub pipe: pipeline::Pipeline,
}

impl DrawContext {
    pub fn begin(&mut self) {
        self.batcher.begin(&mut self.device);
    }

    /// Ends the pass and flushes batch data to actually draw to a render target
    pub fn end(&mut self) {
        self.batcher.end(&mut self.device, &mut self.pipe);
    }
}
