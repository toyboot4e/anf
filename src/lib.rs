//! ANF is an 2D framework powered by [SDL] & [FNA3D] ([Rust-SDL2] and [Rust-FNA3D])
//!
//! The primary feature of ANF is object-oriented API.
//!
//! ANF should always have API that is close to the best. If you feel anything uncomfortable, feel
//! free to open issues or to send pull requests.
//!
//! # Index
//!
//! * To get started, see [`framework`]
//! * To render sprites, see [`gfx::api`]
//! * To handle input, see [`input`]
//! * For more examples, see [anf/examples] (GitHub)
//!
//! # Restrictions
//!
//! * One-window, single-threaded
//! * Pre-defined game loop
//!
//! Note that ANF is unstable.
//!
//! [SDL]: https://www.sdl.com/
//! [FNA3D]: https://github.com/FNA-XNA/FNA3D
//! [Rust-SDL2]: https://github.com/Rust-SDL2/rust-sdl2
//! [Rust-FNA3D]: https://github/toyboot4e/rust-fna3d
//! [anf/examples]: https://github/toyboot4e/anf/examples

pub use sdl2;

pub mod gfx;
pub mod input;
pub mod vfs;

pub mod framework;
pub mod prelude;
