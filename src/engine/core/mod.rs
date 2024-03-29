//! Core of the ANF engine

pub mod clock;
pub mod lifecycle;
pub mod window;

use {
    fna3h::{
        draw::{blend::BlendState, pip::RasterizerState, Viewport},
        win::PresentationParameters,
        Device,
    },
    sdl2::sys::SDL_Window,
};

use self::window::{WindowConfig, WindowHandle};

/// Returns `(window, device, params): (WindowHandle, Device, PresentationParameters)`
///
/// The `device` is set initial states so that it can soon be used:
///
/// * pre-multiplied alpha `BlendState`
/// * viewport with size of the screen
///
/// TODO: note about the presentation parameters
fn init(cfg: &WindowConfig) -> (WindowHandle, Device, PresentationParameters) {
    // setup FNA3D
    log::info!("FNA version {}", fna3h::fna3d::linked_version());
    fna3h::win::hook_log_functions_default();

    let win = WindowHandle::from_cfg(&cfg);
    let (params, device) = self::create_fna3d_device(cfg, win.raw_window());

    (win, device, params)
}

fn create_fna3d_device(
    cfg: &WindowConfig,
    raw_window: *mut SDL_Window,
) -> (PresentationParameters, Device) {
    let params = {
        let mut params =
            fna3h::fna3d::utils::default_params_from_window_handle(raw_window as *mut _);
        params.backBufferWidth = cfg.w as i32;
        params.backBufferHeight = cfg.h as i32;
        params
    };

    let mut device = Device::from_params(params, cfg.is_debug);
    self::init_device(&mut device, &params);

    (params, device)
}

/// Initializes the graphics devices
///
/// FNA3D requires us to set viewport/rasterizer/blend state. **If this is skipped, we can't
/// draw anything** (we only can clear the screen)
fn init_device(
    device: &Device,
    // batcher: &mut Batcher,
    params: &PresentationParameters,
) {
    let viewport = Viewport {
        x: 0,
        y: 0,
        w: params.backBufferWidth as i32,
        h: params.backBufferHeight as i32,
        minDepth: 0.0,
        maxDepth: 1.0, // TODO: what's this
    };
    device.set_viewport(&viewport);

    let rst = RasterizerState::default();
    device.apply_rasterizer_state(&rst);

    // multiplied alpha blend
    let bst = BlendState::alpha_blend();
    device.set_blend_state(&bst);
}
