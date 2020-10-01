//! ANF is a 2D framework powered by [SDL] & [FNA3D] ([Rust-SDL2] and [Rust-FNA3D])
//!
//! Note that ANF is unstable and experimental; ANF does not provide solutions in most areas -- you
//! have to do it yourself.
//!
//! # Restrictions
//!
//! * One-window, single-threaded
//! * Pre-defined game loop
//!
//! # Note
//!
//! * right-handed coordinate system
//!
//! [SDL]: https://www.sdl.com/
//! [FNA3D]: https://github.com/FNA-XNA/FNA3D
//! [Rust-SDL2]: https://github.com/Rust-SDL2/rust-sdl2
//! [Rust-FNA3D]: https://github/toyboot4e/rust-fna3d
//! [anf/examples]: https://github/toyboot4e/anf/examples

pub use fna3d;
pub use sdl2;

pub mod engine;

pub mod gfx;
pub mod input;
pub mod utils;
pub mod vfs;
