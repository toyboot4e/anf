//! ANF is a 2D framework powered by [SDL] & [FNA3D] ([Rust-SDL2] and [Rust-FNA3D])
//!
//! Note that ANF is unstable and experimental; ANF does not provide solutions in most areas -- you
//! have to do it yourself.
//!
//! # Note
//!
//! * Right-handed coordinate system
//! * Row-major matrices
//!
//! # Restrictions
//!
//! * One-window, single-threaded
//! * Pre-defined game loop
//!
//! FIXME: `cargo test` fails in ANF (so using `no_run` for now)
//!
//! [SDL]: https://www.sdl.com/
//! [FNA3D]: https://github.com/FNA-XNA/FNA3D
//! [Rust-SDL2]: https://github.com/Rust-SDL2/rust-sdl2
//! [Rust-FNA3D]: https://github/toyboot4e/rust-fna3d
//! [anf/examples]: https://github/toyboot4e/anf/examples

pub mod engine;
pub mod gfx;
pub mod vfs;

pub mod prim {
    //! Primary imports
    //!
    //! It contains external crates so you can write as this:
    //!
    //! ```
    //! use {anf::prim::*, xdl::Keyboard};
    //! ```

    pub use ::{
        anyhow::{anyhow, bail, ensure, Context, Result},
        fna3d::{self, Color, Device},
        indoc::indoc,
        log::{debug, error, info, trace, warn},
        sdl2,
    };

    #[cfg(feature = "input")]
    pub use ::xdl::{self, Key};

    #[cfg(feature = "audio")]
    pub use ::soloud;

    #[cfg(feature = "debug-gui")]
    pub use ::imgui;

    #[cfg(feature = "debug-gui")]
    pub use ::imgui_fna3d;

    pub use crate::{engine::prelude::*, gfx::prelude::*, vfs};
}

pub use ::{fna3d, sdl2};

#[cfg(feature = "input")]
pub use ::xdl;

#[cfg(feature = "audio")]
pub use ::soloud;

#[cfg(feature = "debug-gui")]
pub use ::imgui;

#[cfg(feature = "debug-gui")]
pub use ::imgui_fna3d;
