//! ANF is a 2D framework powered by [SDL] & [FNA3D] ([Rust-SDL2] and [Rust-FNA3D])
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

// This is for `examples/`. FIXME: delete this
pub use env_logger;
