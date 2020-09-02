//! Bare-bone game loop
//!
//! # Boilerplate
//!
//! ```
//! use anf::app::{App, AppState, WindowConfig};
//!
//! struct MyAppState {}
//! impl AppState for MyAppState {}
//!
//! fn main() {
//!     let (window, device) = WindowConfig::default().create();
//!     let mut app = App::new(MyAppState {}, window, device);
//!
//!     match app.run() {
//!         Ok(()) => {}
//!         Err(why) => println!("Error occured: {}", why),
//!     };
//! }
//! ```

use crate::gfx::{batcher::Batcher, DrawContext, Pipeline};
use fna3d::Device;
use sdl2::{
    render::WindowCanvas,
    {event::Event, keyboard::Keycode},
};
use std::time::Duration;

/// Initial window settings
pub struct WindowConfig {
    pub title: String,
    pub w: u32,
    pub h: u32,
}

/// Window handle created from `WindowConfig`
pub struct WindowHandle {
    pub sdl: sdl2::Sdl,
    pub raw_window: *mut sdl2::sys::SDL_Window,
    pub params: fna3d::PresentationParameters,
    pub canvas: WindowCanvas,
}

/// The core
pub struct App<T: AppState> {
    dcx: DrawContext,
    clear_color: fna3d::Color,
    state: T,
    win: WindowHandle,
}

/// User data driven by `App`
pub trait AppState {
    fn render(&mut self, dcx: &mut DrawContext);
    fn update(&mut self);
}

enum UpdateResult {
    Continue,
    Quit,
}

/// Return value of `App::run`
pub type AppResult = std::result::Result<(), Box<dyn std::error::Error>>;

impl WindowConfig {
    pub fn default() -> Self {
        Self {
            title: "† ANF game †".to_string(),
            w: 1280,
            h: 720,
        }
    }

    pub fn create(&self) -> (WindowHandle, fna3d::Device) {
        log::info!("FNA version {}", fna3d::linked_version());
        let flags = fna3d::prepare_window_attributes();
        let sdl = sdl2::init().unwrap();
        let canvas = self.canvas(&sdl, flags.0);
        let win = canvas.window().raw();
        let (params, device) = self.device(win as *mut _);

        let handle = WindowHandle {
            sdl,
            raw_window: win,
            params,
            canvas,
        };

        (handle, device)
    }

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

impl<T: AppState> App<T> {
    pub fn new(state: T, win: WindowHandle, mut device: Device) -> Self {
        Self::init_gfx(&mut device, &win.params);

        let pipe = Pipeline::from_device(&mut device);
        let batcher = Batcher::from_device(&mut device);

        App {
            dcx: DrawContext {
                device,
                batcher,
                pipe,
            },
            clear_color: fna3d::Color::cornflower_blue(),
            state,
            win,
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

    pub fn run(&mut self) -> AppResult {
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

impl<T: AppState> App<T> {
    /// Does nothing for now
    fn update(&mut self) {
        self.state.update();
    }

    /// Runs the rendering pipeline
    fn render(&mut self) {
        self.clear();
        self.state.render(&mut self.dcx);
        self.dcx
            .device
            .swap_buffers(None, None, self.win.raw_window as *mut _);
    }

    fn clear(&mut self) {
        self.dcx
            .device
            .clear(fna3d::ClearOptions::TARGET, self.clear_color, 0.0, 0);
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
