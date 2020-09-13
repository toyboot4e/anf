//! Primary types to write ANF game
//!
//! # Getting started
//!
//! This is a hello-world program:
//!
//! ```no_run
//! use anf::prelude::*;
//!
//! fn main() -> AnfResult {
//!     let app = AnfApp::from_cfg(AnfAppConfig::default());
//!     let state = self::lib_rs::MyState {};
//!     anf::run_game(app, state)
//! }
//!
//! mod lib_rs {
//!     use anf::prelude::*;
//!     use fna3d::Color;
//!
//!     pub struct MyState {}
//!
//!     impl AnfGame for MyState {
//!         fn update(&mut self, ts: TimeStep) {}
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
//! [cornflower blue]: https://www.google.com/search?q=cornflower%20blue
//! [examples]: https://github.com/toyboot4e/anf/examples

pub use crate::preset::framework::{
    app::{AnfApp, AnfAppConfig},
    game::{AnfGame, AnfResult},
    time::TimeStep,
};

pub use crate::gfx::api::*;
pub use fna3d;
pub use sdl2;
