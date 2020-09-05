//! Application, the game loop
//!
//! # Getting started
//!
//! This is a hello-world program:
//!
//! ```no_run
//! // main.rs or bin.rs side
//! use anf::app::{App, AppConfig, AppResult};
//!
//! fn main() -> AppResult {
//!     let cfg = AppConfig::default();
//!     let app = App::from_cfg(cfg);
//!     let state = MyAppState {};
//!     app.run(state)
//! }
//!
//! // lib.rs side
//! use anf::{app::AppState, gfx::DrawContext};
//! use anf::fna3d::Color;
//!
//! struct MyAppState {}
//!
//! impl AppState for MyAppState {
//!     fn update(&mut self) {}
//!     fn render(&mut self, dcx: &mut DrawContext) {
//!         anf::gfx::clear_frame(dcx, Color::cornflower_blue());
//!     }
//! }
//! ```
//!
//! Your screen will be filled with [cornflower blue] pixels. Feel like you're home -- you're
//! welcome :)
//!
//! See the [examples] for more information.
//!
//! [cornflower blue]: https://www.google.com/search?q=cornflower%20blue
//! [examples]: https://github.com/toyboot4e/anf/examples

use crate::{gfx::DrawContext, vfs};
use anf_gfx::{batcher::Batcher, pipeline::Pipeline};

use anf_deps::{fna3d, sdl2};
use fna3d::Device;
use sdl2::{
    render::WindowCanvas,
    {event::Event, keyboard::Keycode},
};

use std::time::Duration;

/// User data driven by `AppImpl`
pub trait AppState {
    fn update(&mut self) {}
    /// Clear and render next frame
    #[allow(unused_variables)]
    fn render(&mut self, dcx: &mut DrawContext) {}
}

/// Data to create `App`
///
/// It only contains initial window settings (for now).
///
/// * TODO: high DPI
pub struct AppConfig {
    pub title: String,
    pub w: u32,
    pub h: u32,
}

/// Data to run application
///
/// Internally, it's using [Rust-SDL2] to make, hold and drop window.
///
/// [Rust-SDL2]: https://github.com/Rust-SDL2/rust-sdl2
pub struct App {
    win: SdlWindow,
    pub device: Device,
    pub params: fna3d::PresentationParameters,
}

/// Hides the use of SDL2 (Rust-SDL2)
///
/// The window is dropped when this handle goes out of scope.
struct SdlWindow {
    pub sdl: sdl2::Sdl,
    pub raw_window: *mut sdl2::sys::SDL_Window,
    pub canvas: WindowCanvas,
}

impl App {
    pub fn from_cfg(cfg: AppConfig) -> Self {
        log::info!("FNA version {}", fna3d::linked_version());

        let flags = fna3d::prepare_window_attributes();
        let sdl = sdl2::init().unwrap();
        let canvas = cfg.canvas(&sdl, flags.0);
        let raw_window = canvas.window().raw();
        let (params, device) = cfg.device(raw_window as *mut _);

        App {
            win: SdlWindow {
                sdl,
                raw_window,
                canvas,
            },
            params,
            device,
        }
    }

    pub fn run<T: AppState>(self, state: T) -> AppResult {
        AppImpl::new(state, self).run()
    }
}

enum UpdateResult {
    Continue,
    Quit,
}

/// Return value of `App::run`
pub type AppResult = std::result::Result<(), Box<dyn std::error::Error>>;

impl AppConfig {
    pub fn default() -> Self {
        Self {
            title: "† ANF game †".to_string(),
            w: 1280,
            h: 720,
        }
    }
}

/// Dirty creation methods based on Rust-SDL2 and Rust-FNA3D
impl AppConfig {
    fn canvas(&self, sdl: &sdl2::Sdl, flags: u32) -> WindowCanvas {
        let video = sdl.video().unwrap();
        let win = self.window(video, flags);
        win.into_canvas().build().unwrap()
    }

    fn window(&self, video: sdl2::VideoSubsystem, flags: u32) -> sdl2::video::Window {
        video
            .window(&self.title, self.w, self.h)
            .set_window_flags(flags)
            .position_centered()
            .build()
            .unwrap()
    }

    fn device(&self, win: *mut std::ffi::c_void) -> (fna3d::PresentationParameters, fna3d::Device) {
        let params = {
            let mut params = fna3d::utils::default_params_from_window_handle(win);
            params.backBufferWidth = self.w as i32;
            params.backBufferHeight = self.h as i32;
            params
        };
        let device = fna3d::Device::from_params(params, true);
        (params, device)
    }
}

/// Application state that drives user state
struct AppImpl<T: AppState> {
    dcx: DrawContext,
    state: T,
    win: SdlWindow,
}

impl<T: AppState> AppImpl<T> {
    pub fn new(state: T, mut src: App) -> Self {
        fna3d::utils::hook_log_functions_default();
        Self::init_gfx(&mut src.device, &src.params);

        let pipe = Pipeline::from_device(&mut src.device, vfs::default_shader());
        let batcher = Batcher::from_device(&mut src.device);

        AppImpl {
            dcx: DrawContext::new(src.device, batcher, pipe),
            state,
            win: src.win,
        }
    }

    /// Initializes the graphics devices
    ///
    /// FNA3D requires us to set viewport/rasterizer/blend state. **If this is skipped, we can't
    /// draw anything** (we only can clear the screen)
    fn init_gfx(
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

        // let dst = fna3d::DepthStencilState::default();
        // device.set_depth_stencil_state(&dst);
    }

    pub fn run(mut self) -> AppResult {
        let mut events = self.win.sdl.event_pump().unwrap();
        log::trace!("Start ANF game loop");

        'main_loop: loop {
            // pump events
            for ev in events.poll_iter() {
                match self.handle_event(&ev) {
                    UpdateResult::Quit => break 'main_loop,
                    UpdateResult::Continue => {}
                }
            }

            self.update();
            self.render();

            // FIXME: timestep handling & `GameTime`
            let fps = 60;
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / fps));
        }

        Ok(())
    }
}

impl<T: AppState> AppImpl<T> {
    /// Does nothing for now
    fn update(&mut self) {
        self.state.update();
    }

    /// Runs the rendering pipeline
    fn render(&mut self) {
        self.state.render(&mut self.dcx);
        self.dcx
            .device
            .swap_buffers(None, None, self.win.raw_window as *mut _);
    }

    /// Just quits on `Escape` key down
    fn handle_event(&mut self, ev: &sdl2::event::Event) -> UpdateResult {
        match ev {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => UpdateResult::Quit,
            _ => UpdateResult::Continue,
        }
    }
}
