//! Primitive framework
//!
//! [`AnfLifecycle`] is very primitive; it doesn't contain specific stages such as `debug_render`.
//! So you would build your own framework on top of it.
//!
//! [`anf_samples`] contains an examples framework where context/user-data pattern lifecycle is
//! run.
//!
//! [`AnfLifecycle`]: crate::engine::prelude::AnfLifecycle
//! [`anf_samples`]: https://github.com/toyboot4e/anf_samples

pub mod core;
pub mod draw;
pub mod utils;

mod embedded;

pub mod prelude {
    //! Exports the ANF engine, dependent crates and utility macros (`log` and `anyhow`)

    pub use ::{
        anyhow::{anyhow, bail, ensure, Context, Result},
        log::{debug, error, info, trace, warn},
    };

    pub use ::{
        fna3d::{self, Color},
        sdl2, soloud, xdl,
    };

    #[cfg(feature = "debug-gui")]
    pub use ::{imgui, imgui_fna3d};

    pub use crate::engine::{
        core::{
            lifecycle::{AnfFramework, AnfLifecycle, AnfResult},
            window::{WindowConfig, WindowHandle},
        },
        draw::*,
    };
}
