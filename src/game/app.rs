//! SDL window and FNA3D device
//!
//! # Example
//!
//! Making SDL window with FNA3D device:
//!
//! ```
//! use anf::app::{init_app, WindowConfig};
//!
//! let cfg = WindowConfig::default();
//! let (win, device, params) = init_app(&cfg);
//! ```

use std::ffi::NulError;

use sdl2::{
    sys::SDL_Window,
    video::{FullscreenType, WindowPos},
    EventPump, IntegerOrSdlError,
};

/// Returns `(window, device, params): (SdlWindowHandle, fna3d::Device, fna3d::PresentationParameters)`
///
/// The device is initialized with:
///
/// * pre-multiplied alpha `BlendState`
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

        // multiplied alpha blend
        let bst = fna3d::BlendState::alpha_blend();
        device.set_blend_state(&bst);
    }
}

/// Initial settings of the window
pub struct WindowConfig {
    pub title: String,
    pub w: u32,
    pub h: u32,
    pub is_debug: bool,
    pub rm_decoration: bool,
    pub allow_high_dpi: bool,
    pub is_resizable: bool,
}

impl Default for WindowConfig {
    fn default() -> Self {
        WindowConfig {
            title: "† ANF game †".to_string(),
            w: 1280,
            h: 720,
            is_debug: true,
            rm_decoration: false,
            allow_high_dpi: true,
            is_resizable: false,
        }
    }
}

/// Owner of SDL2 window
///
/// The window is dropped when this handle goes out of scope.
pub struct WindowHandle {
    pub sdl: sdl2::Sdl,
    pub win: sdl2::video::Window,
}

impl AsRef<sdl2::video::Window> for WindowHandle {
    fn as_ref(&self) -> &sdl2::video::Window {
        &self.win
    }
}

// TODOs:
// set icon: https://github.com/FNA-XNA/FNA/blob/master/src/FNAPlatform/SDL2_FNAPlatform.cs#L534
// display orientation
impl WindowHandle {
    pub fn from_cfg(cfg: &WindowConfig) -> Self {
        let flags = fna3d::prepare_window_attributes();
        let sdl = sdl2::init().unwrap();
        let win = self::create_sdl_window(cfg, &sdl, flags.0);

        WindowHandle { sdl, win }
    }

    pub(crate) fn raw_window(&self) -> *mut SDL_Window {
        self.win.raw()
    }

    pub(crate) fn event_pump(&mut self) -> Result<EventPump, String> {
        self.sdl.event_pump()
    }

    pub fn screen_size(&self) -> (u32, u32) {
        self.win.size()
    }
    pub fn set_screen_size(
        &mut self,
        size: [u32; 2],
        device: &mut fna3d::Device,
        params: &mut fna3d::PresentationParameters,
    ) {
        self.win.set_size(size[0], size[1]).unwrap();
        params.backBufferWidth = size[0] as i32;
        params.backBufferHeight = size[1] as i32;
        device.reset_backbuffer(&params);
    }

    pub fn title(&self) -> &str {
        self.win.title()
    }
    pub fn set_title(&mut self, title: &str) -> Result<(), NulError> {
        self.win.set_title(title)
    }

    pub fn position(&self) -> (i32, i32) {
        self.win.position()
    }
    pub fn set_position(&mut self, x: WindowPos, y: WindowPos) {
        self.win.set_position(x, y);
    }

    pub fn min_size(&self) -> (u32, u32) {
        self.win.minimum_size()
    }
    pub fn set_max_size(&mut self, width: u32, height: u32) -> Result<(), IntegerOrSdlError> {
        self.win.set_maximum_size(width, height)
    }

    pub fn minimize(&mut self) {
        self.win.minimize();
    }
    pub fn maximize(&mut self) {
        self.win.maximize();
    }

    pub fn show(&mut self) {
        self.win.show();
    }
    pub fn hide(&mut self) {
        self.win.hide();
    }
    pub fn fullscreen_state(&self) -> FullscreenType {
        self.win.fullscreen_state()
    }

    pub fn set_opacity(&mut self, opacity: f32) -> Result<(), String> {
        self.win.set_opacity(opacity)
    }
    pub fn opacity(&self) -> Result<f32, String> {
        self.win.opacity()
    }
}

fn create_sdl_window(cfg: &WindowConfig, sdl: &sdl2::Sdl, flags: u32) -> sdl2::video::Window {
    let video = sdl.video().unwrap();
    let mut window = video.window(&cfg.title, cfg.w, cfg.h);
    window.set_window_flags(flags).position_centered();
    if cfg.rm_decoration {
        window.borderless();
    }
    if cfg.is_resizable {
        window.resizable();
    }
    if cfg.allow_high_dpi {
        window.allow_highdpi();
    }
    window.build().unwrap()
}
