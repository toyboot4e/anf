//! ANF is a 2D framework powered by FNA3D
//!
//! See the [`app`] module or [examples] to get started.
//!
//! # Restrictions
//!
//! I could do more buuuut since I'm a goblin, ANF is:
//!
//! * feature-poor
//! * desktop only
//! * one window, single-threaded rendering
//!
//! Note that SDL2 and FNA3D have much more potencial.
//!
//! [my roguelike game development]: https://github/toyboot4e/rlbox
//! [`app`]: ./app/index.html
//! [examples]: https://github/toyboot4e/anf/examples

pub use fna3d;
pub use sdl2;

pub mod app;
pub mod gfx;
pub mod vfs;
