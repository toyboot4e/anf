//! ANF is an FNA-like 2D framework powered by FNA3D
//!
//! WIP: It offers game loop, `TextureGen`  and `Batcher`.
//!
//! ANF is also intended to introduce FNA3D so the documentation goes into internals details.
//!
//! # TODOs:
//!
//! * TODO: free memory on neessary
//! * TODO: copy FNA3D to output
//! * TODO: copy `assets/` to output
//! * TODO: FPS
//! * TODO: `Texture2D` with or without lifetime
//! * TODO: content loader (cache `Teture2D`)

pub use fna3d;
pub use sdl2;

pub mod gfx;
pub mod vfs;

use sdl2::render::WindowCanvas;
use std::time::Duration;

// ----------------------------------------

/// Initial window settings
pub struct WindowConfig {
    pub title: String,
    pub w: u32,
    pub h: u32,
}

/// State with methods called from the game loop (`run_loop`)
pub trait State {
    fn update(&mut self);
    fn render(&mut self);
    fn handle_event(&mut self, ev: &sdl2::event::Event) -> StateUpdateResult;
}

pub enum StateUpdateResult {
    Continue,
    Quit,
}

// ----------------------------------------

/// The final notification from the application returned by `run_loop`
pub type GameResult = std::result::Result<(), Box<dyn std::error::Error>>;

/// The game loop
pub fn run_loop(state: &mut impl State, sdl: &mut sdl2::Sdl) -> GameResult {
    let mut events = sdl.event_pump().unwrap();
    log::trace!("Start ANF game loop");

    'main_loop: loop {
        // pump events
        for ev in events.poll_iter() {
            match state.handle_event(&ev) {
                StateUpdateResult::Quit => break 'main_loop,
                StateUpdateResult::Continue => {}
            }
        }

        state.update();
        state.render();

        // FIXME: timestep handling & `GameTime`
        let fps = 60;
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / fps));
    }

    Ok(())
}

impl WindowConfig {
    pub fn default() -> Self {
        Self {
            title: "† ANF †".to_string(),
            w: 1280,
            h: 720,
            // w: 640,
            // h: 360,
        }
    }

    /// Returns (sdl, canvas, params, device)
    ///
    /// `WindowCanvas` holds `SDL_Window` and drops it when it goes out of scopes. You can get raw
    /// pointer to `SDL_Window` from `WindowCanvas`.
    pub fn create(
        &self,
    ) -> (
        sdl2::Sdl,
        WindowCanvas,
        fna3d::PresentationParameters,
        fna3d::Device,
    ) {
        log::info!("FNA version {}", fna3d::linked_version());
        let flags = fna3d::prepare_window_attributes();

        let sdl = sdl2::init().unwrap();
        let canvas = self.canvas(&sdl, flags.0);
        let win = canvas.window().raw();
        let (params, device) = self.device(win as *mut _);

        (sdl, canvas, params, device)
    }
}

impl WindowConfig {
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

    pub fn device(
        &self,
        win: *mut std::ffi::c_void,
    ) -> (fna3d::PresentationParameters, fna3d::Device) {
        let params = {
            let mut params = fna3d::utils::params_from_window_handle(win);
            params.backBufferWidth = self.w as i32;
            params.backBufferHeight = self.h as i32;
            params
        };
        let device = fna3d::Device::from_params(params, true);
        (params, device)
    }
}
