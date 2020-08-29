//! 2D graphics
//!
//! Call `init` to begin with (or you can't do other than clear screen even if you don't see
//! erroors).

pub mod batcher;
pub mod pipeline;
pub mod vertices;

mod texture;
pub use texture::Texture2D;

pub use batcher::Batcher;
pub use pipeline::Pipeline;

/// The first thing to call after making `gfx::Device`
///
/// FNA3D requires us to set viewport and rasterizer state first and **if this is skipped, we
/// can't draw sprites** (without any error).
pub fn init(
    device: &mut fna3d::Device,
    // batcher: &mut Batcher,
    params: &fna3d::PresentationParameters,
) {
    let viewport = fna3d::Viewport {
        x: 0,
        y: 0,
        w: params.backBufferWidth as i32,
        h: params.backBufferHeight as i32,
        minDepth: 0.0,
        maxDepth: 1.0,
    };
    device.set_viewport(&viewport);

    // set material
    {
        let bst = fna3d::BlendState::alpha_blend();
        device.set_blend_state(&bst);
        // let dsst = fna3d::DepthStencilState::default();
        // device.set_depth_stencil_state(&dsst);
        let rst = fna3d::RasterizerState::default();
        device.apply_rasterizer_state(&rst);
    }
}

/// Swaps the front/back buffers
pub fn end_frame(device: &mut fna3d::Device, win: *mut std::ffi::c_void) {
    // batcher.flush(device, p);
    device.swap_buffers(None, None, win);
}

pub fn clear(device: &mut fna3d::Device, color: fna3d::Color) {
    device.clear(
        fna3d::ClearOptions::TARGET,
        fna3d::Color::cornflower_blue(),
        0.0,
        0,
    );
}

/// The main interface to render 2D sprites
pub struct DrawContext {
    pub device: fna3d::Device,
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

pub struct AssetStore {
    textures: std::collections::HashMap<String, Texture2D>,
}
