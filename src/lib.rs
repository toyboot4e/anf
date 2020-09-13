//! ANF is a 2D framework powered by SDL & FNA3D
//!
//! Note that ANF is **very much a work in progress**.
//!
//! # Index
//!
//! * To run your application, see [`framework`]
//! * To make your game content, see [`prelude`]
//!     * To render sprites, see [`gfx::api`]
//! * For more examples, see [examples]
//!
//! [examples]: https://github/toyboot4e/anf/examples

pub use fna3d;
pub use sdl2;

pub mod framework;
pub mod prelude;

pub mod gfx;
pub mod input;
pub mod vfs;

// This is for `examples/`. FIXME: delete this
pub use env_logger;
