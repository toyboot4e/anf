//! Game loop
//!
//! It is rather fixed than being extensible.
//!
//! # Getting started
//!
//! This is a hello-world program:
//!
//! ```no_run
//! // main.rs or bin.rs side
//! use anf::game::{app::{App, AppConfig}, GameLoop, GameResult},
//! };
//!
//! fn main() -> GameResult {
//!     let app = App::from_cfg(AppConfig::default());
//!     let state = MyState {};
//!     GameLoop::run_app(app, state)
//! }
//!
//! // lib.rs side
//! use anf::{game::GameState, gfx::DrawContext};
//! use anf::fna3d::Color;
//!
//! struct MyState {}
//!
//! impl GameState for MyState {
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

pub mod app;
pub mod input;

use crate::{gfx::DrawContext, vfs};
use app::{App, SdlWindowHandle};
use sdl2::{event::Event, keyboard::Keycode};
use std::time::Duration;

/// User data driven by [`GameLoop`]
pub trait GameState {
    fn update(&mut self) {}
    #[allow(unused_variables)]
    fn render(&mut self, dcx: &mut DrawContext) {}
    fn listen_event(&mut self, ev: &Event) {}
}

/// Return type of [`GameLoop::run`]
pub type GameResult = std::result::Result<(), Box<dyn std::error::Error>>;

pub fn run_game<T: GameState>(app: App, state: T) -> GameResult {
    GameLoop::new(state, app).run()
}

/// Does everything after window creation
struct GameLoop<T: GameState> {
    dcx: DrawContext,
    state: T,
    win: SdlWindowHandle,
}

/// Device initialization
/// ---
impl<T: GameState> GameLoop<T> {
    fn new(state: T, mut src: App) -> Self {
        self::init_device(&mut src.device, &src.params);
        GameLoop {
            dcx: DrawContext::new(src.device, vfs::default_shader()),
            state,
            win: src.win,
        }
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

    // let dst = fna3d::DepthStencilState::default();
    // device.set_depth_stencil_state(&dst);
}

/// Internal type for the game loop
enum UpdateResult {
    Continue,
    Quit,
}

impl<T: GameState> GameLoop<T> {
    fn run(mut self) -> GameResult {
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

    fn update(&mut self) {
        self.state.update();
    }

    fn render(&mut self) {
        self.state.render(&mut self.dcx);
        self.dcx
            .device
            .swap_buffers(None, None, self.win.raw_window as *mut _);
    }

    /// Just quits on `Escape` key down for now
    fn handle_event(&mut self, ev: &Event) -> UpdateResult {
        match ev {
            Event::Quit { .. } => UpdateResult::Quit,
            ev => {
                self.state.listen_event(&ev);
                UpdateResult::Continue
            }
        }
    }
}
