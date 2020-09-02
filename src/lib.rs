//! ANF is a 2D framework powered by FNA3D
//!
//! It offers game loop, asset management  and 2D rendering API. [Examples](https://github)
//!
//! # TODOs:
//!
//! ## Infructure
//!
//! * copy FNA3D to output
//! * copy `assets/` to output
//!
//! ## API
//!
//! * hide `fna3d` (e.g. re-export `fna3d::Color` to `anf::gfx`)
//!
//! ## Features
//!
//! * viewport etc.
//! * `Texture2D` with or without lifetime
//! * content loader (cache `Teture2D`)
//! * async texture loading
//!
//! ## impls
//!
//! * free memory on neessary
//! * FPS
//!
//! ## Improve Rust-FNA3D
//!
//! * use render target bindings

pub use fna3d;
pub use sdl2;

pub mod app;
pub mod gfx;
pub mod vfs;
