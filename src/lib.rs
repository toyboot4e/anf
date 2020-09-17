//! ANF is an 2D framework powered by [SDL] & [FNA3D] ([Rust-SDL2] and [Rust-FNA3D])
//!
//! ANF aims privide an object-oriented APIs.
//!
//! # Restrictions
//!
//! * One-window, single-threaded
//! * Pre-defined game loop
//!
//! Also, note that ANF is unstable.
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
    //! Prelude, primary types for writing ANF games
    //!
    //! Contains external crates, draw API and lifecycle types

    pub use fna3d;
    pub use sdl2;

    pub use crate::game::{app::*, draw::*, AnfFramework};
}
