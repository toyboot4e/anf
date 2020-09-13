//! ANF is a 2D framework powered by SDL & FNA3D
//!
//! Note that ANF is **very much a work in progress**.
//!
//! # Index
//!
//! * To get started, see [`prelude`]
//! * To render sprites, see [`gfx::api`]
//! * To see more examples, see [anf/examples] (GitHub)
//!
//! [anf/examples]: https://github/toyboot4e/anf/examples

pub use sdl2;

pub mod prelude;

pub mod preset;
pub use preset::framework::game::run_game;

pub mod gfx;
pub mod input;
pub mod vfs;

// This is for `examples/`. FIXME: delete this
pub use env_logger;
