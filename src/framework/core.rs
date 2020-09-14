//! Window and graphics device

use sdl2::{sys::SDL_Window, EventPump};

pub fn anf_create_core(
    cfg: &AnfConfig,
) -> (
    SdlWindowHandle,
    fna3d::Device,
    fna3d::PresentationParameters,
) {
    // setup FNA3D
    log::info!("FNA version {}", fna3d::linked_version());
    fna3d::utils::hook_log_functions_default();

    let win = SdlWindowHandle::from_cfg(&cfg);
    let (params, mut device) = cfg.device(win.raw_window);
    self::init_device(&mut device, &params);

    (win, device, params)
}

/// TODO: high DPI
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

    fn window(&self, sdl: &sdl2::Sdl, flags: u32) -> sdl2::video::Window {
        let video = sdl.video().unwrap();
        video
            .window(&self.title, self.w, self.h)
            .set_window_flags(flags)
            .position_centered()
            .build()
            .unwrap()
    }

    fn device(&self, win: *mut SDL_Window) -> (fna3d::PresentationParameters, fna3d::Device) {
        let params = {
            let mut params = fna3d::utils::default_params_from_window_handle(win as *mut _);
            params.backBufferWidth = self.w as i32;
            params.backBufferHeight = self.h as i32;
            params
        };
        let device = fna3d::Device::from_params(params, true);
        (params, device)
    }
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

/// Hides the use of SDL2 (Rust-SDL2)
///
/// The window is dropped when this handle goes out of scope.
pub struct SdlWindowHandle {
    sdl: sdl2::Sdl,
    win: sdl2::video::Window,
    raw_window: *mut SDL_Window,
}

impl SdlWindowHandle {
    // TODO: do wee need canavs?
    pub fn from_cfg(cfg: &AnfConfig) -> Self {
        let flags = fna3d::prepare_window_attributes();

        let sdl = sdl2::init().unwrap();
        let win = cfg.window(&sdl, flags.0);
        let raw_window = win.raw();

        SdlWindowHandle {
            sdl,
            raw_window,
            win,
        }
    }

    pub fn raw_window(&self) -> *mut SDL_Window {
        self.win.raw()
    }

    pub fn event_pump(&mut self) -> Result<EventPump, String> {
        self.sdl.event_pump()
    }
}
