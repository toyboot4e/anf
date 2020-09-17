//! Draw API + lifecycle + window
//!
//! Recommended variable names:
//!
//! * `ucx`: [`UpdateContext`]
//! * `dcx`: [`DrawContext`]

mod draw;
mod lifecycle;
mod time;
mod window;

pub use self::{
    draw::*,
    lifecycle::{AnfLifecycle, UpdateContext},
    time::TimeStep,
    window::{WindowConfig, WindowHandle},
};
