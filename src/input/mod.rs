//! Input
//!
//! The primary ANF input system is just a state.
//!
//! # Usage
//!
//! Handling input:
//!
//! ```no_run
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
//! ```no_run
//! use anf::engine::prelude::*;
//! use anf::input::Keyboard;
//! use sdl2::event::Event;
//!
//! struct SampleState {
//!      kbd: Keyboard,
//! }
//!
//! impl AnfLifecycle for SampleState {
//!     fn event(&mut self, ev: &Event) -> AnfResult<()> {
//!         self.kbd.event(ev);
//!         Ok(())
//!     }
//!     fn on_end_frame(&mut self) -> AnfResult<()> {
//!         self.kbd.on_end_frame();
//!         Ok(())
//!     }
//! }
//! ```

pub mod vinput;

mod keyboard;
mod mouse;

pub use keyboard::{Key, Keyboard};

#[derive(Debug)]
struct Double<T> {
    /// Last
    a: T,
    /// Current
    b: T,
}

impl<T: Default> Default for Double<T> {
    fn default() -> Self {
        Self {
            a: T::default(),
            b: T::default(),
        }
    }
}
