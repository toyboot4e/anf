//! ANF is a 2D framework powered by SDL & FNA3D
//!
//! See the [`game`] or [examples] to get started. Note that ANF is **very much a work in
//! progress**.
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
//! [examples]: https://github/toyboot4e/anf/examples

pub use fna3d;
pub use fna3d_hie;
pub use sdl2;

// pub use anf_lib;
// pub use anf_gfx;

pub mod game;
pub mod gfx;
pub mod vfs;

/// for `examples/`. FIXME: delete this
pub use env_logger;
