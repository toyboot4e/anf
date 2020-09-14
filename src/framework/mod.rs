//! Framework, the partially built application
//!
//! # Getting started
//!
//! This is a hello-world program:
//!
//! ```no_run
//! use anf::framework::*;
//!
//! fn main() -> AnfResult {
//!     anf_run_game(&AnfConfig::default(), |_| hello::MyState {})
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
//! Your screen will be filled with [cornflower blue] pixels. Feel like you're home -- you're
//! welcome :)
//!
//! [`AnfGame`] has more lifecycle methods such as `update`.
//!
//! [cornflower blue]: https://www.google.com/search?q=cornflower%20blue
//! [examples]: https://github.com/toyboot4e/anf/examples

mod core;
mod game;
mod time;

pub use self::{
    core::AnfConfig,
    game::{AnfGame, AnfResult},
    time::TimeStep,
};

use self::{
    core::{anf_create_core, SdlWindowHandle},
    game::AnfGameLoop,
};

fn init_framework(cfg: &AnfConfig) -> (SdlWindowHandle, AnfGameLoop) {
    let (win, device, _params) = anf_create_core(&cfg);
    let looper = AnfGameLoop::new(win.raw_window(), device);
    (win, looper)
}

pub fn anf_run_game<T: AnfGame>(
    cfg: &AnfConfig,
    f: impl FnOnce(&mut fna3d::Device) -> T,
) -> AnfResult {
    let (mut win, mut looper) = init_framework(cfg);
    let mut events = win.event_pump().unwrap();
    let mut state = f(looper.as_mut());
    while looper.tick_one_frame(&mut state, &mut events) {}
    Ok(())
}
