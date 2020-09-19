//! Input
//!
//! The primary ANF input system is just a state.
//!
//! # Usage
//!
//! Handling input:
//!
//! ```
//! use anf::input::{Key, Keyboard};
//!
//! fn handle_input(kbd: &Keyboard) {
//!     if kbd.is_key_pressed(Key::Enter) {
//!         // do something
//!     }
//! }
//! ```
//!
//! Why is it not event-based? Because it's for emitting higher-level input events like UI commands,
//! and there are not many listeners to primitive events such as key down or key up.
//!
//! # Integration
//!
//! Call the lifecycle methods of input objects:
//!
//! ```
//! use anf::prelude::*;
//! use anf::input::Keyboard;
//! use sdl2::event::Event;
//!
//! struct SampleState {
//!      kbd: Keyboard,
//! }
//!
//! impl AnfAppLifecycle for SampleState {
//!     fn event(&mut self, ev: &Event) {
//!         self.kbd.listen_sdl_event(ev);
//!     }
//!     fn on_end_frame(&mut self) {
//!         self.kbd.on_end_frame();
//!     }
//! }
//! ```

mod keyboard;
pub mod vinput;
pub use keyboard::{Key, Keyboard};
