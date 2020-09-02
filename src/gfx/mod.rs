//! 2D graphics API

pub mod batcher;
pub mod pipeline;
pub mod vertices;

mod texture;
pub use texture::Texture2D;

pub use batcher::Batcher;
pub use pipeline::Pipeline;

/// The main interface to render 2D sprites
///
/// * TODO: drop `Device`
/// * TODO: better push API
pub struct DrawContext {
    pub(crate) device: fna3d::Device,
    pub batcher: Batcher,
    pub pipe: Pipeline,
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
