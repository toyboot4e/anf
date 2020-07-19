//! ANF is an XNA-like 2D game framework in Rust powered by FNA3D

pub mod gfx;

/// The final notification from the application returned by `run_loop`
pub type GameResult = std::result::Result<(), Box<dyn std::error::Error>>;

use sdl2::{keyboard::Keycode, render::WindowCanvas};
use std::time::Duration;

// --------------------------------------------------------------------------------
// Window configuration

/// Initial window settings
pub struct WindowConfig {
    pub title: String,
    pub width: u32,
    pub height: u32,
}

/// Creates a window from `WindowConfig` and returns a handle to it
///
/// You can get raw pointer to SDL window from the returned `WindowCanvas`
pub fn create(cfg: &WindowConfig) -> (sdl2::Sdl, WindowCanvas) {
    // sdl2::hint::set("SDL_RENDER_DRIVER", "metal");
    let scx = sdl2::init().unwrap();
    let canvas = self::create_canvas(&scx, &cfg);
    (scx, canvas)
}

fn create_canvas(scx: &sdl2::Sdl, cfg: &WindowConfig) -> WindowCanvas {
    let video = scx.video().unwrap();
    let win = cfg.create_window(video);
    win.into_canvas().build().unwrap()
}

impl WindowConfig {
    pub fn default() -> Self {
        Self {
            title: "† ANF †".to_string(),
            width: 1280,
            height: 720,
        }
    }

    fn create_window(&self, video: sdl2::VideoSubsystem) -> sdl2::video::Window {
        video
            .window(&self.title, self.width, self.height)
            .position_centered()
            .build()
            .unwrap()
    }
}

// --------------------------------------------------------------------------------
// Game loop

/// State with methods called from the game loop (`run_loop`)
pub trait State {
    fn update(&mut self);
    fn render(&mut self);
    fn handle_event(&mut self, ev: &sdl2::event::Event) -> StateUpdateResult;
}

/// Way for a `State` to communicate with the game loop (`run_loop`)
pub enum StateUpdateResult {
    Continue,
    Quit,
}

/// The game loop
pub fn run_loop(state: &mut impl State, scx: &mut sdl2::Sdl) -> GameResult {
    let mut events = scx.event_pump().unwrap();
    log::trace!("Start game loop");

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

        let fps = 60;
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / fps));
    }

    Ok(())
}
