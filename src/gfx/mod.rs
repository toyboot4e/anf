//! 2D graphics
//!
//! Call `init` to begin with (or you can't do other than clear screen even if you don't see
//! erroors).
//!
//! # The rendering cycle
//!
//! `begin_frame` → [`Batcher::flush`] → `end_frame`
//!
//! [`Batcher::flush`]: ./batcher/struct.Batcher.html#method.flush

pub mod batcher;
pub mod pipeline;
pub mod vertices;

mod texture;
pub use texture::Texture2D;

use batcher::Batcher;
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
        minDepth: 0 as f32,
        maxDepth: 1 as f32,
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

/// `FNA3D_BeginFrame`
pub fn begin_frame(device: &mut fna3d::Device) {
    device.begin_frame();
}

/// Swaps the front/back buffers
pub fn end_frame(device: &mut fna3d::Device, p: &mut Pipeline, batcher: &mut Batcher) {
    // batcher.flush(device, p);
    device.swap_buffers(None, None, batcher.win);
}

/// Clears the active draw buffers with cornflower blue i.e. (r, g, b) = (100, 149, 237)
pub fn clear(device: &mut fna3d::Device) {
    device.clear(
        fna3d::ClearOptions::TARGET,
        fna3d::colors::CORNFLOWER_BLUE,
        0.0,
        0,
    );
}
