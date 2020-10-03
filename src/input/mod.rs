//! Input
//!
//! These types are states and should be updated via [`AnfLifecycle`].
//!
//! [`AnfLifecycle`]: crate::engine::lifecycle::AnfLifecycle

pub mod axis;
pub mod vinput;

mod keyboard;
mod mouse;
mod repeat;

pub use keyboard::{Key, Keyboard};
pub use mouse::{Mouse, MouseInput};
pub use repeat::KeyRepeat;

/// Contains all the input states
#[derive(Debug)]
pub struct Input {
    kbd: Keyboard,
    mouse: Mouse,
}

impl Input {
    pub fn kbd(&self) -> &Keyboard {
        &self.kbd
    }

    pub fn mouse(&self) -> &Mouse {
        &self.mouse
    }
}
