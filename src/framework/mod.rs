//! Framework, the partially built application
//!
//! [`anf_run_game`] drives user data [`AnfLifecycle`]. I hope this is generic enough for you. If
//! not, TODO: provide a way to build custom game loop without much efforts. Maybe separate crate
//!
//! # Getting started
//!
//! This is a hello-world program:
//!
//! ```no_run
//! use anf::framework::*;
//!
//! fn main() -> AnfResult {
//!     anf_run_game(&AnfConfig::default(), |_device| hello::MyState {})
//! }
//!
//! mod hello {
//!     use anf::prelude::*;
//!     use fna3d::Color;
//!
//!     pub struct MyState {}
//!
//!     impl AnfLifecycle for MyState {
//!         fn render(&mut self, ts: TimeStep, dcx: &mut DrawContext) {
//!             anf::gfx::clear_frame(dcx, Color::cornflower_blue());
//!         }
//!     }
//! }
//! ```
//!
//! Your screen will be filled with [cornflower blue] pixels. Feel like you're home, XNA developers
//! -- you're welcome :D
//!
//! [cornflower blue]: https://www.google.com/search?q=cornflower%20blue
//!
//! # Internals
//!
//! The game loop was taken from FNA.
//!
//! * TODO: use `AnfResult`
//! * TODO: access to non-fixed FPS

mod game_loop;
mod time;
mod window;

pub use self::{
    game_loop::{AnfLifecycle, UpdateContext},
    time::TimeStep,
    window::{AnfConfig, WindowHandle},
};

use crate::{framework::game_loop::AnfGameLoop, gfx::api::DrawContext, vfs};

/// Return type of [`anf_run_game`]
pub type AnfResult = std::result::Result<(), Box<dyn std::error::Error>>;

/// Drives user data
pub fn anf_run_game<T: AnfLifecycle>(
    cfg: &AnfConfig,
    user_state_constructor: impl FnOnce(&mut DrawContext) -> T,
) -> AnfResult {
    let (mut window, mut game_loop) = {
        // construct SDL window handle and FNA3D device
        let (window, device, params) = window::init(&cfg);

        let dcx = DrawContext::new(device, vfs::default_shader(), params);
        let game_loop = AnfGameLoop::new(window.raw_window(), dcx);

        (window, game_loop)
    };

    // run the game loop
    let mut state = user_state_constructor(&mut game_loop.dcx);
    let mut events = window.event_pump().unwrap();
    while game_loop.tick_one_frame(&mut state, &mut events) {}

    // it's always Ok for now
    Ok(())
}
