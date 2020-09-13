//! Framework, the partially built application
//!
//! # Getting started
//!
//! This is a hello-world program:
//!
//! ```no_run
//! // main.rs or bin.rs side
//! use anf::framework::{
//!     startup::{App, AppConfig},
//!     gameloop::{GameResult, run_game},
//! };
//!
//! fn main() -> GameResult {
//!     let app = App::from_cfg(AppConfig::default());
//!     let state = MyState {};
//!     run_game(app, state)
//! }
//!
//! // lib.rs side
//! use anf::{framework::gameloop::GameState, prelude::*};
//! use fna3d::Color;
//!
//! struct MyState {}
//!
//! impl GameState for MyState {
//!     fn update(&mut self, ts: TimeStep) {}
//!     fn render(&mut self, ts: TimeStep, dcx: &mut DrawContext) {
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

pub mod gameloop;
pub mod startup;
pub mod utils;
