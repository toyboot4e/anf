//! Time, window and device
//!
//! This is basically internals of [`crate::game::AnfFramework`]

mod app;
mod time;
mod window;

pub use self::{
    app::{AnfApp, AnfAppLifecycle, AnfAppResult},
    time::{TargetFps, TimeStep},
    window::{WindowConfig, WindowHandle},
};

use sdl2::sys::SDL_Window;

/// Returns `(window, device, params): (SdlWindowHandle, fna3d::Device, fna3d::PresentationParameters)`
pub fn init_app(
    cfg: &WindowConfig,
) -> (WindowHandle, fna3d::Device, fna3d::PresentationParameters) {
    // setup FNA3D
    log::info!("FNA version {}", fna3d::linked_version());
    fna3d::utils::hook_log_functions_default();

    let win = WindowHandle::from_cfg(&cfg);
    let (params, device) = create_fna3d_device(cfg, win.raw_window());
    return (win, device, params);

    fn create_fna3d_device(
        cfg: &WindowConfig,
        raw_window: *mut SDL_Window,
    ) -> (fna3d::PresentationParameters, fna3d::Device) {
        let params = {
            let mut params = fna3d::utils::default_params_from_window_handle(raw_window as *mut _);
            params.backBufferWidth = cfg.w as i32;
            params.backBufferHeight = cfg.h as i32;
            params
        };
        let mut device = fna3d::Device::from_params(params, cfg.is_debug);
        init_device(&mut device, &params);
        (params, device)
    }

    /// Initializes the graphics devices
    ///
    /// FNA3D requires us to set viewport/rasterizer/blend state. **If this is skipped, we can't
    /// draw anything** (we only can clear the screen)
    fn init_device(
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
            maxDepth: 1.0, // TODO: what's this
        };
        device.set_viewport(&viewport);

        let rst = fna3d::RasterizerState::default();
        device.apply_rasterizer_state(&rst);

        let bst = fna3d::BlendState::alpha_blend();
        device.set_blend_state(&bst);
    }
}
