//! Primitive framework
//!
//! Users need to write some boilerplate code to run their games.
//!
//! # Lifecycle
//!
//! [`lifecycle::AnfLifecycle`] provides a very primitive lifecycle. It doesn't contain specific
//!  stages such as `debug_render`. So it's for building your own framework lifecycle on top of it!
//!
//! See [`examples`] to get started; it contains context/user-data patten lifecycle.
//!
//! [`examples`]: https://github.com/toyboot4e/anf_samples

pub mod app;
pub mod draw;
pub mod lifecycle;
pub mod time;
pub mod utils;

pub mod prelude {
    //! Exports most of the ANF engine

    pub use fna3d;
    pub use fna3d::Color;
    pub use sdl2;

    pub use crate::engine::{app::*, draw::*, lifecycle::*, time::TimeStep};
}
