//! Application, the framework
//!
//! [`AnfFramework`] drives user data that implements[`AnfLifecycle`]. I hope this is generic enough
//! for you. If TODO: provide a way to build custom game loop without much efforts. Maybe separate
//! crate.
//!
//! [`AnfFramework`]: framework::AnfFramework
//! [`AnfLifecycle`]: prelude::AnfLifecycle
//!
//! # Getting started
//!
//! This is a hello-world program:
//!
//! ```no_run
//! use anf::app::{framework::*, prelude::*};
//!
//! fn main() -> AnfResult {
//!     AnfFramework::with_cfg(WindowConfig::default())
//!         .run(|win, cfg, dcx| hello::MyState {})
//! }
//!
//! mod hello {
//!     use anf::prelude::*;
//!     use fna3d::Color;
//!
//!     pub struct MyState {}
//!
//!     impl AnfLifecycle for MyState {
//!         fn render(&mut self, dcx: &mut DrawContext) {
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

pub mod framework;
pub mod prelude;
