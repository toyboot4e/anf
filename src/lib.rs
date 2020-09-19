//! ANF is a 2D framework powered by [SDL] & [FNA3D] ([Rust-SDL2] and [Rust-FNA3D])
//!
//! Note that ANF is unstable. ANF does not provide solutions in most areas.
//!
//! # Restrictions
//!
//! * One-window, single-threaded
//! * Pre-defined game loop
//!
//! [SDL]: https://www.sdl.com/
//! [FNA3D]: https://github.com/FNA-XNA/FNA3D
//! [Rust-SDL2]: https://github.com/Rust-SDL2/rust-sdl2
//! [Rust-FNA3D]: https://github/toyboot4e/rust-fna3d
//! [anf/examples]: https://github/toyboot4e/anf/examples

pub use fna3d;
pub use sdl2;

pub mod game;

pub mod gfx;
pub mod input;
pub mod vfs;

pub mod prelude {
    //! Dependent crates, draw APIs and lifecycle types
    //!
    //! They are primary types for writing ANF games.

    pub use fna3d;
    pub use sdl2;

    pub use crate::game::{app::*, draw::*};
}
