//! Graphics

pub mod batch;
pub mod texture;
pub mod vertices;

use batch::Batcher;

// FIXME: this may be nonsense
/// The first thing to call before using `fna3d::Device`
pub fn init(
    device: &mut fna3d::Device,
    batcher: &mut Batcher,
    params: &fna3d::PresentationParameters,
) {
    // set default render state
    let blend = fna3d::BlendState::alpha_blend();
    device.set_blend_state(&blend);
    let rst = fna3d::RasterizerState::default();
    device.apply_rasterizer_state(&rst);
    let dsst = fna3d::DepthStencilState::default();
    device.set_depth_stencil_state(&dsst);

    let viewport = fna3d::Viewport {
        x: 0,
        y: 0,
        w: params.backBufferWidth as i32,
        h: params.backBufferHeight as i32,
        minDepth: 0 as f32,
        maxDepth: 1 as f32,
    };
    device.set_viewport(&viewport)

    // device.set_render_targets(
    //     Some(&mut batcher.v_binds.bind),
    //     1,
    //     None, // FIXME: DepthStencilBuffer
    //     fna3d::DepthFormat::D24S8,
    // );
}

/// `FNA3D_BeginFrame`
pub fn begin_frame(device: &mut fna3d::Device) {
    device.begin_frame();
}

/// Makes sure the `Batcher` flushes and swaps buffers
pub fn end_frame(device: &mut fna3d::Device, batcher: &mut Batcher) {
    batcher.flush(device);
    device.swap_buffers(None, None, batcher.win);
}

/// Clears the active draw buffers with cornflower blue i.e. (r, g, b) = (100, 149, 237)
pub fn clear(device: &mut fna3d::Device) {
    let color = fna3d::Color {
        r: 100,
        g: 149,
        b: 237,
        a: 0,
    };
    device.clear(fna3d::ClearOptions::Target, color, 0.0, 0);
}
