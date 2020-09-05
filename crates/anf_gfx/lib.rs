//! Internals of ANF graphics
//!
//! Bigger, handy structs over FNA3D types + 2D sprite batch. The latter is rather fixed than
//! extensible.
//!
//! The main purpose of this crate is to hide internals while enabling `cargo test` to work
//! without errors.

pub mod batcher;
pub mod buffers;
pub mod pipeline;
pub mod texture;
