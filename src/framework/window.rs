//! SDL window and `fna3d::Device`

use sdl2::{sys::SDL_Window, EventPump};

/// Returns `(window, device, params): (SdlWindowHandle, fna3d::Device, fna3d::PresentationParameters)`
pub fn init(cfg: &AnfConfig) -> (WindowHandle, fna3d::Device, fna3d::PresentationParameters) {
    // setup FNA3D
    log::info!("FNA version {}", fna3d::linked_version());
    fna3d::utils::hook_log_functions_default();

    let win = WindowHandle::from_cfg(&cfg);
    let (params, device) = self::create_fna3d_device(cfg, win.raw_window());

    (win, device, params)
}

/// TODO: high DPI. TODO: more settings such as `d_debug`
pub struct AnfConfig {
    pub title: String,
    pub w: u32,
    pub h: u32,
}

impl AnfConfig {
    pub fn default() -> Self {
        AnfConfig {
            title: "† ANF game †".to_string(),
            w: 1280,
            h: 720,
        }
    }
}

/// Window object
///
/// The window is dropped when this handle goes out of scope.
pub struct WindowHandle {
    sdl: sdl2::Sdl,
    win: sdl2::video::Window,
}

impl WindowHandle {
    pub fn from_cfg(cfg: &AnfConfig) -> Self {
        let flags = fna3d::prepare_window_attributes();
        let sdl = sdl2::init().unwrap();
        let win = self::create_sdl_window(cfg, &sdl, flags.0);

        WindowHandle { sdl, win }
    }

    pub fn screen_size(&self) -> [u32; 2] {
        let size = self.win.size();
        [size.0, size.1]
    }

    pub fn set_screen_size(&mut self, size: [u32; 2], dcx: &mut crate::gfx::api::DrawContext) {
        self.win.set_size(size[0], size[1]).unwrap();
        dcx.params.backBufferWidth = size[0] as i32;
        dcx.params.backBufferHeight = size[1] as i32;
        dcx.device.reset_backbuffer(&dcx.params);
    }

    pub(crate) fn raw_window(&self) -> *mut SDL_Window {
        self.win.raw()
    }

    pub(crate) fn event_pump(&mut self) -> Result<EventPump, String> {
        self.sdl.event_pump()
    }
}

// --------------------------------------------------------------------------------
// Utilities

fn create_sdl_window(cfg: &AnfConfig, sdl: &sdl2::Sdl, flags: u32) -> sdl2::video::Window {
    let video = sdl.video().unwrap();
    video
        .window(&cfg.title, cfg.w, cfg.h)
        .set_window_flags(flags)
        .position_centered()
        .build()
        .unwrap()
}

fn create_fna3d_device(
    cfg: &AnfConfig,
    win: *mut SDL_Window,
) -> (fna3d::PresentationParameters, fna3d::Device) {
    let params = {
        let mut params = fna3d::utils::default_params_from_window_handle(win as *mut _);
        params.backBufferWidth = cfg.w as i32;
        params.backBufferHeight = cfg.h as i32;
        params
    };
    let do_debug = true; // FIXME: inject
    let mut device = fna3d::Device::from_params(params, do_debug);
    init_device(&mut device, &params);
    return (params, device);

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
