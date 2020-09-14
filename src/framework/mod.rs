//! Framework, the partially built application
//!
//! [`anf_run_game`] drives user data [`AnfGame`]. I hope this is generic enough for you. If not,
//! TODO: provide a way to build custom game loop without much efforts. Maybe separate crate
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
//!     impl AnfGame for MyState {
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

mod core;
mod game;
mod time;

pub use self::{core::AnfConfig, game::AnfGame, time::TimeStep};

use self::{
    core::{anf_create_core, SdlWindowHandle},
    game::AnfGameLoop,
};

/// Return type of [`anf_run_game`]
pub type AnfResult = std::result::Result<(), Box<dyn std::error::Error>>;

/// Drives user data
pub fn anf_run_game<T: AnfGame>(
    cfg: &AnfConfig,
    f: impl FnOnce(&mut fna3d::Device) -> T,
) -> AnfResult {
    // create window and game loop runner
    let (mut window, mut loop_runner) = init_framework(cfg);
    // get Rust-SDL2 event pump
    let mut events = window.event_pump().unwrap();
    // create user data giving access to `fna3d::Device`
    let mut state = f(loop_runner.as_mut());
    // run the game loop
    while loop_runner.tick_one_frame(&mut state, &mut events) {}
    Ok(())
}

/// Creates window and game loop runner
fn init_framework(cfg: &AnfConfig) -> (SdlWindowHandle, AnfGameLoop) {
    // create SDL window and fna3d Device
    let (win, device, _params) = anf_create_core(&cfg);
    let looper = AnfGameLoop::new(win.raw_window(), device);
    (win, looper)
}
