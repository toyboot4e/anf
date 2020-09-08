use sdl2::sys::SDL_Window;

/// Data to create [`App`]
///
/// It only contains initial window settings (for now).
///
/// * TODO: high DPI
pub struct AppConfig {
    pub title: String,
    pub w: u32,
    pub h: u32,
}

impl AppConfig {
    pub fn default() -> Self {
        Self {
            title: "† ANF game †".to_string(),
            w: 1280,
            h: 720,
        }
    }
}

pub struct App {
    pub win: SdlWindowHandle,
    pub device: fna3d::Device,
    pub params: fna3d::PresentationParameters,
}

/// Window & device creation
/// ---
impl App {
    /// Creates the `App` = window + device
    pub fn from_cfg(cfg: AppConfig) -> Self {
        // setup FNA3D
        log::info!("FNA version {}", fna3d::linked_version());
        fna3d::utils::hook_log_functions_default();

        let win = SdlWindowHandle::from_cfg(&cfg);
        let (params, device) = cfg.device(win.raw_window);

        App {
            win,
            params,
            device,
        }
    }

    pub fn raw_window(&self) -> *mut SDL_Window {
        self.win.raw_window
    }
}

/// Hides the use of SDL2 (Rust-SDL2)
///
/// The window is dropped when this handle goes out of scope.
pub struct SdlWindowHandle {
    pub sdl: sdl2::Sdl,
    pub win: sdl2::video::Window,
    pub raw_window: *mut SDL_Window,
}

impl SdlWindowHandle {
    // TODO: do wee need canavs?
    pub fn from_cfg(cfg: &AppConfig) -> Self {
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
}

/// Creation methods based on Rust-SDL2 and Rust-FNA3D
impl AppConfig {
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
