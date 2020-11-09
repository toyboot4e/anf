/*! Primitive framework

[`AnfLifecycle`] is very primitive; it doesn't contain specific stages such as `debug_render`.
So you would build your own framework on top of it. One example is [`anf_samples`], where
context/user-data pattern lifecycle is run.

[`AnfLifecycle`]: crate::engine::prelude::AnfLifecycle
[`anf_samples`]: https://github.com/toyboot4e/anf_samples
!*/

pub mod core;
pub mod draw;
pub mod utils;

mod embedded;

pub mod prelude {
    //! Exports the ANF engine, dependent crates and utility macros (`log` and `anyhow`)
    pub use ::fna3d::{self, Color};

    pub use crate::engine::{
        core::{
            lifecycle::{AnfFramework, AnfLifecycle, AnfResult},
            window::{WindowConfig, WindowHandle},
        },
        draw::*,
    };
}
